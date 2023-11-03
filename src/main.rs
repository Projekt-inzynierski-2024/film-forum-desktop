use reqwest::{self, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Film {
    id: String,
    title: String,
    description: String,
    is_movie: bool,
    episodes: Vec<Episode>,
}

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
struct Episode {
    id: String,
    title: String,
    description: String,
    episode_number: i32,
    season_number: i32,
    length: i32,
    year: i32,
}

#[tokio::main]
async fn main() -> Result<()> {
    let url = "http://localhost:5105/api";
    let films_url = format!("{}/{}", url, "film/details");

    let client = reqwest::Client::new();
    let res = client.get(films_url).send().await?.text().await?;

    let films: Vec<Film> = serde_json::from_str(res.as_str()).expect("Can't parse this");

    for film in films {
        println!("Film title: {0}", film.title);
        println!("Film description: {0}", film.description);

        if film.is_movie {
            match film.episodes.first() {
                Some(e) => {
                    println!("Film length: {0}", e.length);
                    println!("Film year: {0}", e.year);
                }
                None => {}
            }
        } else {
            for episode in film.episodes {
                println!("Episode title: {0}", episode.title);
                println!("Episode description: {0}", episode.description);
                println!(
                    "Episode: {0}x{1}",
                    episode.season_number, episode.episode_number
                );
                println!("Episode length: {0}", episode.length);
                println!("Episode year: {0}", episode.year);
            }
        }
    }

    Ok(())
}
