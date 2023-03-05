use cohere_rust::api::embed::EmbedModel;
use cohere_rust::api::Truncate;
use cohere_rust::Cohere;

#[tokio::main]
async fn main() {
    let co = Cohere::default();

    match co
        .embed(
            vec!["hello".to_string(), "goodbye".to_string()],
            Truncate::End,
            Some(EmbedModel::Small),
        )
        .await
    {
        Ok(r) => println!("Embed response: {:?}", r),
        Err(e) => {
            dbg!(e);
            println!("Embed failed!")
        }
    }
}
