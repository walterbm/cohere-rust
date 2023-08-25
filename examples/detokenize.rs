use cohere_rust::{api::detokenize::DetokenizeRequest, api::GenerateModel, Cohere};

#[tokio::main]
async fn main() {
    let co = Cohere::default();

    let request = DetokenizeRequest {
        tokens: &[10002, 2261, 2012, 8, 2792, 43],
        model: Some(GenerateModel::Command),
    };

    match co.detokenize(&request).await {
        Ok(r) => println!("Detokenize response: {:?}", r),
        Err(e) => {
            println!("Detokenize failed! {}", e)
        }
    }
}
