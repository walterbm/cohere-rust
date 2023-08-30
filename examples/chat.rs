use cohere_rust::api::chat::ChatRequest;
use cohere_rust::api::GenerateModel;
use cohere_rust::Cohere;

#[tokio::main]
async fn main() {
    let co = Cohere::default();

    let request = ChatRequest {
        message: "Hello! Tell me about Cohere.",
        model: Some(GenerateModel::CommandNightly),
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
