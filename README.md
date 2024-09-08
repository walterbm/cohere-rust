# Cohere Rust SDK

This crate provides a simplified interface with the [Cohere](https://cohere.com/) API in Rust.

## Documentation

See the [Cohere API's documentation](https://docs.cohere.com/).

Also see some code examples for the SDK [here](https://github.com/walterbm/cohere-rust/blob/main/examples).

## Usage

To use this crate, you must first obtain a [Cohere API key](https://dashboard.cohere.com/welcome/register). Once you have an API key you can either set it as the `COHERE_API_KEY` environment variable or pass it directly when constructing the client.

Additionally, this crate relies on the [tokio](https://tokio.rs/) async-runtime to make all the API operations non-blocking.

This is a basic example of the creating the client and using the `chat` endpoint.

```rust
use cohere_rust::api::chat::ChatRequest;
use cohere_rust::api::GenerateModel;
use cohere_rust::Cohere;

#[tokio::main]
async fn main() {
    // automatically reads API key from `COHERE_API_KEY`
    let co = Cohere::default();

    let request = ChatRequest {
        message: "Tell me a story about a magical land.",
        model: Some(GenerateModel::CommandR),
        ..Default::default()
    };

    match co.chat(&request).await {
        Ok(mut rx) => {
            while let Some(message) = rx.recv().await {
                match message {
                    Ok(message) => println!("Chat response: {:#?}", message),
                    Err(e) => println!("Chat error! {:#?}", e),
                }
            }
        }
        Err(e) => {
            println!("Chat failed! {}", e)
        }
    }
}
```

Example usage of other endpoints can be found [here](https://github.com/walterbm/cohere-rust/blob/main/examples).

## Versioning

This SDK supports the latest API version. For more information, please refer to the [Versioning Docs](https://docs.cohere.com/reference/versioning).

## Endpoints

For a full breakdown of endpoints and arguments, please consult the [Cohere Docs](https://docs.cohere.com/).

| Cohere Endpoint  | Function             |
| ---------------- | -------------------- |
| /generate        | co.generate()        |
| /chat            | co.chat()            |
| /embed           | co.embed()           |
| /rerank          | co.rerank()          |
| /classify        | co.classify()        |
| /tokenize        | co.tokenize()        |
| /detokenize      | co.detokenize()      |
| /check-api-key   | co.check_api_key()   |

## Responses

All of the endpoint functions will return a Cohere object corresponding to the endpoint (e.g. for generate, it would be `GenerateResponse`). The names of these fields and a detailed breakdown of the response body can be found in the [Cohere Docs](https://docs.cohere.com/).

## Errors

Unsuccessful API calls from the SDK will return an error. Please see the documentation's page on [errors](https://docs.cohere.com/errors-reference) for more information about what the errors mean.
