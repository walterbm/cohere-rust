use cohere_rust::Cohere;

#[tokio::main]
async fn main() {
    let co = Cohere::default();

    match co.tokenize("tokenize me! :D".to_string()).await {
        Ok(r) => println!("Tokenize response: {:?}", r),
        Err(e) => {
            dbg!(e);
            println!("Tokenize failed!")
        }
    }
}
