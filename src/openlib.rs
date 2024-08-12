use actix_web::{get, web::Query, HttpResponse, Responder};
use openlibrsry::{
    requests::{
        search::{Document, SearchBuilder},
        works::WorksEditionsBuilder,
    },
    OlId,
};

use crate::OPENLIB;

#[derive(serde::Deserialize)]
struct SearchQuery {
    title: String,
}

#[get("/api/openlib/search")]
async fn search(Query(SearchQuery { title }): Query<SearchQuery>) -> impl Responder {
    if title.trim_end().is_empty() {
        return HttpResponse::Ok().json(Vec::<Document>::new());
    }

    let search_results = match OPENLIB
        .execute(
            SearchBuilder::default()
                .query(title)
                .build()
                .expect("building Search should never fail"),
        )
        .await
    {
        Ok(results) => results,
        Err(err) => return HttpResponse::ServiceUnavailable().body(err.to_string()),
    };
    HttpResponse::Ok().json(search_results.docs)
}

#[derive(serde::Deserialize)]
struct EditionsQuery {
    work: OlId,
}

#[get("/api/openlib/editions")]
async fn editions(Query(EditionsQuery { work }): Query<EditionsQuery>) -> impl Responder {
    let editions = match OPENLIB
        .execute(
            WorksEditionsBuilder::default()
                .id(work)
                .build()
                .expect("required field `id` is present"),
        )
        .await
    {
        Ok(results) => results,
        Err(err) => return HttpResponse::ServiceUnavailable().body(err.to_string()),
    };
    HttpResponse::Ok().json(editions.entries)
}
