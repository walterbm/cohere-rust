use cohere_rust::api::embed::EmbedRequest;
use cohere_rust::api::{EmbedModel, Truncate};
use cohere_rust::Cohere;

#[tokio::main]
async fn main() {
    let co = Cohere::default();

    let request = EmbedRequest {
        model: Some(EmbedModel::EnglishLightV2),
        texts: &["hello".to_string(), "goodbye".to_string()],
        truncate: Truncate::End,
    };

    match co.embed(&request).await {
        Ok(r) => println!("Embed response: {:?}", r),
        Err(e) => {
            println!("Embed failed! {}", e)
        }
    }
}
