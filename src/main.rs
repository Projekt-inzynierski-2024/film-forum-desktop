mod models;

use iced::widget::{column, scrollable, text_input};
use iced::{Application, Command, Length, Settings, Theme};
use models::{ApiError, Film};

fn main() -> iced::Result {
    FilmForum::run(Settings::default())
}

struct FilmForum {
    query: String,
    films: Vec<Film>,
}

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
