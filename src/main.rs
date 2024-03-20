use iced::keyboard::key::Named;
use iced::widget::container::StyleSheet;
use iced::widget::{
    column, container, horizontal_rule, text, text_input, vertical_space, Scrollable,
};
use iced::window::Position;
use iced::{
    application, executor, keyboard, window, Application, Command, Element, Length, Settings, Size,
    Subscription,
};

use ai::ask_ai;

mod ai;
mod config;
mod theming;

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
        ..Default::default()
    };

    App::run(settings)
}

#[derive(Debug, Clone)]
pub enum Message {
    UpdateInput(String),
    SendToAI,
    AIResponse(String),
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
}

impl App {
    fn new() -> Self {
        Self {
            text: "".to_string(),
            ai_response: "".to_string(),
            loading: State::Done,
        }
    }
}

impl Application for App {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = iced::Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (App::new(), Command::none())
    }

    fn title(&self) -> String {
        String::from("AI Overlay")
    }

    fn update(&mut self, message: Message) -> Command<Message> {
        match message {
            Message::UpdateInput(text) => {
                self.text = text;
                Command::none()
            }
            Message::SendToAI => {
                let message_to_ai = self.text.clone();
                self.text = "".to_string();
                self.loading = State::Loading;
                Command::perform(ask_ai(message_to_ai), Message::AIResponse)
            }
            Message::AIResponse(res) => {
                // self.ai_response = res.replace("\n\n", " ");
                self.ai_response = res;
                self.loading = State::Done;
                Command::none()
            }
            Message::Exit => window::close(window::Id::MAIN),
        }
    }

    fn view(&self) -> Element<Message> {
        let ai_input = container(
            text_input("AI Message", &self.text)
                .style(iced::theme::TextInput::Custom(Box::new(
                    theming::CustomTheme,
                )))
                .on_input(Message::UpdateInput)
                .on_submit(Message::SendToAI),
        )
        .width(Length::Fill)
        .center_x();

        let content = match (&self.loading, &self.ai_response) {
            (State::Done, res) if !res.is_empty() => {
                let scroll = Scrollable::new(
                    column![
                        vertical_space().height(4),
                        container(text(res))
                            .height(Length::Fill)
                            .width(Length::Fill)
                            .padding([0, 8, 0, 8])
                    ]
                    .height(185),
                );

                column![
                    ai_input,
                    container(text("AI's response : ")).padding([8, 0, 8, 0]),
                    horizontal_rule(1),
                    scroll
                ]
            }
            (State::Done, _) => column![ai_input],
            (State::Loading, _) => column![container(text("In progress ..."))
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()],
        };

        container(content)
            .style(theming::CustomTheme.appearance(&theming::CustomStyle))
            .height(Length::Shrink)
            .width(Length::Shrink)
            .padding(20)
            .center_x()
            .into()
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        keyboard::on_key_press(|key, _modifiers| match key.as_ref() {
            keyboard::Key::Named(Named::Escape) => Some(Message::Exit),
            _ => None,
        })
    }

    fn theme(&self) -> Self::Theme {
        iced::Theme::custom(
            "Transparent".to_string(),
            theming::get_palette_for_main_window(),
        )
    }

    fn style(&self) -> <Self::Theme as application::StyleSheet>::Style {
        <Self::Theme as application::StyleSheet>::Style::default()
    }
}