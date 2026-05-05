use cohere_rust::Cohere;
use cohere_rust::api::chat_v2::{ChatV2Message, ChatV2Request};

#[tokio::main]
async fn main() {
    let co = Cohere::default();

    println!("Client: {:?}", co);

    let _co_clone = co.clone();

    let request = ChatV2Request::new("command-a-03-2025", "What is the capital of France?");
    match co.chat_v2_non_stream(&request).await {
        Ok(response) => {
            println!("Response: {}", response.message.text().unwrap_or("No text"));
            println!("Finish reason: {}", response.finish_reason);
        }
        Err(e) => println!("Error: {}", e),
    }

    let request = ChatV2Request {
        model: "command-a-03-2025".to_string(),
        messages: vec![
            ChatV2Message::system("You respond in concise sentences."),
            ChatV2Message::user("Hello"),
            ChatV2Message::assistant("Hi, how can I help you today?"),
            ChatV2Message::user("Tell me about Rust programming language."),
        ],
        ..ChatV2Request::new("command-a-03-2025", "")
    };
    match co.chat_v2_non_stream(&request).await {
        Ok(response) => {
            println!(
                "\nMulti-turn response: {}",
                response.message.text().unwrap_or("No text")
            );
        }
        Err(e) => println!("Error: {}", e),
    }

    let request = ChatV2Request::new("command-a-03-2025", "Tell me a short joke about Rust.");
    match co.chat_v2(&request).await {
        Ok(mut stream) => {
            print!("\nStreaming: ");
            while let Some(event) = stream.recv().await {
                match event {
                    Ok(event) => {
                        if let Some(text) = event.text() {
                            print!("{}", text);
                        }
                    }
                    Err(e) => println!("\nStream error: {}", e),
                }
            }
            println!();
        }
        Err(e) => println!("Error: {}", e),
    }
}
