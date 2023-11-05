pub mod films {
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Film {
        pub id: String,
        pub title: String,
        pub description: String,
        pub is_movie: bool,
        pub episodes: Vec<Episode>,
    }

    #[derive(Debug, Deserialize)]
    #[serde(rename_all = "camelCase")]
    pub struct Episode {
        pub id: String,
        pub title: String,
        pub description: String,
        pub episode_number: i32,
        pub season_number: i32,
        pub length: i32,
        pub year: i32,
    }
}

pub mod users {}
