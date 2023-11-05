mod models;

use models::films::Film;
use reqwest::{self, Result};

pub struct ApiClient {
    url: String,
    client: reqwest::Client,
}

impl ApiClient {
    pub fn new() -> ApiClient {
        ApiClient {
            url: "http://127.0.0.1:5105/api".to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn get_films(&self) -> Result<Vec<Film>> {
        let films_url = format!("{}/{}/{}", self.url, "film", "details");
        let response = self.client.get(films_url).send().await?.text().await?;

        Ok(serde_json::from_str(response.as_str()).unwrap_or(vec![]))
    }

    pub async fn get_film(&self, id: &str) -> Result<Option<Film>> {
        let film_url = format!("{}/{}/{}/{}", self.url, "film", id, "details");
        let response = self.client.get(film_url).send().await?.text().await?;

        Ok(serde_json::from_str(response.as_str()).unwrap_or(None))
    }
}
