use cohere_rust::api::classify::{ClassifyExample, ClassifyRequest};
use cohere_rust::api::EmbedModel;
use cohere_rust::Cohere;

#[tokio::main]
async fn main() {
    let co = Cohere::default();

    let examples = &vec![
        ClassifyExample {
            text: "Dermatologists don't like her!",
            label: "Spam",
        },
        ClassifyExample {
            text: "Hello, open to this?",
            label: "Spam",
        },
        ClassifyExample {
            text: "I need help please wire me $1000 right now",
            label: "Spam",
        },
        ClassifyExample {
            text: "Nice to know you ;)",
            label: "Spam",
        },
        ClassifyExample {
            text: "Please help me?",
            label: "Spam",
        },
        ClassifyExample {
            text: "Your parcel will be delivered today",
            label: "Not spam",
        },
        ClassifyExample {
            text: "Review changes to our Terms and Conditions",
            label: "Not spam",
        },
        ClassifyExample {
            text: "Weekly sync notes",
            label: "Not spam",
        },
        ClassifyExample {
            text: "Re: Follow up from today's meeting",
            label: "Not spam",
        },
        ClassifyExample {
            text: "Pre-read for tomorrow",
            label: "Not spam",
        },
    ];

    let inputs = &vec![
        "Confirm your email address".to_string(),
        "hey i need u to send some $".to_string(),
    ];

    let request = ClassifyRequest {
        examples,
        inputs,
        model: Some(EmbedModel::EnglishLight),
        ..Default::default()
    };

    match co.classify(&request).await {
        Ok(r) => println!("Classify response: {:?}", r),
        Err(e) => {
            println!("Classify failed! {}", e)
        }
    }
}
