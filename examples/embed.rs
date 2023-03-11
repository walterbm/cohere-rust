use cohere_rust::api::embed::{EmbedModel, EmbedRequest};
use cohere_rust::api::Truncate;
use cohere_rust::Cohere;

#[tokio::main]
async fn main() {
    let co = Cohere::default();

    let request = EmbedRequest {
        model: Some(EmbedModel::Small),
        texts: vec!["hello".to_string(), "goodbye".to_string()],
        truncate: Truncate::End,
    };

    match co.embed(&request).await {
        Ok(r) => println!("Embed response: {:?}", r),
        Err(e) => {
            dbg!(e);
            println!("Embed failed!")
        }
    }
}
