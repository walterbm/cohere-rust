use cohere_rust::{api::tokenize::TokenizeRequest, Cohere};

#[tokio::main]
async fn main() {
    let co = Cohere::default();

    let request = TokenizeRequest {
        text: "tokenize me! :D".to_string(),
    };

    match co.tokenize(&request).await {
        Ok(r) => println!("Tokenize response: {:?}", r),
        Err(e) => {
            dbg!(e);
            println!("Tokenize failed!")
        }
    }
}
