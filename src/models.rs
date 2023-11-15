use reqwest::header::{ACCEPT, CONTENT_TYPE};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone)]
pub enum ApiError {}

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
pub struct User {
    pub username: String,
    pub email: String,
    pub password: String,
    pub confirm_password: String,
}

impl User {
    pub fn new(username: &str, email: &str, password: &str, confirm_password: &str) -> User {
        User {
            username: String::from(username),
            email: String::from(email),
            password: String::from(password),
            confirm_password: String::from(confirm_password),
        }
    }

    pub async fn register(self) {
        let url = "http://127.0.0.1:5105/register";
        let client = reqwest::Client::new();

        let _ = client
            .post(url)
            .header(ACCEPT, "*/*")
            .header(CONTENT_TYPE, "application/json")
            .body(serde_json::to_string(&self).unwrap())
            .send()
            .await;

        // TODO: register and return token
    }
}

#[cfg(test)]
mod tests {
    use super::User;

    #[tokio::test]
    async fn test_register() {
        let user = User::new("testowy5", "tsasest@mail.pl", "zaq1@WSX", "zaq1@WSX");

        user.register().await;

        // TODO: do actual test
    }
}
