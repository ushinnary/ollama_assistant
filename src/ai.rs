use iced::futures::lock::Mutex;
use lazy_static::lazy_static;
use ollama_rs::generation::chat::request::ChatMessageRequest;
use ollama_rs::generation::chat::ChatMessage;
use ollama_rs::Ollama;

use crate::debug;

lazy_static! {
    static ref OLLAMA: Mutex<Ollama> = Mutex::new(Ollama::new_default_with_history(30));
}

pub async fn ask_ai(message: String) -> Result<String, String> {
    let settings = crate::config::load_settings();

    let mut ollama = OLLAMA.lock().await;
    debug!(&message);
    debug!(ollama.get_messages_history("user".to_string()));

    ollama
        .send_chat_messages_with_history(
            ChatMessageRequest::new(settings.clone().ai_model, vec![ChatMessage::user(message)]),
            "user".to_string(),
        )
        .await
        .map(|res| res.message.map(|msg| msg.content))
        .map(|content| content.unwrap_or("".to_string()))
        .map_err(|err| err.to_string())
}
