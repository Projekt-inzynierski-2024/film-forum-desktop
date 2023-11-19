mod client;

use iced::widget::{button, column, row, scrollable, text_input};
use iced::{theme, Application, Command, Element, Length, Settings, Theme};

use client::models::film::Film;
use client::models::user::LoginResult;

use client::{LoginError, SearchError};

fn main() -> iced::Result {
    FilmForum::run(Settings::default())
}

#[derive(Debug, Clone)]
enum PageState {
    LoginPage,
    RegisterPage,
    SearchPage,
}

#[derive(Debug, Clone, Copy)]
struct PageController {
    current: usize,
}

impl PageController {
    fn change_page(&mut self, state: PageState) {
        self.current = match state {
            PageState::LoginPage => 0,
            PageState::RegisterPage => 1,
            PageState::SearchPage => 2,
        }
    }
}

struct Pages {
    pages: Vec<Page>,
    controller: PageController,
}

impl Pages {
    fn new() -> Pages {
        Pages {
            controller: PageController { current: 0 },
            pages: vec![
                Page::Login {
                    email: String::new(),
                    password: String::new(),
                },
                Page::Register {
                    username: String::new(),
                    email: String::new(),
                    password: String::new(),
                    confirm_password: String::new(),
                },
                Page::Search {
                    query: String::new(),
                    films: vec![],
                },
            ],
        }
    }

    fn update(&mut self, msg: PageMessage) -> iced::Command<PageMessage> {
        self.pages[self.controller.current].update(msg, &mut self.controller)
    }

    fn view(&self) -> Element<PageMessage> {
        self.pages[self.controller.current].view()
    }
}

enum Page {
    Login {
        email: String,
        password: String,
    },
    Register {
        username: String,
        email: String,
        password: String,
        confirm_password: String,
    },
    Search {
        query: String,
        films: Vec<Film>,
    },
}

impl Page {
    fn update(
        &mut self,
        msg: PageMessage,
        page_controller: &mut PageController,
    ) -> iced::Command<PageMessage> {
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
            PageMessage::Login(email, password) => {
                Command::perform(client::login(email, password), PageMessage::LoggedIn)
            }
            PageMessage::Email(_email) => {
                if let Page::Login { email, .. } = self {
                    *email = _email;
                } else if let Page::Register { email, .. } = self {
                    *email = _email;
                }

                Command::none()
            }
            PageMessage::Password(_password) => {
                if let Page::Login { password, .. } = self {
                    *password = _password;
                } else if let Page::Register { password, .. } = self {
                    *password = _password;
                }

                Command::none()
            }
            PageMessage::LoggedIn(result) => {
                match result {
                    Ok(data) => page_controller.change_page(PageState::SearchPage),
                    Err(LoginError::CredentialsError(field)) => {
                        println!("{}", field);
                    }
                    Err(LoginError::ConnectionError) => {
                        println!("Connection problem...");
                    }
                };

                Command::none()
            }
            PageMessage::Username(_username) => {
                if let Page::Register { username, .. } = self {
                    *username = _username;
                }
                Command::none()
            }
            PageMessage::ConfirmPassword(_confirm_password) => {
                if let Page::Register {
                    confirm_password, ..
                } = self
                {
                    *confirm_password = _confirm_password;
                }
                Command::none()
            }
            PageMessage::Register(username, email, password, confirm_password) => Command::perform(
                client::register(username, email, password, confirm_password),
                PageMessage::Registered,
            ),
            PageMessage::Registered(result) => {
                match result {
                    Ok(data) => page_controller.change_page(PageState::SearchPage),
                    Err(LoginError::CredentialsError(field)) => {
                        println!("{}", field);
                    }
                    Err(LoginError::ConnectionError) => {
                        println!("Connection problem...");
                    }
                };

                Command::none()
            }
            PageMessage::ChangePage(page) => {
                page_controller.change_page(page);
                Command::none()
            }
        }
    }

    fn view(&self) -> Element<PageMessage> {
        match self {
            Page::Login { email, password } => {
                let mut content = column![
                    text_input("Email", &email).on_input(PageMessage::Email),
                    text_input("Password", &password)
                        .password()
                        .on_input(PageMessage::Password),
                ];

                content = content
                    .push(
                        row![
                            button("Register")
                                .style(theme::Button::Secondary)
                                .on_press(PageMessage::ChangePage(PageState::RegisterPage)),
                            button("Login")
                                .on_press(PageMessage::Login(email.clone(), password.clone()))
                        ]
                        .spacing(5),
                    )
                    .spacing(5);

                content.into()
            }
            Page::Register {
                username,
                email,
                password,
                confirm_password,
            } => {
                let mut content = column![
                    text_input("Username", &username).on_input(PageMessage::Username),
                    text_input("Email", &email).on_input(PageMessage::Email),
                    text_input("Password", &password)
                        .password()
                        .on_input(PageMessage::Password),
                    text_input("Confirm password", &confirm_password)
                        .password()
                        .on_input(PageMessage::ConfirmPassword),
                ];

                content = content
                    .push(
                        row![
                            button("Login")
                                .style(theme::Button::Secondary)
                                .on_press(PageMessage::ChangePage(PageState::LoginPage)),
                            button("Register").on_press(PageMessage::Register(
                                username.clone(),
                                email.clone(),
                                password.clone(),
                                confirm_password.clone()
                            ))
                        ]
                        .spacing(5),
                    )
                    .spacing(5);

                content.into()
            }
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
    Login(String, String),
    LoggedIn(Result<LoginResult, LoginError>),
    Username(String),
    Email(String),
    Password(String),
    ConfirmPassword(String),
    Register(String, String, String, String),
    Registered(Result<LoginResult, LoginError>),
    ChangePage(PageState),
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

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
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
