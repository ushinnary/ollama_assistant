use ollama_rs::generation::chat::request::ChatMessageRequest;
use ollama_rs::generation::chat::ChatMessage;
use ollama_rs::Ollama;

pub async fn ask_ai(message: String) -> Result<String, String> {
    let settings = crate::config::load_settings();

    let ollama = Ollama::default();
    dbg!(&message);

    ollama
        .send_chat_messages(ChatMessageRequest::new(
            settings.clone().ai_model,
            vec![ChatMessage::user(message)],
        ))
        .await
        .map(|res| res.message
            .map(|msg| msg.content))
        .map(|content| content.unwrap_or("".to_string()))
        .map_err(|err| err.to_string())
}
