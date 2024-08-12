use std::{
    borrow::Cow,
    collections::{hash_map::Entry, BTreeSet},
    time::Duration,
};

use actix_web::{
    get,
    web::{Data, Query},
    HttpResponse, Responder,
};
use itertools::Itertools;
use tmdb_api::{
    movie::{details::MovieDetails, search::MovieSearch, watch_providers::MovieWatchProviders},
    prelude::Command,
};

use crate::{
    schema::{Movie, MovieStub, Platform},
    AppState, TMDB,
};

#[derive(serde::Deserialize)]
struct SearchQuery {
    title: String,
}

fn err_to_string(error: tmdb_api::error::Error) -> String {
    use tmdb_api::error::Error;
    match error {
        Error::Request { source } | Error::Response { source } => source.to_string(),
        Error::Validation(err) => format!("server validation error: {}", err.errors.join(", ")),
        Error::Server { code, content } => format!(
            "server error {code}: {} {}",
            content.status_code, content.status_message,
        ),
    }
}

#[get("/api/tmdb/search")]
async fn search(Query(SearchQuery { title }): Query<SearchQuery>) -> impl Responder {
    if title.trim_end().is_empty() {
        return HttpResponse::Ok().json(Vec::<MovieStub>::new());
    }

    let search_results = match MovieSearch::new(title).execute(&TMDB).await {
        Ok(results) => results,
        Err(err) => return HttpResponse::ServiceUnavailable().body(err_to_string(err)),
    };
    let stubs = search_results.results.into_iter().map(|result| MovieStub {
        tmdb_id: result.inner.id,
        title: result.inner.title,
        description: result.inner.overview,
        release_date: result.inner.release_date.unwrap_or_default(),
        poster: result.inner.poster_path,
    });
    HttpResponse::Ok().json(stubs.collect_vec())
}

#[derive(serde::Deserialize)]
struct ByIdQuery {
    id: String,
}

struct FindByImdbId(u32);
impl Command for FindByImdbId {
    type Output = FindResult;

    fn path(&self) -> Cow<'static, str> {
        format!("/find/tt{:07}", self.0).into()
    }

    fn params(&self) -> Vec<(&'static str, Cow<'_, str>)> {
        vec![("external_source", "imdb_id".into())]
    }
}

#[derive(serde::Deserialize)]
struct FindResult {
    movie_results: Vec<FindMovie>,
}

#[derive(serde::Deserialize)]
struct FindMovie {
    id: u64,
}

#[get("/api/tmdb/by_id")]
async fn by_id(data: Data<AppState>, Query(ByIdQuery { id }): Query<ByIdQuery>) -> impl Responder {
    let mut data_lock = data.0.lock().await;
    let movie = match data_lock.tmdb_cache.entry(id.clone()) {
        Entry::Vacant(entry) => {
            let tmdb_id = match id.strip_prefix("tt") {
                Some(imdb_id) => match imdb_id.parse::<u32>() {
                    Ok(id) => match FindByImdbId(id).execute(&TMDB).await {
                        Ok(result) => match result.movie_results.first() {
                            Some(movie) => movie.id,
                            None => {
                                return HttpResponse::NotFound()
                                    .body(format!("no movie with IMDb ID tt{id:07} found"))
                            }
                        },
                        Err(err) => {
                            return HttpResponse::ServiceUnavailable().body(err_to_string(err))
                        }
                    },
                    Err(_) => {
                        return HttpResponse::BadRequest()
                            .body(format!("malformed ID 'tt{imdb_id}'"))
                    }
                },
                None => match id.parse::<u64>() {
                    Ok(id) => id,
                    Err(_) => {
                        return HttpResponse::BadRequest().body(format!("malformed ID '{id}'"))
                    }
                },
            };

            // powered by JustWatch
            let platforms = match MovieWatchProviders::new(tmdb_id).execute(&TMDB).await {
                Ok(providers) => {
                    let mut platforms = BTreeSet::new();
                    if let Some(de) = providers.results.get("DE") {
                        for provider in &de.flatrate {
                            // Disney Plus: 337
                            // Netflix: 8
                            // Amazon Prime Video: 119
                            if provider.provider_id == 337 {
                                platforms.insert(Platform::DisneyPlus);
                            } else if provider.provider_id == 8 {
                                platforms.insert(Platform::Netflix);
                            } else if provider.provider_id == 119 {
                                platforms.insert(Platform::PrimeVideo);
                            }
                        }
                    }
                    platforms
                }
                Err(err) => return HttpResponse::ServiceUnavailable().body(err_to_string(err)),
            };

            let tmdb_movie = match MovieDetails::new(tmdb_id).execute(&TMDB).await {
                Ok(movie) => movie,
                Err(err) => return HttpResponse::ServiceUnavailable().body(err_to_string(err)),
            };
            let movie = Movie {
                // TODO: don't unwrap
                imdb_id: tmdb_movie.imdb_id.map(|id| {
                    id.trim_start_matches('t')
                        .parse()
                        .unwrap_or_else(|_| panic!("invalid IMDb ID '{id}'"))
                }),
                tmdb_id: tmdb_movie.inner.id,
                title: tmdb_movie.inner.title,
                description: tmdb_movie.inner.overview,
                ratings: vec![],
                tags: BTreeSet::new(),
                platforms,
                poster: tmdb_movie.inner.poster_path,
                release_date: tmdb_movie.inner.release_date.unwrap_or_default(),
                runtime: Duration::from_secs(tmdb_movie.runtime.unwrap_or(0) * 60),
                score: tmdb_movie.inner.vote_average,
            };
            entry.insert(movie)
        }
        Entry::Occupied(entry) => entry.into_mut(),
    };
    let movie = movie.clone();
    match crate::save_to_disk(&data_lock).await {
        Ok(()) => HttpResponse::Ok().json(movie),
        Err(err) => HttpResponse::InternalServerError()
            .body(format!("Failed to save new data to disk: {err}")),
    }
}
