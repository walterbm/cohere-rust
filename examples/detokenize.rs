use cohere_rust::Cohere;

#[tokio::main]
async fn main() {
    let co = Cohere::default();

    match co.detokenize(vec![10104, 12221, 1315, 34, 1420, 69]).await {
        Ok(r) => println!("Detokenize response: {:?}", r),
        Err(e) => {
            dbg!(e);
            println!("Detokenize failed!")
        }
    }
}
