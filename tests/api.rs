#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use cohere_rust::{
        api::{
            chat::{ChatRequest, ChatResponse, ChatStreamResponse},
            classify::{Classification, ClassifyExample, ClassifyRequest, LabelProperties},
            detokenize::DetokenizeRequest,
            embed::EmbedRequest,
            generate::{GenerateRequest, ReturnLikelihoods},
            rerank::{ReRankModel, ReRankRequest, ReRankResult},
            tokenize::TokenizeRequest,
            GenerateModel, Truncate,
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

        let client = Cohere::new(mock_url, "test-key");

        let response = client
            .classify(&ClassifyRequest {
                inputs: &[
                    "Confirm your email address".to_string(),
                    "hey i need u to send some $".to_string(),
                ],
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

        assert_eq!(
            Classification {
                id: "e09487b4-dd33-43a3-9732-139e6652b289".to_string(),
                prediction: "Not spam".to_string(),
                confidence: 0.80833024,
                labels: HashMap::from([
                    (
                        "Not spam".to_string(),
                        LabelProperties {
                            confidence: 0.80833024
                        }
                    ),
                    (
                        "Spam".to_string(),
                        LabelProperties {
                            confidence: 0.19166975
                        }
                    )
                ]),
                input: "Confirm your email address".to_string(),
            },
            response[0]
        );

        assert_eq!(
            Classification {
                id: "0d5fdc21-1bb9-4673-9fea-c6858af1db08".to_string(),
                prediction: "Spam".to_string(),
                confidence: 0.9893047,
                labels: HashMap::from([
                    (
                        "Not spam".to_string(),
                        LabelProperties {
                            confidence: 0.010695281
                        }
                    ),
                    (
                        "Spam".to_string(),
                        LabelProperties {
                            confidence: 0.9893047
                        }
                    )
                ]),
                input: "hey i need u to send some $".to_string(),
            },
            response[1]
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

        let client = Cohere::new(mock_url, "test-key");

        let request = DetokenizeRequest {
            tokens: &[10104, 12221, 1315, 34, 1420, 69],
            model: Some(GenerateModel::CommandNightly),
        };

        let response = client.detokenize(&request).await;

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

        let client = Cohere::new(mock_url, "test-key");

        let request = EmbedRequest {
            model: None,
            texts: &["hi".to_string()],
            truncate: Truncate::End,
        };

        let response = client.embed(&request).await;

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

        let client = Cohere::new(mock_url, "test-key");

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
    async fn test_chat() {
        // Create mock server
        let mut mock_server = mockito::Server::new_async().await;
        let mock_url = mock_server.url();

        let mock_chat_stream = [
            "{\"is_finished\":false,\"event_type\":\"stream-start\",\"generation_id\":\"0c9cb118-f841-4588-b835-f9a4fe2c572e\"}\n",
            "{\"is_finished\":false,\"event_type\":\"text-generation\",\"text\":\" Thomas\"}\n",
            "{\"is_finished\":false,\"event_type\":\"text-generation\",\"text\":\" P\"}\n",
            "{\"is_finished\":false,\"event_type\":\"text-generation\",\"text\":\".\"}\n",
            "{\"is_finished\":false,\"event_type\":\"text-generation\",\"text\":\" Frank\"}\n",
            "{\"is_finished\":false,\"event_type\":\"text-generation\",\"text\":\".\"}\n",
            "{\"is_finished\":true,\"event_type\":\"stream-end\",\"response\":{\"response_id\":\"feab94ed-789b-42f2-8f4f-c49d56d28734\",\"text\":\"Thomas P. Frank.\",\"generation_id\":\"0c9cb118-f841-4588-b835-f9a4fe2c572e\",\"token_count\":{\"prompt_tokens\":71,\"response_tokens\":17,\"total_tokens\":88,\"billed_tokens\":77}},\"finish_reason\":\"COMPLETE\"}\n",
        ];

        // Create a mock
        let mock_endpoint = mock_server
            .mock("POST", "/chat")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_chunked_body(move |w| {
                for chunk in mock_chat_stream.iter() {
                    w.write_all(chunk.as_bytes()).unwrap();
                }
                Ok(())
            })
            .create_async()
            .await;

        let client = Cohere::new(mock_url, "test-key");

        let response = client
            .chat(&ChatRequest {
                message: "who wrote the book where is my cheese?",
                ..Default::default()
            })
            .await;

        // assert that mock endpoint was called
        mock_endpoint.assert_async().await;

        assert!(response.is_ok());

        let mut stream = response.unwrap();
        let expected_messages = [
            ChatStreamResponse::ChatStreamStart {
                generation_id: "0c9cb118-f841-4588-b835-f9a4fe2c572e".to_string(),
                is_finished: false,
            },
            ChatStreamResponse::ChatTextGeneration {
                is_finished: false,
                text: " Thomas".to_string(),
            },
            ChatStreamResponse::ChatTextGeneration {
                is_finished: false,
                text: " P".to_string(),
            },
            ChatStreamResponse::ChatTextGeneration {
                is_finished: false,
                text: ".".to_string(),
            },
            ChatStreamResponse::ChatTextGeneration {
                is_finished: false,
                text: " Frank".to_string(),
            },
            ChatStreamResponse::ChatTextGeneration {
                is_finished: false,
                text: ".".to_string(),
            },
            ChatStreamResponse::ChatStreamEnd {
                finish_reason: "COMPLETE".to_string(),
                is_finished: true,
                response: ChatResponse {
                    generation_id: "0c9cb118-f841-4588-b835-f9a4fe2c572e".to_string(),
                    response_id: "feab94ed-789b-42f2-8f4f-c49d56d28734".to_string(),
                    text: "Thomas P. Frank.".to_string(),
                },
            },
        ];

        let mut count: usize = 0;
        while let Some(message) = stream.recv().await {
            assert!(message.is_ok());
            assert_eq!(expected_messages[count], message.unwrap());
            count += 1;
        }
        assert_eq!(expected_messages.len(), count);
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

        let client = Cohere::new(mock_url, "test-key");

        let request = TokenizeRequest {
            text: "tokenize me! :D",
            model: Some(GenerateModel::CommandNightly),
        };

        let response = client.tokenize(&request).await;

        // assert that mock endpoint was called
        mock_endpoint.assert_async().await;

        assert!(response.is_ok());

        let response = response.unwrap();

        assert_eq!(vec![34160, 974, 514, 34, 1420, 69], response.tokens);
        assert_eq!(
            vec!["token", "ize", " me", "!", " :", "D"],
            response.token_strings
        );
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

        let client = Cohere::new(mock_url, "test-key");

        let documents = [
            "Carson City is the capital city of the American state of Nevada.",
            "The Commonwealth of the Northern Mariana Islands is a group of islands in the Pacific Ocean. Its capital is Saipan.",
            "Washington, D.C. (also known as simply Washington or D.C., and officially as the District of Columbia) is the capital of the United States. It is a federal district.",
            "Capital punishment (the death penalty) has existed in the United States since beforethe United States was a country. As of 2017, capital punishment is legal in 30 of the 50 states.",
        ];

        let request = ReRankRequest {
            query: "What is the capital of the United States?",
            documents: &documents.map(|d| d.to_string()),
            model: ReRankModel::EnglishV2,
            top_n: Some(4),
            ..Default::default()
        };

        let response = client.rerank(&request).await;

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

        let client = Cohere::new(mock_url, "test-key");

        let request = TokenizeRequest {
            text: "",
            model: None,
        };

        let response = client.tokenize(&request).await;

        // assert that mock endpoint was called
        mock_endpoint.assert_async().await;

        assert!(response.is_err());

        let response = response.err().unwrap();

        assert_eq!("API request failed with status code `500 Internal Server Error` and error message `invalid request: inputs cannot be empty`", response.to_string());
    }
}
