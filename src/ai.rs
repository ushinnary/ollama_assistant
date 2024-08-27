use std::sync::LazyLock;

use iced::futures::lock::Mutex;
use ollama_rs::generation::chat::request::ChatMessageRequest;
use ollama_rs::generation::chat::ChatMessage;
use ollama_rs::Ollama;

static OLLAMA: LazyLock<Mutex<Ollama>> =
    LazyLock::new(|| Mutex::new(Ollama::new_default_with_history(30)));

pub async fn ask_ai(
    message: String,
    settings: crate::config::ApplicationSettings,
) -> Result<String, String> {
    let mut ollama = OLLAMA.lock().await;

    ollama
        .send_chat_messages_with_history(
            ChatMessageRequest::new(
                settings.ai_model.clone(),
                vec![ChatMessage::user(message)],
            ),
            &settings.ai_model,
        )
        .await
        .map(|res| res.message.map(|msg| msg.content))
        .map(|content| content.unwrap_or("".to_string()))
        .map_err(|err| err.to_string())
}

pub async fn check_ai_health() -> bool {
    let ollama = OLLAMA.lock().await;
    ollama.list_local_models().await.is_ok()
}

pub async fn get_ai_models_installed() -> Result<Vec<String>, String> {
    let ollama = OLLAMA.lock().await;
    ollama
        .list_local_models()
        .await
        .map(|models| {
            models.iter().map(|m| m.name.clone()).collect::<Vec<String>>()
        })
        .map_err(|err| err.to_string())
}

pub async fn reset_history() {
    let mut ollama = OLLAMA.lock().await;
    *ollama = Ollama::new_default_with_history(30);
}
