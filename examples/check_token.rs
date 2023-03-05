use cohere_rust::Cohere;

#[tokio::main]
async fn main() {
    let co = Cohere::default();

    match co.check_api_key().await {
        Ok(_) => println!("COHERE_API_KEY is valid!"),
        Err(e) => {
            dbg!(e);
            println!("COHERE_API_KEY is not valid!")
        }
    }
}
