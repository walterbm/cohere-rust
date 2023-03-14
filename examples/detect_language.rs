use cohere_rust::{api::detect_language::DetectLanguageRequest, Cohere};

#[tokio::main]
async fn main() {
    let co = Cohere::default();

    let request = DetectLanguageRequest {
        texts: &["Hello Cohere!".to_string(), "Hola mis amigos!".to_string()],
    };

    match co.detect_language(&request).await {
        Ok(r) => println!("Detect language response: {:?}", r),
        Err(e) => {
            println!("Detect language failed! {}", e)
        }
    }
}
