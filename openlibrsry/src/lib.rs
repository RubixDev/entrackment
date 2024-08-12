use reqwest::Url;
use serde::de::DeserializeOwned;

mod olid;
pub mod requests;

pub use olid::{Key, OlId, OlIdKind};

const HOST: &str = "https://openlibrary.org";

#[derive(Default)]
pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn execute<R: OpenLibRequest>(&self, req: R) -> anyhow::Result<R::Result> {
        let url = Url::parse_with_params(&format!("{HOST}{}", req.path()), req.query())?;
        let response = self.client.get(url).send().await?;
        Ok(response.json().await?)
    }
}

pub trait OpenLibRequest {
    type Result: DeserializeOwned;
    fn path(&self) -> String;
    fn query(&self) -> Vec<(&'static str, String)>;
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn search_erebos() {
        let client = Client::new();
        let res = client
            .execute(
                requests::search::SearchBuilder::default()
                    .query("erebos")
                    .build()
                    .unwrap(),
            )
            .await;
        let _ = dbg!(&res);
        let work_id = res.unwrap().docs.first().unwrap().key.id;
        let editions = client
            .execute(
                requests::works::WorksEditionsBuilder::default()
                    .id(work_id)
                    .build()
                    .unwrap(),
            )
            .await;
        let _ = dbg!(&editions);
    }
}
