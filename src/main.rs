mod client;

use iced::widget::{button, column, scrollable, text_input};
use iced::{Application, Command, Element, Length, Settings, Theme};

use client::models::film::{Film, SearchError};
use client::models::user::{LoginError, LoginResult};

fn main() -> iced::Result {
    FilmForum::run(Settings::default())
}

#[derive(Debug, Clone)]
struct LoginData {
    email: String,
    password: String,
}

struct Pages {
    pages: Vec<Page>,
    current: usize,
}

impl Pages {
    fn new() -> Pages {
        Pages {
            current: 0,
            pages: vec![
                Page::Login {
                    email: String::new(),
                    password: String::new(),
                },
                Page::Search {
                    query: String::new(),
                    films: vec![],
                },
            ],
        }
    }

    fn update(&mut self, msg: PageMessage) -> iced::Command<PageMessage> {
        self.pages[self.current].update(msg)
    }

    fn view(&self) -> Element<PageMessage> {
        self.pages[self.current].view()
    }
}

enum Page {
    Login { email: String, password: String },
    Search { query: String, films: Vec<Film> },
}

impl Page {
    fn update(&mut self, msg: PageMessage) -> iced::Command<PageMessage> {
        match msg {
            PageMessage::SearchFound(Ok(result)) => {
                if let Page::Search { films, .. } = self {
                    *films = result;
                }
                Command::none()
            }
            PageMessage::SearchFound(Err(_)) => Command::none(),
            PageMessage::Search(text_query) => {
                if let Page::Search { query, .. } = self {
                    *query = text_query.clone();
                }

                Command::perform(client::search(text_query), PageMessage::SearchFound)
            }
            PageMessage::Login(data) => Command::perform(
                client::login(data.email, data.password),
                PageMessage::LoggedIn,
            ),
            PageMessage::Email(_email) => {
                if let Page::Login { email, .. } = self {
                    *email = _email;
                }

                Command::none()
            }
            PageMessage::Password(_password) => {
                if let Page::Login { password, .. } = self {
                    *password = _password;
                }

                Command::none()
            }
            PageMessage::LoggedIn(result) => {
                println!("TOKEN: {}", result.unwrap().jwt_token);
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<PageMessage> {
        match self {
            Page::Login { email, password } => column![
                text_input("Email", &email).on_input(PageMessage::Email),
                text_input("Password", &password)
                    .password()
                    .on_input(PageMessage::Password),
                button("Login").on_press(PageMessage::Login(LoginData {
                    email: email.clone(),
                    password: password.clone()
                }))
            ]
            .into(),
            Page::Search { query, films } => {
                let mut content =
                    column![text_input("Search for film", query).on_input(PageMessage::Search)];

                for film in films {
                    content = content.push(iced::widget::text(&film.title.clone()));
                }

                content.into()
            }
        }
    }
}

#[derive(Debug, Clone)]
enum PageMessage {
    SearchFound(Result<Vec<Film>, SearchError>),
    Search(String),
    Login(LoginData),
    LoggedIn(Result<LoginResult, LoginError>),
    Email(String),
    Password(String),
}

struct FilmForum {
    pages: Pages,
}

#[derive(Debug, Clone)]
enum Message {
    PageMessage(PageMessage),
}

impl Application for FilmForum {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            FilmForum {
                pages: Pages::new(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("FilmForum")
    }

    fn update(&mut self, message: Self::Message) -> Command<Message> {
        match message {
            Message::PageMessage(msg) => self.pages.update(msg).map(Message::PageMessage),
        }
    }

    fn view(&self) -> Element<Message> {
        let content = self.pages.view().map(Message::PageMessage);

        scrollable(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
