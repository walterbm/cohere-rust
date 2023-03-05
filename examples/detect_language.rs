use cohere_rust::Cohere;

#[tokio::main]
async fn main() {
    let co = Cohere::default();

    match co
        .detect_language(vec![
            "Hello Cohere!".to_string(),
            "Hola mis amigos!".to_string(),
        ])
        .await
    {
        Ok(r) => println!("Detect language response: {:?}", r),
        Err(e) => {
            dbg!(e);
            println!("Detect language failed!")
        }
    }
}
