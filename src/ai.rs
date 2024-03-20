use ollama_rs::generation::chat::request::ChatMessageRequest;
use ollama_rs::generation::chat::ChatMessage;
use ollama_rs::Ollama;

pub async fn ask_ai(message: String) -> String {
    let settings = crate::config::load_settings();

    let ollama = Ollama::default();
    dbg!(&message);
    let response = ollama
        .send_chat_messages(ChatMessageRequest::new(
            settings.clone().ai_model,
            vec![ChatMessage::user(message)],
        ))
        .await
        .unwrap()
        .message
        .unwrap()
        .content;

    dbg!(&response);

    response
}
