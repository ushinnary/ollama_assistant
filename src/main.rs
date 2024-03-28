use iced::keyboard::key::Named;
use iced::widget::container::StyleSheet;
use iced::widget::{
    column, container, horizontal_rule, svg, text, text_input, vertical_space, Scrollable,
};
use iced::window::Position;
use iced::{
    application, executor, keyboard, window, Application, Command, Element, Length, Settings, Size,
    Subscription,
};

use ai::ask_ai;
use styles::{get_palette_for_main_window, CustomTheme, PADDING_SIZE};
use ui::gui::{search_bar, top_bar};
use ui::RouterView;

mod ai;
mod config;
mod macros;
mod styles;
mod ui;

pub fn main() -> iced::Result {
    let settings = Settings {
        window: window::Settings {
            decorations: false,
            size: Size::new(800.0, 300.0),
            min_size: Some(Size::new(800.0, 300.0)),
            max_size: Some(Size::new(800.0, 500.0)),
            position: Position::Centered,
            resizable: false,
            transparent: true,
            ..Default::default()
        },
        antialiasing: true,
        ..Default::default()
    };

    App::run(settings)
}

#[derive(Debug, Clone)]
pub enum MainMessage {
    UpdateInput(String),
    SendToAI,
    AIResponse(Result<String, String>),
    Exit,
}

#[derive(Debug, Clone)]
pub enum State {
    Loading,
    Done,
}

pub struct App {
    text: String,
    ai_response: String,
    loading: State,
    error: Option<String>,
    settings_icon: svg::Handle,
    view: RouterView,
}

impl App {
    fn new() -> Self {
        Self {
            text: "".to_string(),
            ai_response: "".to_string(),
            loading: State::Done,
            error: None,
            settings_icon: svg::Handle::from_memory(
                include_bytes!("../assets/settings.svg").to_vec(),
            ),
            view: RouterView::Home,
        }
    }
}

impl Application for App {
    type Executor = executor::Default;
    type Message = MainMessage;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (App::new(), Command::none())
    }

    fn title(&self) -> String {
        String::from("AI Overlay")
    }

    fn update(&mut self, message: MainMessage) -> Command<MainMessage> {
        match message {
            MainMessage::UpdateInput(text) => {
                self.text = text;
                Command::none()
            }
            MainMessage::SendToAI => {
                let message_to_ai = self.text.clone();
                self.loading = State::Loading;
                Command::perform(ask_ai(message_to_ai), MainMessage::AIResponse)
            }
            MainMessage::AIResponse(result) => {
                match result {
                    Ok(response) => {
                        self.error = None;
                        self.ai_response = response;
                        self.text = "".to_string();
                    }
                    Err(e) => {
                        self.ai_response = "".to_string();
                        self.error = Some(e);
                    }
                };

                self.loading = State::Done;
                Command::none()
            }
            MainMessage::Exit => window::close(window::Id::MAIN),
        }
    }

    fn view(&self) -> Element<MainMessage> {
        let ai_input = search_bar(&self.text).into();

        let header = top_bar(self.settings_icon.clone()).into();

        let content = match (&self.loading, &self.ai_response, &self.error) {
            (State::Done, res, None) if !res.is_empty() => {
                let scroll = Scrollable::new(
                    column![
                        vertical_space().height(4),
                        container(text(res))
                            .height(Length::Fill)
                            .width(Length::Fill)
                            .padding([0, PADDING_SIZE, 0, PADDING_SIZE])
                    ]
                    .height(185),
                );

                column![
                    ai_input,
                    container(text("AI's response : ")).padding([
                        PADDING_SIZE,
                        0,
                        PADDING_SIZE,
                        PADDING_SIZE
                    ]),
                    horizontal_rule(1),
                    scroll
                ]
            }
            (State::Done, _, Some(err_msg)) => {
                column![
                    ai_input,
                    vertical_space().height(4),
                    container(text("There was an error :(")).center_x(),
                    vertical_space().height(4),
                    text(err_msg)
                ]
            }
            (State::Done, _, None) => {
                column![ai_input]
            }
            (State::Loading, _, _) => column![container(text("In progress ..."))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()],
        };

        container(column![header, content])
            .style(CustomTheme.appearance(&self.theme()))
            .height(Length::Shrink)
            .width(Length::Shrink)
            .padding(12)
            .center_x()
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        keyboard::on_key_press(|key, _modifiers| match key.as_ref() {
            keyboard::Key::Named(Named::Escape) => Some(MainMessage::Exit),
            keyboard::Key::Named(Named::Enter) => Some(MainMessage::SendToAI),
            _ => None,
        })
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::custom("Transparent".to_string(), get_palette_for_main_window())
    }

    fn style(&self) -> <Self::Theme as application::StyleSheet>::Style {
        <Self::Theme as application::StyleSheet>::Style::default()
    }
}
