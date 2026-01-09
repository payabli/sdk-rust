# Payabli Rust Library

[![fern shield](https://img.shields.io/badge/%F0%9F%8C%BF-Built%20with%20Fern-brightgreen)](https://buildwithfern.com?utm_source=github&utm_medium=github&utm_campaign=readme&utm_source=https%3A%2F%2Fgithub.com%2Fpayabli%2Fsdk-rust)
[![crates.io shield](https://img.shields.io/crates/v/payabli_api)](https://crates.io/crates/payabli_api)

The Payabli Rust library provides convenient access to the Payabli APIs from Rust.

## Table of Contents

- [Documentation](#documentation)
- [Installation](#installation)
- [Reference](#reference)
- [Usage](#usage)
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
payabli_api = "0.0.606"
```

Or install via cargo:

```sh
cargo add payabli_api
```

## Reference

A full reference for this library is available [here](https://github.com/payabli/sdk-rust/blob/HEAD/./reference.md).

## Usage

Instantiate and use the client with the following:

```rust
use payabli_api::prelude::*;
use payabli_api::{
    Accountid, AchHolder, AchHolderType, AchSecCode, AchValidation, Achaccount, Achaccounttype,
    Achrouting, AdditionalData, AdditionalDataString, Attachments, BillData, BillDataPaymentTerms,
    BillItem, BillingAddressAddtlNullable, BillingAddressNullable, BillingCityNullable,
    BillingCountryNullable, BillingStateNullable, BillingZip, Cardcvv, Cardexp, Cardholder,
    Cardnumber, Cardzip, Cash, Check, CustomerId, CustomerNumberNullable, Datenullable, Device,
    Discount, DutyAmount, Email, Entrypointfield, FileContent, FileContentFtype,
    ForceCustomerCreation, FreightAmount, Frequency, IdempotencyKey, Identifierfields, Initiator,
    InvoiceAmount, InvoiceNumber, InvoiceType, Invoicestatus, IpAddress, ItemCommodityCode,
    ItemDescription, ItemProductCode, ItemProductName, ItemUnitofMeasure, Methodall, OrderId,
    Orderdescription, PayMethodAch, PayMethodBodyAllFields, PayMethodCloud, PayMethodCredit,
    PayMethodStoredMethod, PayMethodStoredMethodMethod, PaymentCategories, PaymentDetail,
    PaymentMethod, PayorDataRequest, PhoneNumber, PurchaseOrder, SaveIfSuccess, ShippingFromZip,
    Shippingaddress, Shippingaddressadditional, Shippingcity, Shippingcountry, Shippingstate,
    Shippingzip, Source, SplitFunding, SplitFundingContent, StoredMethodUsageType, Storedmethodid,
    Subdomain, Subscriptionid, SummaryCommodityCode, Tax, TermsConditions, TransRequestBody,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .money_in
        .getpaid(
            &GetpaidRequest {
                body: TransRequestBody {
                    account_id: None,
                    customer_data: Some(PayorDataRequest {
                        additional_data: None,
                        billing_address_1: None,
                        billing_address_2: None,
                        billing_city: None,
                        billing_country: None,
                        billing_email: None,
                        billing_phone: None,
                        billing_state: None,
                        billing_zip: None,
                        company: None,
                        customer_id: Some(CustomerId(4440)),
                        customer_number: None,
                        first_name: None,
                        identifier_fields: None,
                        last_name: None,
                        shipping_address_1: None,
                        shipping_address_2: None,
                        shipping_city: None,
                        shipping_country: None,
                        shipping_state: None,
                        shipping_zip: None,
                    }),
                    entry_point: Some(Entrypointfield("f743aed24a".to_string())),
                    invoice_data: None,
                    ipaddress: Some(IpAddress("255.255.255.255".to_string())),
                    order_description: None,
                    order_id: None,
                    payment_details: PaymentDetail {
                        categories: None,
                        check_image: None,
                        check_number: None,
                        currency: None,
                        service_fee: Some(0.0),
                        split_funding: None,
                        total_amount: 100.0,
                    },
                    payment_method: PaymentMethod::PayMethodCredit(PayMethodCredit {
                        cardcvv: Some(Cardcvv("999".to_string())),
                        cardexp: Cardexp("02/27".to_string()),
                        card_holder: Some(Cardholder("John Cassian".to_string())),
                        cardnumber: Cardnumber("4111111111111111".to_string()),
                        cardzip: Some(Cardzip("12345".to_string())),
                        initiator: Some(Initiator("payor".to_string())),
                        method: "card".to_string(),
                        save_if_success: None,
                    }),
                    source: None,
                    subdomain: None,
                    subscription_id: None,
                },
                ach_validation: None,
                force_customer_creation: None,
                include_details: None,
            },
            None,
        )
        .await;
}
```

## Errors

When the API returns a non-success status code (4xx or 5xx response), an error will be returned.

```rust
match client.money_in.getpaid(None)?.await {
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

let request = RequestAppByAuth {
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
- [5XX](https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/500) (Internal Server Errors)

Use the `max_retries` method to configure this behavior.

```rust
let response = client.money_in.getpaid(
    Some(RequestOptions::new().max_retries(3))
)?.await;
```

### Timeouts

The SDK defaults to a 30 second timeout. Use the `timeout` method to configure this behavior.

```rust
let response = client.money_in.getpaid(
    Some(RequestOptions::new().timeout_seconds(30))
)?.await;
```

### Additional Headers

You can add custom headers to requests using `RequestOptions`.

```rust
let response = client.money_in.getpaid(
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
let response = client.money_in.getpaid(
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