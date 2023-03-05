use cohere_rust::api::{
    generate::{GenerateRequest, ReturnLikelihoods},
    Truncate,
};
use cohere_rust::Cohere;

#[tokio::main]
async fn main() {
    let co = Cohere::default();

    let request = GenerateRequest {
        max_tokens: Some(20),
        return_likelihoods: Some(ReturnLikelihoods::None),
        truncate: Some(Truncate::End),
        prompt: "Once upon a time in a magical land called".to_string(),
        ..Default::default()
    };

    match co.generate(request).await {
        Ok(r) => println!("Generate response: {:?}", r),
        Err(e) => {
            dbg!(e);
            println!("Generate failed!")
        }
    }
}
