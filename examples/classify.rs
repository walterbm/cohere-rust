use cohere_rust::api::classify::{ClassifyExample, ClassifyRequest};
use cohere_rust::Cohere;

#[tokio::main]
async fn main() {
    let co = Cohere::default();

    let examples = vec![
        ClassifyExample {
            text: "Dermatologists don't like her!".to_string(),
            label: "Spam".to_string(),
        },
        ClassifyExample {
            text: "Hello, open to this?".to_string(),
            label: "Spam".to_string(),
        },
        ClassifyExample {
            text: "I need help please wire me $1000 right now".to_string(),
            label: "Spam".to_string(),
        },
        ClassifyExample {
            text: "Nice to know you ;)".to_string(),
            label: "Spam".to_string(),
        },
        ClassifyExample {
            text: "Please help me?".to_string(),
            label: "Spam".to_string(),
        },
        ClassifyExample {
            text: "Your parcel will be delivered today".to_string(),
            label: "Not spam".to_string(),
        },
        ClassifyExample {
            text: "Review changes to our Terms and Conditions".to_string(),
            label: "Not spam".to_string(),
        },
        ClassifyExample {
            text: "Weekly sync notes".to_string(),
            label: "Not spam".to_string(),
        },
        ClassifyExample {
            text: "Re: Follow up from today's meeting".to_string(),
            label: "Not spam".to_string(),
        },
        ClassifyExample {
            text: "Pre-read for tomorrow".to_string(),
            label: "Not spam".to_string(),
        },
    ];

    let inputs = vec![
        "Confirm your email address".to_string(),
        "hey i need u to send some $".to_string(),
    ];

    let request = ClassifyRequest {
        examples,
        inputs,
        ..Default::default()
    };

    match co.classify(request).await {
        Ok(r) => println!("Classify response: {:?}", r),
        Err(e) => {
            dbg!(e);
            println!("Classify failed!")
        }
    }
}
