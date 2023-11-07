use iced::widget::{column, container, row, scrollable, text_input};
use iced::{Application, Command, Length, Settings, Theme};

use reqwest;
use serde::Deserialize;

fn main() -> iced::Result {
    FilmForum::run(Settings::default())
}

struct FilmForum {
    query: String,
    films: Vec<Film>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Film {
    id: String,
    title: String,
    description: String,
    is_movie: bool,
}

#[derive(Debug, Clone)]
enum ApiError {}

#[derive(Debug, Clone)]
enum Message {
    FilmsFound(Result<Vec<Film>, ApiError>),
    Search(String),
}

impl Application for FilmForum {
    type Executor = iced::executor::Default;

    type Message = Message;

    type Theme = Theme;

    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            FilmForum {
                query: String::new(),
                films: vec![],
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("FilmForum")
    }

    fn update(&mut self, message: Self::Message) -> iced::Command<Self::Message> {
        match message {
            Message::FilmsFound(Ok(films)) => {
                self.films = films;
                Command::none()
            }
            Message::FilmsFound(Err(_)) => Command::none(),
            Message::Search(query) => {
                self.query = query;
                let newq = String::clone(&self.query);
                Command::perform(Film::search(newq), Message::FilmsFound)
            }
        }
    }

    fn view(&self) -> iced::Element<Message> {
        let mut content =
            column![text_input("Search for film", &self.query).on_input(Message::Search)];

        for film in &self.films {
            let ntit = String::clone(&film.title);
            content = content.push(iced::widget::text(ntit));
        }

        scrollable(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl Film {
    async fn search(query: String) -> Result<Vec<Film>, ApiError> {
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
