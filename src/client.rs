pub mod models;

use models::film::{Film, SearchError};
use models::user::{LoginError, LoginResult};

use reqwest::header::{ACCEPT, CONTENT_TYPE};
use serde_json::json;

pub async fn search(query: String) -> Result<Vec<Film>, SearchError> {
    let url = format!("http://127.0.0.1:5105/api/film/search/{query}");
    let films_result = match reqwest::get(url).await {
        Ok(response) => match response.text().await {
            Ok(content) => {
                let films: Vec<Film> = serde_json::from_str(content.as_str()).unwrap_or(vec![]);
                films
            }
            Err(_) => vec![],
        },
        Err(_) => vec![],
    };

    Ok(films_result)
}

pub async fn login(email: String, password: String) -> Result<LoginResult, LoginError> {
    let url = "http://127.0.0.1:5105/login";
    let client = reqwest::Client::new();

    let login_json = json!({
        "email": email.as_str(),
        "password": password.as_str()
    });

    let request = client
        .post(url)
        .header(ACCEPT, "*/*")
        .header(CONTENT_TYPE, "application/json")
        .body(login_json.to_string())
        .send()
        .await;

    let response = match request {
        Ok(res) => res.text().await.unwrap_or(String::new()),
        Err(_) => String::new(),
    };

    Ok(serde_json::from_str(response.as_str()).unwrap())
}
