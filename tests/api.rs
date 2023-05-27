#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use cohere_rust::{
        api::{
            detect_language::{DetectLanguageResult, DetectLanguageRequest},
            generate::{GenerateRequest, ReturnLikelihoods},
            Truncate, summarize::{SummarizeRequest, SummarizeLength, SummarizeFormat, SummarizeModel, SummarizeExtractiveness}, classify::{ClassifyRequest, ClassifyExample, Classification, LabelProperties}, embed::EmbedRequest, tokenize::TokenizeRequest, detokenize::DetokenizeRequest, rerank::{ReRankRequest, ReRankModel, ReRankResult},
        },
        Cohere,
    };

    #[tokio::test]
    async fn test_classify() {
        // Create mock server
        let mut mock_server = mockito::Server::new_async().await;
        let mock_url = mock_server.url();

        // Create a mock
        let mock_endpoint = mock_server
            .mock("POST", "/classify")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "id": "64a2c222-e40c-477f-8764-7db1c28c6471",
                    "classifications": [
                      {
                        "id": "e09487b4-dd33-43a3-9732-139e6652b289",
                        "input": "Confirm your email address",
                        "prediction": "Not spam",
                        "confidence": 0.80833024,
                        "labels": {
                          "Not spam": {
                            "confidence": 0.80833024
                          },
                          "Spam": {
                            "confidence": 0.19166975
                          }
                        }
                      },
                      {
                        "id": "0d5fdc21-1bb9-4673-9fea-c6858af1db08",
                        "input": "hey i need u to send some $",
                        "prediction": "Spam",
                        "confidence": 0.9893047,
                        "labels": {
                          "Not spam": {
                            "confidence": 0.010695281
                          },
                          "Spam": {
                            "confidence": 0.9893047
                          }
                        }
                      }
                    ],
                    "meta": {
                      "api_version": {
                        "version": "1"
                      }
                    }
                  }"#,
            )
            .create_async()
            .await;

        let client = Cohere::new(mock_url, "test-key", "test-version");

        let response = client
            .classify(&ClassifyRequest {
                inputs: &["Confirm your email address".to_string(),
                    "hey i need u to send some $".to_string()],
                examples: &vec![
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
                ],
                ..Default::default()
            })
            .await;

        // assert that mock endpoint was called
        mock_endpoint.assert_async().await;

        assert!(response.is_ok());

        let response = response.unwrap();

        assert_eq!(2, response.len());

        assert_eq!(Classification {
            id: "e09487b4-dd33-43a3-9732-139e6652b289".to_string(),
            prediction: "Not spam".to_string(),
            confidence: 0.80833024,
            labels: HashMap::from([
                ("Not spam".to_string(), LabelProperties {confidence: 0.80833024}),
                ("Spam".to_string(), LabelProperties {confidence: 0.19166975})
            ]),
            input: "Confirm your email address".to_string(),
        }, response[0]);

        assert_eq!(Classification {
            id: "0d5fdc21-1bb9-4673-9fea-c6858af1db08".to_string(),
            prediction: "Spam".to_string(),
            confidence: 0.9893047,
            labels: HashMap::from([
                ("Not spam".to_string(), LabelProperties {confidence: 0.010695281}),
                ("Spam".to_string(), LabelProperties {confidence: 0.9893047})
            ]),
            input: "hey i need u to send some $".to_string(),
        }, response[1]);
    }

    #[tokio::test]

    async fn test_detect_language() {
        // Create mock server
        let mut mock_server = mockito::Server::new_async().await;
        let mock_url = mock_server.url();

        // Create a mock
        let mock_endpoint = mock_server
            .mock("POST", "/detect-language")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                "id":"e09aaf3b-5806-4b54-9b9d-110d262e2baf",
                "results":[
                    {"language_code":"en","language_name":"English"},
                    {"language_code":"es","language_name":"Spanish"}
                ],
                "meta":{"api_version":{"version":"1"}}}"#,
            )
            .create_async()
            .await;

        let client = Cohere::new(mock_url, "test-key", "test-version");

        let request = DetectLanguageRequest {
            texts: &[
                "Hello Cohere!".to_string(),
                "Hola mis amigos!".to_string(),
            ]
        };

        let response = client
            .detect_language(&request)
            .await;

        // assert that mock endpoint was called
        mock_endpoint.assert_async().await;

        assert!(response.is_ok());

        let response = response.unwrap();

        assert_eq!(2, response.len());

        assert_eq!(
            vec![
                DetectLanguageResult {
                    language_code: "en".to_string(),
                    language_name: "English".to_string(),
                },
                DetectLanguageResult {
                    language_code: "es".to_string(),
                    language_name: "Spanish".to_string(),
                }
            ],
            response
        );
    }

    #[tokio::test]
    async fn test_detokenize() {
        // Create mock server
        let mut mock_server = mockito::Server::new_async().await;
        let mock_url = mock_server.url();

        // Create a mock
        let mock_endpoint = mock_server
            .mock("POST", "/detokenize")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "text": "detokenized! :D",
                    "meta": {
                      "api_version": {
                        "version": "1"
                      }
                    }
                  }"#,
            )
            .create_async()
            .await;

        let client = Cohere::new(mock_url, "test-key", "test-version");

        let request = DetokenizeRequest {
            tokens: &[10104, 12221, 1315, 34, 1420, 69],
        };

        let response = client
            .detokenize(&request)
            .await;

        // assert that mock endpoint was called
        mock_endpoint.assert_async().await;

        assert!(response.is_ok());

        let response = response.unwrap();

        assert_eq!("detokenized! :D".to_string(), response);
    }

    #[tokio::test]
    async fn test_embed() {
        // Create mock server
        let mut mock_server = mockito::Server::new_async().await;
        let mock_url = mock_server.url();

        let mock_embeddings: Vec<f64> = vec![
            1.1484375,
            1.703125,
            -0.45654297,
            -1.2236328,
            -1.0341797,
            -0.42114258,
            1.5732422,
            -2.3652344,
            0.7109375,
            1.4482422,
            0.3322754,
            -0.66845703,
            0.38183594,
            -0.83251953,
            -2.125,
            0.11755371,
            0.8574219,
            -0.47827148,
            -0.8745117,
            -2.5253906,
            0.033172607,
            -1.171875,
            1.0205078,
            1.0771484,
            0.20349121,
            -0.13110352,
            2.0,
        ];

        // Create a mock
        let mock_endpoint = mock_server
            .mock("POST", "/embed")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                format!("{{\"id\":\"3915734e-bd8e-4ff4-9be1-f6dfd29dc386\",\"texts\":[\"hi\"],\"embeddings\":[{}],\"meta\":{{\"api_version\":{{\"version\":\"1\"}}}}}}", serde_json::to_string(&mock_embeddings).unwrap()),
            )
            .create_async()
            .await;

        let client = Cohere::new(mock_url, "test-key", "test-version");

        let request = EmbedRequest {
            model: None,
            texts: &["hi".to_string()],
            truncate: Truncate::End,
        };

        let response = client
            .embed(&request)
            .await;

        // assert that mock endpoint was called
        mock_endpoint.assert_async().await;

        assert!(response.is_ok());

        let response = response.unwrap();

        assert_eq!(1, response.len());

        assert_eq!(mock_embeddings, response[0]);
    }

    #[tokio::test]
    async fn test_generate() {
        // Create mock server
        let mut mock_server = mockito::Server::new_async().await;
        let mock_url = mock_server.url();

        // Create a mock
        let mock_endpoint = mock_server
            .mock("POST", "/generate")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "id": "65e5ecf2-0872-45d2-b15c-f59647273e97",
                    "generations": [
                      {
                        "id": "ef047b8a-0231-40e1-9f58-af7b135ce7d7",
                        "text": " Silicon Valley, there was a very unusual sight: An actual new idea. It was a strange sight"
                      }
                    ],
                    "prompt": "Once upon a time in a magical land called",
                    "meta": {
                      "api_version": {
                        "version": "1"
                      }
                    }
                  }"#,
            )
            .create_async()
            .await;

        let client = Cohere::new(mock_url, "test-key", "test-version");

        let response = client
            .generate(&GenerateRequest {
                max_tokens: Some(20),
                return_likelihoods: Some(ReturnLikelihoods::None),
                truncate: Some(Truncate::End),
                prompt: "Once upon a time in a magical land called",
                ..Default::default()
            })
            .await;

        // assert that mock endpoint was called
        mock_endpoint.assert_async().await;

        assert!(response.is_ok());

        let response = response.unwrap();

        assert_eq!(1, response.len());

        assert_eq!(" Silicon Valley, there was a very unusual sight: An actual new idea. It was a strange sight".to_string(), response[0].text);
    }

    #[tokio::test]
    async fn test_summarize() {
        // Create mock server
        let mut mock_server = mockito::Server::new_async().await;
        let mock_url = mock_server.url();

        // Create a mock
        let mock_endpoint = mock_server
            .mock("POST", "/summarize")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "id": "52fe10e7-0bb9-4b7d-84e5-36422f8e7343",
                    "summary": "What is ice cream? How is it made? What is its history? Find out all about ice cream on the BBC Food website.",
                    "meta": {
                      "api_version": {
                        "version": "1"
                      }
                    }
                  }"#,
            )
            .create_async()
            .await;

        let client = Cohere::new(mock_url, "test-key", "test-version");

        let response = client
            .summarize(&SummarizeRequest { 
                text: "Ice cream is a sweetened frozen food typically eaten as a snack or dessert. It may be made from milk or cream and is flavoured with a sweetener, either sugar or an alternative, and a spice, such as cocoa or vanilla, or with fruit such as strawberries or peaches. It can also be made by whisking a flavored cream base and liquid nitrogen together. Food coloring is sometimes added, in addition to stabilizers. The mixture is cooled below the freezing point of water and stirred to incorporate air spaces and to prevent detectable ice crystals from forming. The result is a smooth, semi-solid foam that is solid at very low temperatures (below 2 °C or 35 °F). It becomes more malleable as its temperature increases.\n\nThe meaning of the name \"ice cream\" varies from one country to another. In some countries, such as the United States, \"ice cream\" applies only to a specific variety, and most governments regulate the commercial use of the various terms according to the relative quantities of the main ingredients, notably the amount of cream. Products that do not meet the criteria to be called ice cream are sometimes labelled \"frozen dairy dessert\" instead. In other countries, such as Italy and Argentina, one word is used fo\r all variants. Analogues made from dairy alternatives, such as goat's or sheep's milk, or milk substitutes (e.g., soy, cashew, coconut, almond milk or tofu), are available for those who are lactose intolerant, allergic to dairy protein or vegan.",
                length: Some(SummarizeLength::Medium),
                format: Some(SummarizeFormat::Paragraph),
                model: Some(SummarizeModel::XLarge),
                extractiveness: Some(SummarizeExtractiveness::Low),
                temperature: Some(0.3),
                ..Default::default()
            })
            .await;

        // assert that mock endpoint was called
        mock_endpoint.assert_async().await;

        assert!(response.is_ok());

        let response = response.unwrap();

        assert_eq!("What is ice cream? How is it made? What is its history? Find out all about ice cream on the BBC Food website.".to_string(), response);
    }

    #[tokio::test]
    async fn test_tokenize() {
        // Create mock server
        let mut mock_server = mockito::Server::new_async().await;
        let mock_url = mock_server.url();

        // Create a mock
        let mock_endpoint = mock_server
            .mock("POST", "/tokenize")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "tokens": [
                      34160,
                      974,
                      514,
                      34,
                      1420,
                      69
                    ],
                    "token_strings": [
                      "token",
                      "ize",
                      " me",
                      "!",
                      " :",
                      "D"
                    ],
                    "meta": {
                      "api_version": {
                        "version": "1"
                      }
                    }
                  }"#,
            )
            .create_async()
            .await;

        let client = Cohere::new(mock_url, "test-key", "test-version");

        let request = TokenizeRequest {
            text: "tokenize me! :D"
        };

        let response = client
            .tokenize(&request)
            .await;

        // assert that mock endpoint was called
        mock_endpoint.assert_async().await;

        assert!(response.is_ok());

        let response = response.unwrap();

        assert_eq!(vec![34160,974,514,34,1420,69], response.tokens);
        assert_eq!(vec!["token","ize"," me","!"," :","D"], response.token_strings);
    }

    #[tokio::test]

    async fn test_rerank() {
        // Create mock server
        let mut mock_server = mockito::Server::new_async().await;
        let mock_url = mock_server.url();

        // Create a mock
        let mock_endpoint = mock_server
            .mock("POST", "/rerank")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(
                r#"{
                    "id": "1004c6d5-873b-4899-9072-6a13c40f19a7",
                    "results": [
                        {
                        "index": 2,
                        "relevance_score": 0.98005307
                        },
                        {
                        "index": 3,
                        "relevance_score": 0.27904198
                        },
                        {
                        "index": 0,
                        "relevance_score": 0.10194652
                        },
                        {
                        "index": 1,
                        "relevance_score": 0.0721122
                        }
                    ],
                    "meta": {
                        "api_version": {
                        "version": "1"
                        }
                    }
                }"#,
            )
            .create_async()
            .await;

        let client = Cohere::new(mock_url, "test-key", "test-version");

        let documents = [
            "Carson City is the capital city of the American state of Nevada.",
            "The Commonwealth of the Northern Mariana Islands is a group of islands in the Pacific Ocean. Its capital is Saipan.",
            "Washington, D.C. (also known as simply Washington or D.C., and officially as the District of Columbia) is the capital of the United States. It is a federal district.",
            "Capital punishment (the death penalty) has existed in the United States since beforethe United States was a country. As of 2017, capital punishment is legal in 30 of the 50 states.",
        ];

        let request = ReRankRequest {
            query: "What is the capital of the United States?",
            documents: &documents.map(|d| d.to_string()),
            model: ReRankModel::English,
            top_n: Some(4),
            ..Default::default()
        };

        let response = client
            .rerank(&request)
            .await;

        // assert that mock endpoint was called
        mock_endpoint.assert_async().await;

        println!("{:?}", response);

        assert!(response.is_ok());

        let response = response.unwrap();

        assert_eq!(4, response.len());

        assert_eq!(
            vec![
                ReRankResult {
                    index: 2,
                    relevance_score: 0.98005307
                  },
                  ReRankResult {
                    index: 3,
                    relevance_score: 0.27904198
                  },
                  ReRankResult {
                    index: 0,
                    relevance_score: 0.10194652
                  },
                  ReRankResult {
                    index: 1,
                    relevance_score: 0.0721122
                  }
            ],
            response
        );
    }

    #[tokio::test]
    async fn test_api_failure() {
        // Create mock server
        let mut mock_server = mockito::Server::new_async().await;
        let mock_url = mock_server.url();

        // Create a mock
        let mock_endpoint = mock_server
            .mock("POST", "/tokenize")
            .with_status(500)
            .with_header("content-type", "application/json")
            .with_body(r#"{"message":"invalid request: inputs cannot be empty"}"#)
            .create_async()
            .await;

        let client = Cohere::new(mock_url, "test-key", "test-version");

        let request = TokenizeRequest {
            text: ""
        };

        let response = client
            .tokenize(&request)
            .await;

        // assert that mock endpoint was called
        mock_endpoint.assert_async().await;

        assert!(response.is_err());

        let response = response.err().unwrap();

        assert_eq!("API request failed with status code `500 Internal Server Error` and error message `invalid request: inputs cannot be empty`", response.to_string());
    }
}
