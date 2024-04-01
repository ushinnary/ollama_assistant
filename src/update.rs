use iced::{window, Command};

use crate::{
    ai::{ask_ai, check_ai_health},
    App, AppState, MainMessage,
};

pub fn handle_update(
    app: &mut App,
    message: MainMessage,
) -> Command<MainMessage> {
    match message {
        MainMessage::UpdateInput(text) => {
            app.text = text;
            Command::none()
        }
        MainMessage::SendToAI => {
            let message_to_ai = app.text.clone();
            app.loading = AppState::Loading;
            Command::perform(
                ask_ai(
                    message_to_ai,
                    app.config_settings.clone(),
                ),
                MainMessage::AIResponse,
            )
        }
        MainMessage::AIResponse(result) => {
            match result {
                Ok(response) => {
                    app.error = None;
                    app.ai_response = response;
                    app.text = "".to_string();
                    app.is_ai_api_live = true;
                }
                Err(e) => {
                    app.ai_response = "".to_string();
                    app.error = Some(e);

                    return handle_update(
                        app,
                        MainMessage::RunAiHealthCheck,
                    );
                }
            };

            app.loading = AppState::Done;
            Command::none()
        }
        MainMessage::ChangeView(view) => {
            app.view = view;
            Command::none()
        }
        MainMessage::AiHealthCheck(is_live) => {
            app.is_ai_api_live = is_live;
            app.loading = AppState::Done;
            Command::none()
        }
        MainMessage::RunAiHealthCheck => {
            app.loading = AppState::Loading;
            Command::perform(
                check_ai_health(),
                MainMessage::AiHealthCheck,
            )
        }
        MainMessage::Exit => {
            window::close(window::Id::MAIN)
        }
    }
}
