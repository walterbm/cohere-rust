use cohere_rust::api::chat::{ChatMessage, ChatRequest};
use cohere_rust::api::GenerateModel;
use cohere_rust::Cohere;

#[tokio::main]
async fn main() {
    let co = Cohere::default();

    let chat_history = vec![
        ChatMessage::User {
            message: "Hello! Tell me about Cohere.".to_string(),
        },
        ChatMessage::Chatbot {
            message: "Cohere is a startup based in Toronto.".to_string(),
        },
    ];

    let request = ChatRequest {
        message: "Tell me more.",
        model: Some(GenerateModel::CommandNightly),
        chat_history: Some(&chat_history),
        ..Default::default()
    };

    match co.chat(&request).await {
        Ok(mut rx) => {
            while let Some(message) = rx.recv().await {
                match message {
                    Ok(message) => println!("Chat response: {:#?}", message),
                    Err(e) => println!("Chat error! {:#?}", e),
                }
            }
        }
        Err(e) => {
            println!("Chat failed! {}", e)
        }
    }
}
