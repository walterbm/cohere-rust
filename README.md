# Cohere Rust SDK

This crate provided a simplified interface with the [cohere.ai](https://cohere.ai/) API in Rust.

## Documentation

See the [Cohere API's documentation](https://docs.cohere.ai/).

Also see some code examples for the SDK [here](https://github.com/walterbm/cohere-rust/blob/main/examples).

## Usage

To use this crate, you must first obtain a [Cohere API key](https://dashboard.cohere.ai/welcome/register). Once you have an API key you can either set it as the `COHERE_API_KEY` environment variable or pass it directly when constructing the client.

Additionally, this crate relies on the [tokio](https://tokio.rs/) async-runtime to make all the API operations non-blocking.

This is a basic example of the creating the client and using the `generate` endpoint.

```rust
use cohere_rust::api::{
    generate::{GenerateRequest, ReturnLikelihoods},
    Truncate,
};
use cohere_rust::Cohere;

#[tokio::main]
async fn main() {
    // automatically reads API key from `COHERE_API_KEY`
    let co = Cohere::default();

    let request = GenerateRequest {
        max_tokens: Some(20),
        return_likelihoods: Some(ReturnLikelihoods::None),
        truncate: Some(Truncate::End),
        prompt: "Once upon a time in a magical land called",
        ..Default::default()
    };

    match co.generate(&request).await {
        Ok(r) => println!("Generate response: {:?}", r),
        Err(e) => {
            println!("Generate failed! {}", e)
        }
    }
}
```

Example usage of other endpoints can be found [here](https://github.com/walterbm/cohere-rust/blob/main/examples).

## Versioning

This SDK supports the latest API version. For more information, please refer to the [Versioning Docs](https://docs.cohere.ai/reference/versioning).

## Endpoints

For a full breakdown of endpoints and arguments, please consult the [Cohere Docs](https://docs.cohere.ai/).

| Cohere Endpoint  | Function             |
| ---------------- | -------------------- |
| /generate        | co.generate()        |
| /embed           | co.embed()           |
| /rerank          | co.rerank()          |
| /classify        | co.classify()        |
| /summarize       | co.summarize()       |
| /tokenize        | co.tokenize()        |
| /detokenize      | co.detokenize()      |
| /detect-language | co.detect_language() |
| /check-api-key   | co.check_api_key()   |

## Responses

All of the endpoint functions will return a Cohere object corresponding to the endpoint (e.g. for generation, it would be `GenerateResponse`). The names of these fields and a detailed breakdown of the response body can be found in the [Cohere Docs](https://docs.cohere.ai/).

## Errors

Unsuccessful API calls from the SDK will return an error. Please see the documentation's page on [errors](https://docs.cohere.ai/errors-reference) for more information about what the errors mean.
