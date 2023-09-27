use std::collections::{btree_map::Entry, hash_map};

use actix_web::{
    patch, post, put,
    web::{Data, Json, Path},
    HttpResponse, Responder,
};
use schema::{Movie, Rating, Tag};

use crate::AppState;

#[post("/api/movie")]
async fn post_movie(data: Data<AppState>, Json(movie): Json<Movie>) -> impl Responder {
    let mut data_lock = data.0.lock().await;
    match data_lock.movies.entry(movie.tmdb_id) {
        Entry::Vacant(entry) => {
            entry.insert(movie);
        }
        Entry::Occupied(_) => {
            return HttpResponse::Conflict().body("a movie with that ID is already present")
        }
    }
    match crate::save_to_disk(&data_lock).await {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Failed to save new data to disk"),
    }
}

#[patch("/api/movie")]
async fn patch_movie(data: Data<AppState>, Json(movie): Json<Movie>) -> impl Responder {
    let mut data_lock = data.0.lock().await;
    match data_lock.movies.entry(movie.tmdb_id) {
        Entry::Vacant(_) => {
            return HttpResponse::NotFound()
                .body(format!("movie with ID {} does not exist", movie.tmdb_id))
        }
        Entry::Occupied(mut entry) => {
            *entry.get_mut() = movie;
        }
    }
    match crate::save_to_disk(&data_lock).await {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Failed to save new data to disk"),
    }
}

#[post("/api/tag")]
async fn post_tag(data: Data<AppState>, Json(tag): Json<Tag>) -> impl Responder {
    let mut data_lock = data.0.lock().await;
    let tag_id = loop {
        let new_id = rand::random::<u32>();
        if !data_lock.tags.contains_key(&new_id) {
            break new_id;
        }
    };
    let new_tag = Tag { id: tag_id, ..tag };
    let resp = HttpResponse::Ok().json(&new_tag);
    data_lock.tags.insert(tag_id, new_tag);
    match crate::save_to_disk(&data_lock).await {
        Ok(()) => resp,
        Err(_) => HttpResponse::InternalServerError().body("Failed to save new data to disk"),
    }
}

#[patch("/api/tag")]
async fn patch_tag(data: Data<AppState>, Json(tag): Json<Tag>) -> impl Responder {
    let mut data_lock = data.0.lock().await;
    match data_lock.tags.entry(tag.id) {
        hash_map::Entry::Vacant(_) => {
            return HttpResponse::NotFound().body(format!("tag with ID {} does not exist", tag.id))
        }
        hash_map::Entry::Occupied(mut entry) => {
            *entry.get_mut() = tag;
        }
    }
    match crate::save_to_disk(&data_lock).await {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Failed to save new data to disk"),
    }
}

#[put("/api/movie/{id}/rating")]
async fn movie_put_rating(
    data: Data<AppState>,
    id: Path<u64>,
    Json(rating): Json<Rating>,
) -> impl Responder {
    let mut data_lock = data.0.lock().await;
    match data_lock.movies.get_mut(&id) {
        Some(movie) => {
            let insert_index = match movie.ratings.binary_search_by_key(&rating.date, |w| w.date) {
                Ok(_) => {
                    return HttpResponse::Conflict().body(format!(
                        "movie with ID {id} already has a rating set for {}",
                        rating.date.format("%Y-%m-%d")
                    ))
                }
                Err(index) => index,
            };
            movie.ratings.insert(insert_index, rating);
        }
        None => return HttpResponse::NotFound().body(format!("movie with ID {id} does not exist")),
    }
    match crate::save_to_disk(&data_lock).await {
        Ok(()) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().body("Failed to save new data to disk"),
    }
}
