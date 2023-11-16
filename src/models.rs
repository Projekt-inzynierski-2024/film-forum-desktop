use reqwest::header::{ACCEPT, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum ApiError {
    EmailExists,
    UsernameExists,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Film {
    pub id: String,
    pub title: String,
    pub description: String,
    pub is_movie: bool,
}

impl Film {
    pub async fn search(query: String) -> Result<Vec<Film>, ApiError> {
        // TODO: error handling

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
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResult {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub jwt: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret_key: Option<String>,
}

impl User {
    pub fn new_user(username: &str, email: &str, password: &str, confirm_password: &str) -> User {
        User {
            username: String::from(username),
            email: String::from(email),
            password: String::from(password),
            confirm_password: String::from(confirm_password),
            secret_key: None,
        }
    }

    pub fn new_admin(
        username: &str,
        email: &str,
        password: &str,
        confirm_password: &str,
        secret_key: &str,
    ) -> User {
        User {
            secret_key: Some(String::from(secret_key)),
            ..User::new_user(username, email, password, confirm_password)
        }
    }

    pub async fn register(self) -> Result<UserResult, ApiError> {
        let url = "http://127.0.0.1:5105/register";
        let client = reqwest::Client::new();

        let request = client
            .post(url)
            .header(ACCEPT, "*/*")
            .header(CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&self).unwrap())
            .send()
            .await;

        let response = match request {
            Ok(res) => res.text().await.unwrap_or(String::new()),
            Err(_) => String::new(),
        };

        match response.as_str() {
            "Email already exists" => Err(ApiError::EmailExists),
            "Username already exists" => Err(ApiError::UsernameExists),
            json => Ok(serde_json::from_str(json).unwrap()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ApiError, User};

    #[tokio::test]
    async fn test_register() {
        let user = User::new_user("username", "username@mail.pl", "zaq1@WSX", "zaq1@WSX");

        let register = user.register().await;

        match register {
            Ok(result) => println!("JWT: {}", result.jwt),
            Err(ApiError::EmailExists) => println!("Email is taken"),
            Err(ApiError::UsernameExists) => println!("Username is taken"),
        }

        // TODO: do actual test
    }
}
