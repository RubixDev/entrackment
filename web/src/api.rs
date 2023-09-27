use std::{fmt::Display, future::Future};

use dioxus::prelude::*;
use reqwest::{RequestBuilder, Response};
use serde::de::DeserializeOwned;

pub fn use_api_fetch<T: DeserializeOwned>(
    cx: &ScopeState,
    dependencies: impl UseFutureDep,
    path: impl Display + 'static,
) -> &UseFuture<ApiResult<T>> {
    use_future(cx, dependencies, |_| async move { fetch(path).await })
}

pub type ApiResult<T> = Result<T, ApiError>;

#[derive(Debug, thiserror::Error)]
pub enum ApiError {
    #[error("request failed: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("API responded with error: {0}: {1}")]
    ErrResponse(#[source] reqwest::Error, String),
}

pub async fn fetch<T: DeserializeOwned>(path: impl Display) -> ApiResult<T> {
    send_request(
        crate::CLIENT.get(format!("{0}/api{path}", &*crate::BASE_URL)),
        Response::json::<T>,
    )
    .await
}

pub async fn send_request<T, E, F>(request: RequestBuilder, extractor: E) -> ApiResult<T>
where
    E: FnOnce(Response) -> F,
    F: Future<Output = reqwest::Result<T>>,
{
    let res = request.send().await?;
    match res.error_for_status_ref() {
        Ok(_) => Ok(extractor(res).await?),
        Err(err) => Err(ApiError::ErrResponse(err, res.text().await?)),
    }
}

pub async fn no_body(_: Response) -> reqwest::Result<()> {
    Ok(())
}
