use iced::{widget::combo_box, window, Command};

use crate::{
    ai::{ask_ai, check_ai_health},
    config, App, AppState, MainMessage,
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

                    app.ai_response = response
                        .replace('\t', " ")
                        .replace("\n\n", "\n");

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
        MainMessage::UpdateConfigModel(new_model) => {
            app.config_settings.ai_model = new_model;
            config::save_settings(
                app.config_settings.clone(),
            );
            Command::none()
        }
        MainMessage::UpdateAvailableModels(models) => {
            app.available_models =
                combo_box::State::new(models);
            Command::none()
        }
        MainMessage::GetAvailableModels => {
            Command::perform(
                crate::ai::get_ai_models_installed(),
                |result| match result {
                    Ok(models) => {
                        MainMessage::UpdateAvailableModels(
                            models,
                        )
                    }
                    Err(_) => {
                        MainMessage::UpdateAvailableModels(
                            vec![],
                        )
                    }
                },
            )
        }
        MainMessage::Exit => {
            window::close(window::Id::MAIN)
        }
    }
}
