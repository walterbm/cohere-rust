use cohere_rust::Cohere;
use cohere_rust::api::summarize::{SummarizeRequest, SummarizeLength, SummarizeFormat, SummarizeExtractiveness, SummarizeModel};

#[tokio::main]
async fn main() {
    let co = Cohere::default();

    let request = SummarizeRequest { 
        text: "Ice cream is a sweetened frozen food typically eaten as a snack or dessert. It may be made from milk or cream and is flavoured with a sweetener, either sugar or an alternative, and a spice, such as cocoa or vanilla, or with fruit such as strawberries or peaches. It can also be made by whisking a flavored cream base and liquid nitrogen together. Food coloring is sometimes added, in addition to stabilizers. The mixture is cooled below the freezing point of water and stirred to incorporate air spaces and to prevent detectable ice crystals from forming. The result is a smooth, semi-solid foam that is solid at very low temperatures (below 2 °C or 35 °F). It becomes more malleable as its temperature increases.\n\nThe meaning of the name \"ice cream\" varies from one country to another. In some countries, such as the United States, \"ice cream\" applies only to a specific variety, and most governments regulate the commercial use of the various terms according to the relative quantities of the main ingredients, notably the amount of cream. Products that do not meet the criteria to be called ice cream are sometimes labelled \"frozen dairy dessert\" instead. In other countries, such as Italy and Argentina, one word is used fo\r all variants. Analogues made from dairy alternatives, such as goat's or sheep's milk, or milk substitutes (e.g., soy, cashew, coconut, almond milk or tofu), are available for those who are lactose intolerant, allergic to dairy protein or vegan.".to_string(),
        length: Some(SummarizeLength::Medium),
        format: Some(SummarizeFormat::Paragraph),
        model: Some(SummarizeModel::XLarge),
        extractiveness: Some(SummarizeExtractiveness::Low),
        temperature: Some(0.3),
        ..Default::default()
    };

    match co.summarize(&request).await {
        Ok(r) => println!("Summarize response: {:?}", r),
        Err(e) => {
            println!("Summarize failed! {}", e)
        }
    }
}
