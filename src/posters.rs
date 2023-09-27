use std::{
    io::{self, ErrorKind},
    path::Path,
};

use actix_files::NamedFile;
use actix_web::{get, web, Either, HttpResponse};
use tokio::fs::{self, File};

use crate::{CLIENT, POSTERS_DIR};

async fn get_poster(name: web::Path<String>, dir: &str, size: &str) -> Either<HttpResponse, io::Result<NamedFile>> {
    let name = name.into_inner();
    let path = Path::new(POSTERS_DIR).join(dir).join(&name);
    match File::open(&path).await {
        Ok(file) => {
            println!("using saved poster");
            Either::Right(NamedFile::from_file(file.into_std().await, path))
        }
        Err(err) if err.kind() == ErrorKind::NotFound => {
            println!("downloading poster");
            let bytes = match match CLIENT
                .get(format!("https://image.tmdb.org/t/p/{size}/{name}"))
                .send()
                .await
            {
                Ok(resp) => resp,
                Err(err) => {
                    return Either::Left(
                        HttpResponse::ServiceUnavailable()
                            .body(format!("could not download poster: {err}")),
                    )
                }
            }
            .bytes()
            .await
            {
                Ok(bytes) => bytes,
                Err(err) => {
                    return Either::Left(
                        HttpResponse::ServiceUnavailable()
                            .body(format!("could not download poster: {err}")),
                    )
                }
            };
            if let Err(err) = fs::write(&path, &bytes).await {
                return Either::Left(
                    HttpResponse::ServiceUnavailable()
                        .body(format!("could not save poster to file: {err}")),
                );
            }
            Either::Right(NamedFile::open_async(path).await)
        }
        Err(err) => Either::Left(
            HttpResponse::InternalServerError()
                .body(format!("Failed to read poster image file: {err}")),
        ),
    }
}

#[get("/api/posters/small/{name}")]
async fn get_poster_small(name: web::Path<String>) -> Either<HttpResponse, io::Result<NamedFile>> {
    get_poster(name, "small", "w92").await
}

#[get("/api/posters/big/{name}")]
async fn get_poster_big(name: web::Path<String>) -> Either<HttpResponse, io::Result<NamedFile>> {
    get_poster(name, "big", "w185").await
}
