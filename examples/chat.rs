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
        Ok(r) => println!("Chat response: {:?}", r),
        Err(e) => {
            println!("Chat failed! {}", e)
        }
    }
}
