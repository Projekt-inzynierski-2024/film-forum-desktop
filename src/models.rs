pub mod film {
    use serde::Deserialize;

    #[derive(Debug, Clone)]
    pub enum SearchError {}

    #[derive(Debug, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Film {
        pub id: String,
        pub title: String,
        pub description: String,
        pub is_movie: bool,
    }

    impl Film {
        pub async fn search(query: String) -> Result<Vec<Film>, SearchError> {
            // TODO: error handling

            let url = format!("http://127.0.0.1:5105/api/film/search/{query}");
            let films_result = match reqwest::get(url).await {
                Ok(response) => match response.text().await {
                    Ok(content) => {
                        let films: Vec<Film> =
                            serde_json::from_str(content.as_str()).unwrap_or(vec![]);
                        films
                    }
                    Err(_) => vec![],
                },
                Err(_) => vec![],
            };

            Ok(films_result)
        }
    }
}

pub mod user {
    use reqwest::header::{ACCEPT, CONTENT_TYPE};
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone)]
    pub enum RegisterError {
        EmailError(String),
        UsernameError(String),
        PasswordError(String),
    }

    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct RegisterResult {
        pub id: i32,
        pub username: String,
        pub email: String,
        pub jwt: String,
    }

    #[derive(Debug, Clone)]
    pub enum LoginError {}

    #[derive(Debug, Clone, Deserialize, Serialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LoginResult {
        pub id: i32,
        pub username: String,
        pub jwt_token: String,
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

        pub fn empty() -> User {
            User {
                username: String::new(),
                email: String::new(),
                password: String::new(),
                confirm_password: String::new(),
            }
        }

        pub async fn login(&self) -> Result<LoginResult, LoginError> {
            let url = "http://127.0.0.1:5105/login";
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

            Ok(serde_json::from_str(response.as_str()).unwrap())
        }

        pub async fn register(&self) -> Result<RegisterResult, RegisterError> {
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
                "Email already exists" => Err(RegisterError::EmailError(response)),
                "Username already exists" => Err(RegisterError::UsernameError(response)),
                json => Ok(serde_json::from_str(json).unwrap()),
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::{LoginResult, User};

        #[tokio::test]
        async fn test_register() {
            let user = User::new_user("username", "username@mail.pl", "zaq1@WSX", "zaq1@WSX");

            let _ = user.register().await;

            let login = user.login().await;

            match login {
                Ok(res) => println!("TOKEN: {}", res.jwt_token),
                Err(_) => {}
            }

            // TODO: do actual test
        }
    }
}
