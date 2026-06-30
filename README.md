# Payabli Rust Library

[![fern shield](https://img.shields.io/badge/%F0%9F%8C%BF-Built%20with%20Fern-brightgreen)](https://buildwithfern.com?utm_source=github&utm_medium=github&utm_campaign=readme&utm_source=https%3A%2F%2Fgithub.com%2Fpayabli%2Fsdk-rust)
[![crates.io shield](https://img.shields.io/crates/v/payabli_api)](https://crates.io/crates/payabli_api)

The Payabli Rust library provides convenient access to the Payabli APIs from Rust.

## Table of Contents

- [Documentation](#documentation)
- [Installation](#installation)
- [Reference](#reference)
- [Changelog](#changelog)
- [Getting Started](#getting-started)
- [Usage](#usage)
- [Environments](#environments)
- [Errors](#errors)
- [Request Types](#request-types)
- [Advanced](#advanced)
  - [Retries](#retries)
  - [Timeouts](#timeouts)
  - [Additional Headers](#additional-headers)
  - [Additional Query String Parameters](#additional-query-string-parameters)
- [Contributing](#contributing)

## Documentation

API reference documentation is available [here](https://docs.payabli.com).

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
payabli_api = "2.0.2"
```

Or install via cargo:

```sh
cargo add payabli_api
```

## Reference

A full reference for this library is available [here](https://github.com/payabli/sdk-rust/blob/HEAD/./reference.md).

## Changelog

The changelog for the official Payabli Rust SDK is available on the Payabli Docs site. See [Rust SDK Changelog](https://docs.payabli.com/changelog/rust-sdk) for more information.


## Getting Started

Visit the Payabli Docs site to get started with the official Payabli Rust SDK. See [Use the Rust SDK](https://docs.payabli.com/developers/platform-sdk-rust-guide) for more information.


## Usage

Instantiate and use the client with the following:

```rust
use payabli_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .money_in
        .getpaidv_2(
            &Getpaidv2Request {
                body: TransRequestBody {
                    account_id: None,
                    customer_data: Some(PayorDataRequest {
                        customer_id: Some(CustomerId(4440)),
                        ..Default::default()
                    }),
                    entry_point: Some(Entrypointfield("8cfec329267".to_string())),
                    invoice_data: None,
                    ipaddress: Some(IpAddress("255.255.255.255".to_string())),
                    order_description: None,
                    order_id: None,
                    payment_details: PaymentDetail {
                        service_fee: Some(0.0),
                        total_amount: 100.0,
                        ..Default::default()
                    },
                    payment_method: PaymentMethod::PayMethodCredit(PayMethodCredit {
                        cardcvv: Some(Cardcvv("999".to_string())),
                        cardexp: Cardexp("02/27".to_string()),
                        card_holder: Some(Cardholder("John Cassian".to_string())),
                        cardnumber: Cardnumber("4111111111111111".to_string()),
                        cardzip: Some(Cardzip("12345".to_string())),
                        initiator: Some(Initiator("payor".to_string())),
                        method: PayMethodCreditMethod::Card,
                        save_if_success: None,
                    }),
                    source: None,
                    subdomain: None,
                    subscription_id: None,
                },
                ach_validation: None,
                force_customer_creation: None,
            },
            None,
        )
        .await;
}
```

## Environments

This SDK allows you to configure different environments for API requests.

```rust
use payabli_api::prelude::{*};

let config = ClientConfig {
    base_url: Environment::Sandbox.url().to_string(),
    ..Default::default()
};
let client = Client::new(config).expect("Failed to build client");
```

## Errors

When the API returns a non-success status code (4xx or 5xx response), an error will be returned.

```rust
match client.money_in.getpaidv_2(None)?.await {
    Ok(response) => {
        println!("Success: {:?}", response);
    },
    Err(ApiError::HTTP { status, message }) => {
        println!("API Error {}: {:?}", status, message);
    },
    Err(e) => {
        println!("Other error: {:?}", e);
    }
}
```

## Request Types

The SDK exports all request types as Rust structs. Simply import them from the crate to access them:

```rust
use payabli_api::prelude::{*};

let request = CheckCaptureRequestBody {
    ...
};
```

## Advanced

### Retries

The SDK is instrumented with automatic retries with exponential backoff. A request will be retried as long
as the request is deemed retryable and the number of retry attempts has not grown larger than the configured
retry limit (default: 2).

A request is deemed retryable when any of the following HTTP status codes is returned:

- [408](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/408) (Timeout)
- [429](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/429) (Too Many Requests)
- [5XX](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#server_error_responses) (Internal Server Error)

The `retryStatusCodes` configuration controls which [5XX](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status#server_error_responses) status codes are retried:

- `legacy` (default): Retries `408`, `429`, and all `>= 500`
- `recommended`: Retries `408`, `429`, `502`, `503`, `504` only (excludes `500 Internal Server Error` to avoid retrying non-idempotent failures)

Use the `max_retries` method to configure this behavior.

```rust
let response = client.money_in.getpaidv_2(
    Some(RequestOptions::new().max_retries(3))
)?.await;
```

### Timeouts

The SDK defaults to a 30 second timeout. Use the `timeout` method to configure this behavior.

```rust
let response = client.money_in.getpaidv_2(
    Some(RequestOptions::new().timeout_seconds(30))
)?.await;
```

### Additional Headers

You can add custom headers to requests using `RequestOptions`.

```rust
let response = client.money_in.getpaidv_2(
    Some(
        RequestOptions::new()
            .additional_header("X-Custom-Header", "custom-value")
            .additional_header("X-Another-Header", "another-value")
    )
)?
.await;
```

### Additional Query String Parameters

You can add custom query parameters to requests using `RequestOptions`.

```rust
let response = client.money_in.getpaidv_2(
    Some(
        RequestOptions::new()
            .additional_query_param("filter", "active")
            .additional_query_param("sort", "desc")
    )
)?
.await;
```

## Contributing

While we value open-source contributions to this SDK, this library is generated programmatically.
Additions made directly to this library would have to be moved over to our generation code,
otherwise they would be overwritten upon the next generated release. Feel free to open a PR as
a proof of concept, but know that we will not be able to merge it as-is. We suggest opening
an issue first to discuss with us!

On the other hand, contributions to the README are always very welcome!
