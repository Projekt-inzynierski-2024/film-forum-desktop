pub mod film {
    use serde::Deserialize;

    #[derive(Debug, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Film {
        pub id: String,
        pub title: String,
        pub description: String,
        pub is_movie: bool,
    }
}

pub mod user {
    use serde::Deserialize;

    #[derive(Debug, Clone, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct LoginResult {
        pub id: i32,
        pub username: String,
        pub jwt: String,
    }
}
