use std::{
    io::{self, ErrorKind},
    path::Path,
};

use actix_files::NamedFile;
use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{
    get, post,
    web::{self, Query},
    Either, HttpResponse, Responder,
};
use openlibrsry::OlId;
use tokio::fs::{self, File};

use crate::{CLIENT, COVERS_DIR, POSTERS_DIR};

async fn get_image(
    base_path: &str,
    kind: &str,
    name: &str,
    dir: &str,
    download_url: impl FnOnce() -> Result<String, HttpResponse>,
) -> Either<HttpResponse, io::Result<NamedFile>> {
    let path = Path::new(base_path).join(dir).join(name);
    match File::open(&path).await {
        Ok(file) => {
            println!("using saved {kind}");
            Either::Right(NamedFile::from_file(file.into_std().await, path))
        }
        Err(err) if err.kind() == ErrorKind::NotFound => match download_url() {
            Ok(url) => {
                println!("downloading {kind}");
                let bytes = match match CLIENT.get(url).send().await {
                    Ok(resp) => resp,
                    Err(err) => {
                        return Either::Left(
                            HttpResponse::ServiceUnavailable()
                                .body(format!("could not download {kind}: {err}")),
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
                                .body(format!("could not download {kind}: {err}")),
                        )
                    }
                };
                if let Err(err) = fs::write(&path, &bytes).await {
                    return Either::Left(
                        HttpResponse::ServiceUnavailable()
                            .body(format!("could not save {kind} to file: {err}")),
                    );
                }
                Either::Right(NamedFile::open_async(path).await)
            }
            Err(resp) => Either::Left(resp),
        },
        Err(err) => Either::Left(
            HttpResponse::InternalServerError()
                .body(format!("Failed to read {kind} image file: {err}")),
        ),
    }
}

async fn get_poster(name: web::Path<String>, dir: &str, size: &str) -> impl Responder {
    let name = name.into_inner();
    get_image(POSTERS_DIR, "poster", &name, dir, || {
        Ok(format!("https://image.tmdb.org/t/p/{size}/{name}"))
    })
    .await
}

#[get("/api/posters/small/{name}")]
async fn get_poster_small(name: web::Path<String>) -> impl Responder {
    get_poster(name, "small", "w92").await
}

#[get("/api/posters/big/{name}")]
async fn get_poster_big(name: web::Path<String>) -> impl Responder {
    get_poster(name, "big", "w185").await
}

async fn get_cover(
    id: web::Path<u64>,
    dir: &str,
    size: &str,
    olid: Option<OlId>,
) -> impl Responder {
    let id = id.into_inner();
    get_image(COVERS_DIR, "cover", &id.to_string(), dir, || {
        olid.map(|olid| {
            format!("https://covers.openlibrary.org/b/olid/{olid}-{size}.jpg?default=false")
        })
        .ok_or_else(|| {
            HttpResponse::ServiceUnavailable()
                .body("cannot auto-download cover without OpenLibrary ID")
        })
    })
    .await
}

#[derive(serde::Deserialize)]
struct GetCoverQuery {
    olid: Option<OlId>,
}

#[get("/api/covers/small/{id}")]
async fn get_cover_small(
    id: web::Path<u64>,
    Query(GetCoverQuery { olid }): Query<GetCoverQuery>,
) -> impl Responder {
    get_cover(id, "small", "S", olid).await
}

#[get("/api/covers/big/{id}")]
async fn get_cover_big(
    id: web::Path<u64>,
    Query(GetCoverQuery { olid }): Query<GetCoverQuery>,
) -> impl Responder {
    get_cover(id, "big", "M", olid).await
}

#[derive(MultipartForm)]
struct FileUpload {
    #[multipart(limit = "100KB")]
    file: TempFile,
}

#[post("/api/covers/big/{id}")]
async fn upload_cover_big(
    id: web::Path<u64>,
    MultipartForm(FileUpload { file }): MultipartForm<FileUpload>,
) -> impl Responder {
    if let Err(err) = fs::copy(
        file.file.path(),
        Path::new(COVERS_DIR).join("big").join(id.to_string()),
    )
    .await
    {
        return HttpResponse::ServiceUnavailable()
            .body(format!("could not save cover to file: {err}"));
    };
    HttpResponse::Ok().finish()
}
