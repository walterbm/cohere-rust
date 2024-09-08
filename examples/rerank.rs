use cohere_rust::{
    api::rerank::{ReRankModel, ReRankRequest},
    Cohere,
};

#[tokio::main]
async fn main() {
    let co = Cohere::default();

    let documents = [
        "Carson City is the capital city of the American state of Nevada.",
        "The Commonwealth of the Northern Mariana Islands is a group of islands in the Pacific Ocean. Its capital is Saipan.",
        "Washington, D.C. (also known as simply Washington or D.C., and officially as the District of Columbia) is the capital of the United States. It is a federal district.",
        "Capital punishment (the death penalty) has existed in the United States since beforethe United States was a country. As of 2017, capital punishment is legal in 30 of the 50 states.",
    ];

    let request = ReRankRequest {
        query: "What is the capital of the United States?",
        documents: &documents.map(|d| d.to_string()),
        model: ReRankModel::EnglishV3,
        top_n: Some(2),
        ..Default::default()
    };

    match co.rerank(&request).await {
        Ok(r) => {
            let ranked_documents: Vec<&str> = r
                .iter()
                .map(|rank| documents[rank.index as usize])
                .collect();
            println!("Rerank response: {:?}", r);
            println!("Rerank documents: {:?}", ranked_documents);
        }
        Err(e) => {
            println!("Rerank failed! {}", e)
        }
    }
}
