use cohere_rust::{api::tokenize::TokenizeRequest, Cohere};

#[tokio::main]
async fn main() {
    let co = Cohere::default();

    let request = TokenizeRequest {
        text: "tokenize me! :D",
    };

    match co.tokenize(&request).await {
        Ok(r) => println!("Tokenize response: {:?}", r),
        Err(e) => {
            println!("Tokenize failed! {}", e)
        }
    }
}
