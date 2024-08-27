use std::path::Path;

use actix_files::Files;
use actix_web::{web::Data, App, HttpServer};
use anyhow::Result;
use once_cell::sync::Lazy;
use reqwest::Client;
use schema::AppData;
use tokio::{fs, sync::Mutex};

mod getters;
mod openlib;
mod posters;
mod schema;
mod setters;
mod tmdb;

pub const DATA_FILE: &str = "data.json";
pub const POSTERS_DIR: &str = "posters";
pub const COVERS_DIR: &str = "covers";

pub static CLIENT: Lazy<Client> = Lazy::new(Client::new);
pub static TMDB: Lazy<tmdb_api::client::ReqwestClient> = Lazy::new(|| {
    tmdb_api::client::ReqwestClient::new(
        dotenvy::var("TMDB_API_KEY")
            .expect("A TMDB API key must be set in the `TMDB_API_KEY` environment variable"),
    )
});
pub static OPENLIB: Lazy<openlibrsry::Client> = Lazy::new(openlibrsry::Client::new);

pub struct AppState(pub Mutex<AppData>);

pub async fn save_to_disk(data: &AppData) -> Result<()> {
    fs::write(DATA_FILE, serde_json::to_string_pretty(data)?).await?;
    Ok(())
}

#[actix_web::main]
async fn main() -> Result<()> {
    let saved_data = fs::read_to_string(DATA_FILE)
        .await
        .unwrap_or_else(|_| String::from("{}"));
    let data = serde_json::from_str(&saved_data)?;
    fs::create_dir_all(Path::new(POSTERS_DIR).join("small")).await?;
    fs::create_dir_all(Path::new(POSTERS_DIR).join("big")).await?;
    fs::create_dir_all(Path::new(COVERS_DIR).join("small")).await?;
    fs::create_dir_all(Path::new(COVERS_DIR).join("big")).await?;

    let state = Data::new(AppState(Mutex::new(data)));
    HttpServer::new(move || {
        App::new()
            .service(getters::get_all_movies)
            .service(getters::get_all_tags)
            .service(getters::get_all_books)
            .service(getters::get_movie)
            .service(getters::get_book)
            .service(setters::clear_cache)
            .service(setters::post_movie)
            .service(setters::patch_movie)
            .service(setters::delete_movie)
            .service(setters::post_tag)
            .service(setters::patch_tag)
            .service(setters::delete_tag)
            .service(setters::movie_put_rating)
            .service(setters::movie_patch_rating)
            .service(setters::movie_delete_rating)
            .service(setters::post_book)
            .service(setters::patch_book)
            .service(setters::delete_book)
            .service(setters::book_add_reading)
            .service(setters::book_delete_reading)
            .service(setters::book_reading_set_for_date)
            .service(setters::book_reading_set_rating)
            .service(setters::book_reading_delete_rating)
            .service(posters::get_poster_small)
            .service(posters::get_poster_big)
            .service(posters::get_cover_small)
            .service(posters::get_cover_big)
            .service(tmdb::search)
            .service(tmdb::by_id)
            .service(openlib::search)
            .service(openlib::editions)
            .service(Files::new("/", "./web/dist").index_file("index.html"))
            .app_data(state.clone())
    })
    .bind(("0.0.0.0", 19283))?
    .run()
    .await?;
    Ok(())
}
