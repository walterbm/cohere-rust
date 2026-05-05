use cohere_rust::{Cohere, api::detokenize::DetokenizeRequest};

#[tokio::main]
async fn main() {
    let co = Cohere::default();

    let request = DetokenizeRequest {
        tokens: &[206039, 2307, 8, 2168, 43],
    };

    match co.detokenize(&request).await {
        Ok(r) => println!("Detokenize response: {:?}", r),
        Err(e) => {
            println!("Detokenize failed! {}", e)
        }
    }
}
