use cohere_rust::{api::detokenize::DetokenizeRequest, Cohere};

#[tokio::main]
async fn main() {
    let co = Cohere::default();

    let request = DetokenizeRequest {
        tokens: vec![10104, 12221, 1315, 34, 1420, 69],
    };

    match co.detokenize(&request).await {
        Ok(r) => println!("Detokenize response: {:?}", r),
        Err(e) => {
            println!("Detokenize failed! {}", e)
        }
    }
}
