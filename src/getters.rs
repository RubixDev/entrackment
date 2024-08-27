use actix_web::{
    get,
    web::{Data, Path},
    HttpResponse, Responder,
};

use crate::AppState;

#[get("/api/movie")]
async fn get_all_movies(data: Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(&data.0.lock().await.movies)
}

#[get("/api/tag")]
async fn get_all_tags(data: Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(&data.0.lock().await.tags)
}

#[get("/api/book")]
async fn get_all_books(data: Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(&data.0.lock().await.books)
}

#[get("/api/movie/{id}")]
async fn get_movie(data: Data<AppState>, id: Path<u64>) -> impl Responder {
    match data.0.lock().await.movies.get(&id) {
        Some(movie) => HttpResponse::Ok().json(movie),
        None => HttpResponse::NotFound().finish(),
    }
}

#[get("/api/book/{id}")]
async fn get_book(data: Data<AppState>, id: Path<u64>) -> impl Responder {
    match data.0.lock().await.books.get(&id) {
        Some(book) => HttpResponse::Ok().json(book),
        None => HttpResponse::NotFound().finish(),
    }
}
