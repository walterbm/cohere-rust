use cohere_rust::Cohere;
use cohere_rust::api::embed::EmbedRequest;
use cohere_rust::api::{EmbedInputType, EmbedModel};

#[tokio::main]
async fn main() {
    let co = Cohere::default();

    let request = EmbedRequest {
        model: EmbedModel::EnglishV3,
        input_type: EmbedInputType::Classification,
        texts: &["hello".to_string(), "goodbye".to_string()],
        ..Default::default()
    };

    match co.embed(&request).await {
        Ok(r) => println!("Embed response: {:?}", r),
        Err(e) => {
            println!("Embed failed! {}", e)
        }
    }
}
