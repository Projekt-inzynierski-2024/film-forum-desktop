pub mod models;

use models::film::Film;
use models::user::LoginResult;

use reqwest::header::{ACCEPT, CONTENT_TYPE};
use serde_json::json;

#[derive(Debug, Clone)]
pub enum SearchError {}

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

#[derive(Debug, Clone)]
pub enum LoginError {
    ConnectionError,
    CredentialsError(String),
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

    if let Err(_) = request {
        return Err(LoginError::ConnectionError);
    }

    let content = request.unwrap().text().await;

    if let Err(_) = content {
        return Err(LoginError::ConnectionError);
    }

    let result = content.unwrap();

    match serde_json::from_str(result.as_str()) {
        Ok(obj) => Ok(obj),
        Err(_) => Err(LoginError::CredentialsError(result)),
    }
}

pub async fn register(
    username: String,
    email: String,
    password: String,
    confirm_password: String,
) -> Result<LoginResult, LoginError> {
    let url = "http://127.0.0.1:5105/register";
    let client = reqwest::Client::new();

    let login_json = json!({
        "username": username.as_str(),
        "email": email.as_str(),
        "password": password.as_str(),
        "confirmPassword": confirm_password.as_str()
    });

    let request = client
        .post(url)
        .header(ACCEPT, "*/*")
        .header(CONTENT_TYPE, "application/json")
        .body(login_json.to_string())
        .send()
        .await;

    if let Err(_) = request {
        return Err(LoginError::ConnectionError);
    }

    let content = request.unwrap().text().await;

    if let Err(_) = content {
        return Err(LoginError::ConnectionError);
    }

    let result = content.unwrap();

    match serde_json::from_str(result.as_str()) {
        Ok(obj) => Ok(obj),
        Err(_) => Err(LoginError::CredentialsError(result)),
    }
}
