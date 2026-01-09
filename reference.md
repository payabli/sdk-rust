# Reference
## Bill
<details><summary><code>client.bill.<a href="/src/api/resources/bill/client.rs">add_bill</a>(entry: String, request: BillOutData) -> Result<BillResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a bill in an entrypoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    AccountingField, AdditionalData, AdditionalDataString, AddressAddtlNullable, AddressNullable,
    Attachments, BankAccountHolderName, BankAccountHolderType, BankName, BillItem, BillOutData,
    BillOutDataScheduledOptions, BillingData, Billitems, Billstatus, Comments, Contacts,
    ContactsField, Datenullable, Email, FileContent, FileContentFtype, Frequency, IdempotencyKey,
    ItemCommodityCode, ItemDescription, ItemProductCode, ItemProductName, ItemUnitofMeasure,
    LocationCode, Mcc, PayeeName, RemitEmail, Remitaddress1, Remitaddress2, Remitcity,
    Remitcountry, Remitstate, Remitzip, RoutingAccount, Terms, TypeAccount, VendorData, VendorEin,
    VendorName1, VendorName2, VendorNumber, VendorPaymentMethodString, VendorPhone, Vendorstatus,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bill
        .add_bill(
            &"8cfec329267".to_string(),
            &BillOutData {
                accounting_field_1: Some(AccountingField("MyInternalId".to_string())),
                accounting_field_2: None,
                additional_data: None,
                attachments: Some(Attachments(Some(vec![FileContent {
                    f_content: None,
                    filename: Some("my-doc.pdf".to_string()),
                    ftype: Some(FileContentFtype::Pdf),
                    furl: Some("https://mysite.com/my-doc.pdf".to_string()),
                }]))),
                bill_date: Some(Datenullable(Some(
                    NaiveDate::parse_from_str("2024-07-01", "%Y-%m-%d").unwrap(),
                ))),
                bill_items: Some(Billitems(Some(vec![BillItem {
                    item_categories: Some(vec![Some("deposits".to_string())]),
                    item_commodity_code: Some(ItemCommodityCode("010".to_string())),
                    item_cost: 5.0,
                    item_description: Some(ItemDescription("Deposit for materials".to_string())),
                    item_mode: Some(0),
                    item_product_code: Some(ItemProductCode("M-DEPOSIT".to_string())),
                    item_product_name: Some(ItemProductName("Materials deposit".to_string())),
                    item_qty: Some(1),
                    item_tax_amount: Some(7.0),
                    item_tax_rate: Some(0.075),
                    item_total_amount: Some(123.0),
                    item_unit_of_measure: Some(ItemUnitofMeasure("SqFt".to_string())),
                }]))),
                bill_number: Some("ABC-123".to_string()),
                comments: Some(Comments("Deposit for materials".to_string())),
                discount: None,
                due_date: Some(Datenullable(Some(
                    NaiveDate::parse_from_str("2024-07-01", "%Y-%m-%d").unwrap(),
                ))),
                end_date: Some(Datenullable(Some(
                    NaiveDate::parse_from_str("2024-07-01", "%Y-%m-%d").unwrap(),
                ))),
                frequency: Some(Frequency::Monthly),
                lot_number: None,
                mode: Some(0),
                net_amount: Some(3762.87),
                scheduled_options: None,
                status: Some(Billstatus(-99)),
                terms: Some(Terms("NET30".to_string())),
                total_amount: None,
                vendor: Some(VendorData {
                    vendor_number: Some(VendorNumber("1234-A".to_string())),
                    additional_data: None,
                    address_1: None,
                    address_2: None,
                    billing_data: None,
                    city: None,
                    contacts: None,
                    country: None,
                    custom_field_1: None,
                    custom_field_2: None,
                    customer_vendor_account: None,
                    ein: None,
                    email: None,
                    internal_reference_id: None,
                    location_code: None,
                    mcc: None,
                    name_1: None,
                    name_2: None,
                    payee_name_1: None,
                    payee_name_2: None,
                    payment_method: None,
                    phone: None,
                    remit_address_1: None,
                    remit_address_2: None,
                    remit_city: None,
                    remit_country: None,
                    remit_email: None,
                    remit_state: None,
                    remit_zip: None,
                    state: None,
                    vendor_status: None,
                    zip: None,
                }),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bill.<a href="/src/api/resources/bill/client.rs">delete_attached_from_bill</a>(id_bill: i64, filename: String, return_object: Option<Option<bool>>) -> Result<BillResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a file attached to a bill.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .bill
        .delete_attached_from_bill(
            285,
            &"0_Bill.pdf".to_string(),
            &DeleteAttachedFromBillQueryRequest {
                return_object: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id_bill:** `i64` — Payabli ID for the bill. Get this ID by querying `/api/Query/bills/` for the entrypoint or the organization.
    
</dd>
</dl>

<dl>
<dd>

**filename:** `String` 

The filename in Payabli. Filename is `zipName` in response to a
request to `/api/Invoice/{idInvoice}`. Here, the filename is
`0_Bill.pdf`. 

```json
  "DocumentsRef": {
    "zipfile": "inva_269.zip",
    "filelist": [
      {
        "originalName": "Bill.pdf",
        "zipName": "0_Bill.pdf",
        "descriptor": null
      }
    ]
  }
  ```
    
</dd>
</dl>

<dl>
<dd>

**return_object:** `Option<bool>` — When `true`, the request returns the file content as a Base64-encoded string.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bill.<a href="/src/api/resources/bill/client.rs">delete_bill</a>(id_bill: i64) -> Result<BillResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes a bill by ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.bill.delete_bill(285, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id_bill:** `i64` — Payabli ID for the bill. Get this ID by querying `/api/Query/bills/` for the entrypoint or the organization.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bill.<a href="/src/api/resources/bill/client.rs">edit_bill</a>(id_bill: i64, request: BillOutData) -> Result<EditBillResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates a bill by ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    AccountingField, AdditionalData, AdditionalDataString, AddressAddtlNullable, AddressNullable,
    Attachments, BankAccountHolderName, BankAccountHolderType, BankName, BillItem, BillOutData,
    BillOutDataScheduledOptions, BillingData, Billitems, Billstatus, Comments, Contacts,
    ContactsField, Datenullable, Email, FileContent, FileContentFtype, Frequency,
    ItemCommodityCode, ItemDescription, ItemProductCode, ItemProductName, ItemUnitofMeasure,
    LocationCode, Mcc, PayeeName, RemitEmail, Remitaddress1, Remitaddress2, Remitcity,
    Remitcountry, Remitstate, Remitzip, RoutingAccount, Terms, TypeAccount, VendorData, VendorEin,
    VendorName1, VendorName2, VendorNumber, VendorPaymentMethodString, VendorPhone, Vendorstatus,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bill
        .edit_bill(
            285,
            &BillOutData {
                accounting_field_1: None,
                accounting_field_2: None,
                additional_data: None,
                attachments: None,
                bill_date: Some(Datenullable(Some(
                    NaiveDate::parse_from_str("2025-07-01", "%Y-%m-%d").unwrap(),
                ))),
                bill_items: None,
                bill_number: None,
                comments: None,
                discount: None,
                due_date: None,
                end_date: None,
                frequency: None,
                lot_number: None,
                mode: None,
                net_amount: Some(3762.87),
                scheduled_options: None,
                status: None,
                terms: None,
                total_amount: None,
                vendor: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id_bill:** `i64` — Payabli ID for the bill. Get this ID by querying `/api/Query/bills/` for the entrypoint or the organization.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bill.<a href="/src/api/resources/bill/client.rs">get_attached_from_bill</a>(id_bill: i64, filename: String, return_object: Option<Option<bool>>) -> Result<FileContent, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a file attached to a bill, either as a binary file or as a Base64-encoded string.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .bill
        .get_attached_from_bill(
            285,
            &"0_Bill.pdf".to_string(),
            &GetAttachedFromBillQueryRequest {
                return_object: Some(true),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id_bill:** `i64` — Payabli ID for the bill. Get this ID by querying `/api/Query/bills/` for the entrypoint or the organization.
    
</dd>
</dl>

<dl>
<dd>

**filename:** `String` 

The filename in Payabli. Filename is `zipName` in response to a request to `/api/Invoice/{idInvoice}`. Here, the filename is `0_Bill.pdf``. 
"DocumentsRef": {
  "zipfile": "inva_269.zip",
  "filelist": [
    {
      "originalName": "Bill.pdf",
      "zipName": "0_Bill.pdf",
      "descriptor": null
    }
  ]
}
    
</dd>
</dl>

<dl>
<dd>

**return_object:** `Option<bool>` — When `true`, the request returns the file content as a Base64-encoded string.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bill.<a href="/src/api/resources/bill/client.rs">get_bill</a>(id_bill: i64) -> Result<GetBillResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a bill by ID from an entrypoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.bill.get_bill(285, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id_bill:** `i64` — Payabli ID for the bill. Get this ID by querying `/api/Query/bills/` for the entrypoint or the organization.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bill.<a href="/src/api/resources/bill/client.rs">list_bills</a>(entry: String, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<BillQueryResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a list of bills for an entrypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bill
        .list_bills(
            &"8cfec329267".to_string(),
            &ListBillsQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set. 
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response isn't filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>
See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

List of field names accepted:
- `frequency` (`in`, `nin`, `ne`, `eq`)
- `method` (`in`, `nin`, `eq`, `ne`)
- `event` (`in`, `nin`, `eq`, `ne`)
- `target` (`ct`, `nct`, `eq`, `ne`)
- `status` (`eq`, `ne`)
- `approvalUserId` (`eq`, `ne`)
- `parentOrgId` (`ne`, `eq`, `nin`, `in`)
- `approvalUserEmail` (`eq`, `ne`)
- `scheduleId` (`ne`, `eq`)

List of comparison accepted - enclosed between parentheses:
- `eq` or empty => equal
- `gt` => greater than
- `ge` => greater or equal
- `lt` => less than
- `le` => less or equal
- `ne` => not equal
- `ct` => contains
- `nct` => not contains
- `in` => inside array
- `nin` => not inside array

List of parameters accepted:
- `limitRecord` : max number of records for query (default="20", "0" or negative value for all)
- `fromRecord` : initial record in query
Example: `totalAmount(gt)=20` returns all records with a `totalAmount` that's greater than 20.00
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bill.<a href="/src/api/resources/bill/client.rs">list_bills_org</a>(org_id: i64, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<BillQueryResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a list of bills for an organization. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bill
        .list_bills_org(
            123,
            &ListBillsOrgQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response isn't filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>
See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

List of field names accepted:
- `frequency` (in, nin, ne, eq)
- `method` (in, nin, eq, ne)
- `event` (in, nin, eq, ne)
- `target` (ct, nct, eq, ne)
- `status` (eq, ne)
- `parentOrgId` (ne, eq, nin, in)
- `approvalUserId` (eq, ne)
- `approvalUserEmail` (eq, ne)

List of comparison accepted - enclosed between parentheses:
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array
- nin => not inside array

List of parameters accepted:
- limitRecord : max number of records for query (default="20", "0" or negative value for all)
- fromRecord : initial record in query

Example: totalAmount(gt)=20 return all records with totalAmount greater than 20.00
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bill.<a href="/src/api/resources/bill/client.rs">modify_approval_bill</a>(id_bill: i64, request: Vec<String>) -> Result<ModifyApprovalBillResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Modify the list of users the bill is sent to for approval.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .bill
        .modify_approval_bill(285, &vec!["string".to_string()], None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id_bill:** `i64` — Payabli ID for the bill. Get this ID by querying `/api/Query/bills/` for the entrypoint or the organization.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bill.<a href="/src/api/resources/bill/client.rs">send_to_approval_bill</a>(id_bill: i64, request: Vec<String>, autocreate_user: Option<Option<bool>>) -> Result<BillResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Send a bill to a user or list of users to approve.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::IdempotencyKey;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .bill
        .send_to_approval_bill(
            285,
            &SendToApprovalBillRequest {
                body: vec!["string".to_string()],
                autocreate_user: None,
            },
            Some(
                RequestOptions::new()
                    .additional_header("idempotencyKey", "6B29FC40-CA47-1067-B31D-00DD010662DA"),
            ),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id_bill:** `i64` — Payabli ID for the bill. Get this ID by querying `/api/Query/bills/` for the entrypoint or the organization.
    
</dd>
</dl>

<dl>
<dd>

**autocreate_user:** `Option<bool>` — Automatically create the target user for approval if they don't exist.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.bill.<a href="/src/api/resources/bill/client.rs">set_approved_bill</a>(id_bill: i64, approved: String, email: Option<Option<String>>) -> Result<SetApprovedBillResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Approve or disapprove a bill by ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .bill
        .set_approved_bill(
            285,
            &"true".to_string(),
            &SetApprovedBillQueryRequest { email: None },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id_bill:** `i64` — Payabli ID for the bill. Get this ID by querying `/api/Query/bills/` for the entrypoint or the organization.
    
</dd>
</dl>

<dl>
<dd>

**approved:** `String` — String representing the approved status. Accepted values: 'true' or 'false'.
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<String>` — Email or username of user modifying approval status.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Boarding
<details><summary><code>client.boarding.<a href="/src/api/resources/boarding/client.rs">add_application</a>(request: AddApplicationRequest) -> Result<PayabliApiResponse00Responsedatanonobject, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a boarding application in an organization. This endpoint requires an application API token.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    AccountNumber, AchSetup, AddApplicationRequest, AdditionalDataString, Annualrevenue,
    ApplicationData, ApplicationDataManaged, ApplicationDataManagedContactsItem,
    ApplicationDataManagedOwnershipItem, ApplicationDataOdp, ApplicationDataOdpContactsItem,
    ApplicationDataOdpOwnershipItem, ApplicationDataPayIn, ApplicationDataPayInBankData,
    ApplicationDataPayInContactsItem, ApplicationDataPayInOwnershipItem,
    ApplicationDataPayInServices, ApplicationDataPayInServicesAch,
    ApplicationDataPayInServicesCard, Attachments, AttestationDate, Avgmonthly, Baddress1,
    Baddress2, Bank, BankAccountFunction, BankAccountHolderName, BankAccountHolderType, BankData,
    BankName, BankNickname, Bcity, Bcountry, Binperson, Binphone, Binweb, BoardingAverageBillSize,
    BoardingAvgMonthlyBill, BoardingBusinessFax, BoardingBusinessPhone, Bstate, Bsummary,
    Busstartdate, Bzip, CardSetup, Contacts, ContactsField, Dbaname, Ein, Email,
    ExternalPaypointId, FaxNumber, FileContent, FileContentFtype, Highticketamt, Legalname,
    License, Licensestate, Maddress, Maddress1, Mcc, Mcity, Mcountry, Mstate, Mzip, OdpSetup,
    OdpSetupProcessingRegion, OnCreate, Orgid, OwnType, Owners, Ownership,
    PayoutAverageMonthlyVolume, PayoutAverageTicketLimit, PayoutCreditLimit,
    PayoutHighTicketAmount, PciAttestation, PhoneNumber, RecipientEmailNotification, RepCode,
    RepName, RepOffice, Resumable, RoutingAccount, Services, SignDate, SignedDocumentReference,
    SignerAcceptance, SignerAddress1, SignerCity, SignerCountry, SignerDataRequest, SignerDob,
    SignerName, SignerPhone, SignerSsn, SignerState, SignerZip, Signeraddress, Taxfillname,
    TemplateId, Ticketamt, TypeAccount, Website, Whencharged, Whendelivered, Whenprovided,
    Whenrefunded,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.boarding.add_application(&AddApplicationRequest::ApplicationDataPayIn(ApplicationDataPayIn {
        services: ApplicationDataPayInServices {
            ach: ApplicationDataPayInServicesAch {
                ach_setup_fields: AchSetup {
                    accept_ccd: None,
                    accept_ppd: None,
                    accept_web: None
                }
            },
            card: ApplicationDataPayInServicesCard {
                card_setup_fields: CardSetup {
                    accept_amex: Some(true),
                    accept_discover: Some(true),
                    accept_mastercard: Some(true),
                    accept_visa: Some(true)
                }
            },
            odp: None
        },
        annual_revenue: Some(Annualrevenue(Some(1000.0))),
        average_bill_size: Some(BoardingAverageBillSize("500".to_string())),
        average_monthly_bill: Some(BoardingAvgMonthlyBill("5650".to_string())),
        avgmonthly: Some(Avgmonthly(Some(1000.0))),
        baddress: Some(Baddress1("123 Walnut Street".to_string())),
        baddress_1: Some(Baddress2("Suite 103".to_string())),
        bank_data: ApplicationDataPayInBankData {
            bank_fields: Bank {
                id: None,
                account_id: None,
                nickname: None,
                bank_name: None,
                routing_account: None,
                account_number: None,
                type_account: None,
                bank_account_holder_name: None,
                bank_account_holder_type: None,
                bank_account_function: None,
                verified: None,
                status: None,
                services: None
            }
        },
        bcity: Some(Bcity("New Vegas".to_string())),
        bcountry: Some(Bcountry("US".to_string())),
        binperson: Some(Binperson(60)),
        binphone: Some(Binphone(20)),
        binweb: Some(Binweb(20)),
        boarding_link_id: None,
        bstate: Some(Bstate("FL".to_string())),
        bsummary: Some(Bsummary("Brick and mortar store that sells office supplies".to_string())),
        btype: Some(OwnType::LimitedLiabilityCompany),
        bzip: Some(Bzip("33000".to_string())),
        contacts: Some(vec![ApplicationDataPayInContactsItem {
            contacts_fields: Contacts {
                contact_email: Some(Email("herman@hermanscoatings.com".to_string())),
                contact_name: Some("Herman Martinez".to_string()),
                contact_phone: Some("3055550000".to_string()),
                contact_title: Some("Owner".to_string()),
                additional_data: None
            }
        }]),
        credit_limit: Some("creditLimit".to_string()),
        dba_name: Some(Dbaname("Sunshine Gutters".to_string())),
        ein: Some(Ein("123456789".to_string())),
        externalpaypoint_id: None,
        faxnumber: Some(FaxNumber("1234567890".to_string())),
        highticketamt: Some(Highticketamt(Some(1000.0))),
        legal_name: Some(Legalname("Sunshine Services, LLC".to_string())),
        license: Some(License("2222222FFG".to_string())),
        licstate: Some(Licensestate("CA".to_string())),
        maddress: Some(Maddress("123 Walnut Street".to_string())),
        maddress_1: Some(Maddress1("STE 900".to_string())),
        mcc: Some(Mcc("7777".to_string())),
        mcity: Some(Mcity("Johnson City".to_string())),
        mcountry: Some(Mcountry("US".to_string())),
        mstate: Some(Mstate("TN".to_string())),
        mzip: Some(Mzip("37615".to_string())),
        org_id: Some(Orgid(123)),
        ownership: Some(vec![ApplicationDataPayInOwnershipItem {
            owners_fields: Owners {
                ownername: Some("John Smith".to_string()),
                ownertitle: Some("CEO".to_string()),
                ownerpercent: Some(100),
                ownerssn: Some("123456789".to_string()),
                ownerdob: Some("01/01/1990".to_string()),
                ownerphone_1: Some("555888111".to_string()),
                ownerphone_2: Some("555888111".to_string()),
                owneremail: Some(Email("test@email.com".to_string())),
                ownerdriver: Some("CA6677778".to_string()),
                oaddress: Some("33 North St".to_string()),
                ocity: Some("Any City".to_string()),
                ocountry: Some("US".to_string()),
                odriverstate: Some("CA".to_string()),
                ostate: Some("CA".to_string()),
                ozip: Some("55555".to_string()),
                additional_data: None
            }
        }]),
        phonenumber: PhoneNumber("1234567890".to_string()),
        processing_region: "US".to_string(),
        recipient_email: Some(Email("josephray@example.com".to_string())),
        recipient_email_notification: Some(RecipientEmailNotification(true)),
        resumable: Some(Resumable(true)),
        signer: SignerDataRequest {
            name: Some(SignerName("John Smith".to_string())),
            ssn: Some(SignerSsn("123456789".to_string())),
            dob: Some(SignerDob("01/01/1976".to_string())),
            phone: Some(SignerPhone("555888111".to_string())),
            email: Some(Email("test@email.com".to_string())),
            address: Some(Signeraddress("33 North St".to_string())),
            address_1: Some(SignerAddress1("STE 900".to_string())),
            city: Some(SignerCity("Bristol".to_string())),
            country: Some(SignerCountry("US".to_string())),
            state: Some(SignerState("TN".to_string())),
            zip: Some(SignerZip("55555".to_string())),
            acceptance: None,
            signed_document_reference: Some(SignedDocumentReference("https://example.com/signed-document.pdf".to_string())),
            pci_attestation: Some(PciAttestation(Some(true))),
            attestation_date: Some(AttestationDate("04/20/2025".to_string())),
            additional_data: Some(AdditionalDataString("{\"deviceId\":\"499585-389fj484-3jcj8hj3\",\"session\":\"fifji4-fiu443-fn4843\",\"timeWithCompany\":\"6 Years\"}".to_string())),
            sign_date: Some(SignDate("04/20/2025".to_string()))
        },
        startdate: Some(Busstartdate("01/01/1990".to_string())),
        tax_fill_name: Some(Taxfillname("Sunshine LLC".to_string())),
        template_id: Some(TemplateId(22)),
        ticketamt: Some(Ticketamt(Some(1000.0))),
        website: Some(Website("www.example.com".to_string())),
        when_charged: Whencharged::WhenServiceProvided,
        when_delivered: Whendelivered::Over30Days,
        when_provided: Whenprovided::ThirtyDaysOrLess,
        when_refunded: Whenrefunded::ThirtyDaysOrLess,
        additional_data: None,
        rep_code: None,
        rep_name: None,
        rep_office: None,
        on_create: None
    }), None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.boarding.<a href="/src/api/resources/boarding/client.rs">delete_application</a>(app_id: i64) -> Result<PayabliApiResponse00Responsedatanonobject, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes a boarding application by ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.boarding.delete_application(352, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**app_id:** `i64` — Boarding application ID. 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.boarding.<a href="/src/api/resources/boarding/client.rs">get_application</a>(app_id: i64) -> Result<ApplicationDetailsRecord, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details for a boarding application by ID. 
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.boarding.get_application(352, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**app_id:** `i64` — Boarding application ID.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.boarding.<a href="/src/api/resources/boarding/client.rs">get_application_by_auth</a>(x_id: String, request: RequestAppByAuth) -> Result<ApplicationQueryRecord, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Gets a boarding application by authentication information. This endpoint requires an `application` API token. 
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::Email;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .boarding
        .get_application_by_auth(
            &"17E".to_string(),
            &RequestAppByAuth {
                email: Some(Email("admin@email.com".to_string())),
                reference_id: Some("n6UCd1f1ygG7".to_string()),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**x_id:** `String` — The application ID in Hex format. Find this at the end of the boarding link URL returned in a call to api/Boarding/applink/{appId}/{mail2}. For example in:  `https://boarding-sandbox.payabli.com/boarding/externalapp/load/17E`, the xId is `17E`. 
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<Email>` — The email address the applicant used to save the application.
    
</dd>
</dl>

<dl>
<dd>

**reference_id:** `Option<String>` — The referenceId is sent to the applicant via email when they save the application.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.boarding.<a href="/src/api/resources/boarding/client.rs">get_by_id_link_application</a>(boarding_link_id: i64) -> Result<BoardingLinkQueryRecord, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves details for a boarding link, by ID. 
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.boarding.get_by_id_link_application(91, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**boarding_link_id:** `i64` — The boarding link ID. You can find this at the end of the boarding link reference name. For example `https://boarding.payabli.com/boarding/app/myorgaccountname-00091`. The ID is `91`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.boarding.<a href="/src/api/resources/boarding/client.rs">get_by_template_id_link_application</a>(template_id: f64) -> Result<BoardingLinkQueryRecord, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get details for a boarding link using the boarding template ID. This endpoint requires an application API token.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .boarding
        .get_by_template_id_link_application(80.0, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**template_id:** `f64` — The boarding template ID. You can find this at the end of the boarding template URL in PartnerHub. Example: `https://partner-sandbox.payabli.com/myorganization/boarding/edittemplate/80`. Here, the template ID is `80`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.boarding.<a href="/src/api/resources/boarding/client.rs">get_external_application</a>(app_id: i64, mail_2: String, send_email: Option<Option<bool>>) -> Result<PayabliApiResponse00, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a link and the verification code used to log into an existing boarding application. You can also use this endpoint to send a link and referenceId for an existing boarding application to an email address. The recipient can use the referenceId and email address to access and edit the application.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .boarding
        .get_external_application(
            352,
            &"mail2".to_string(),
            &GetExternalApplicationQueryRequest { send_email: None },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**app_id:** `i64` — Boarding application ID. 
    
</dd>
</dl>

<dl>
<dd>

**mail_2:** `String` — Email address used to access the application. If `sendEmail` parameter is true, a link to the application is sent to this email address.
    
</dd>
</dl>

<dl>
<dd>

**send_email:** `Option<bool>` — If `true`, sends an email that includes the link to the application to the `mail2` address. Defaults to `false`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.boarding.<a href="/src/api/resources/boarding/client.rs">get_link_application</a>(boarding_link_reference: String) -> Result<BoardingLinkQueryRecord, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the details for a boarding link, by reference name. This endpoint requires an application API token.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .boarding
        .get_link_application(&"myorgaccountname-00091".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**boarding_link_reference:** `String` — The boarding link reference name. You can find this at the end of the boarding link URL. For example `https://boarding.payabli.com/boarding/app/myorgaccountname-00091`
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.boarding.<a href="/src/api/resources/boarding/client.rs">list_applications</a>(org_id: i64, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryBoardingAppsListResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a list of boarding applications for an organization. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .boarding
        .list_applications(
            123,
            &ListApplicationsQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query 

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

List of field names accepted:
- `createdAt` (gt, ge, lt, le, eq, ne)
- `startDate` (gt, ge, lt, le, eq, ne)
- `dbaname` (ct, nct)
- `legalname` (ct, nct)
- `ein` (ct, nct)
- `address` (ct, nct)
- `city` (ct, nct)
- `state` (ct, nct)
- `phone` (ct, nct)
- `mcc` (ct, nct)
- `owntype` (ct, nct)
- `ownerName` (ct, nct)
- `contactName` (ct, nct)
- `status` (in, nin, eq,ne)
- `orgParentname` (ct, nct)
- `externalpaypointID` (ct, nct, eq, ne)
- `repCode` (ct, nct, eq, ne)
- `repName` (ct, nct, eq, ne)
- `repOffice` (ct, nct, eq, ne)
List of comparison accepted - enclosed between parentheses:
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array
- nin => not inside array
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.boarding.<a href="/src/api/resources/boarding/client.rs">list_boarding_links</a>(org_id: i64, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryBoardingLinksResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Return a list of boarding links for an organization. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .boarding
        .list_boarding_links(
            123,
            &ListBoardingLinksQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query 

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

List of field names accepted:
- `lastUpdated` (gt, ge, lt, le, eq, ne)
- `templateName` (ct, nct)
- `referenceName` (ct, nct)
- `acceptRegister` (eq, ne)
- `acceptAuth` (eq, ne)
- `templateCode` (ct, nct)
- `templateId` (eq, ne)
- `orgParentname` (ct, nct)

List of comparison accepted - enclosed between parentheses:
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than 
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array
- nin => not inside array

List of parameters accepted:
- limitRecord : max number of records for query (default="20", "0" or negative value for all)
- fromRecord : initial record in query

Example: templateName(ct)=hoa return all records with template title containing "hoa"
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.boarding.<a href="/src/api/resources/boarding/client.rs">update_application</a>(app_id: i64, request: ApplicationData) -> Result<PayabliApiResponse00Responsedatanonobject, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates a boarding application by ID. This endpoint requires an application API token.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    AccountNumber, AchSetup, AdditionalDataString, Annualrevenue, ApplicationData, Attachments,
    AttestationDate, Avgmonthly, Baddress1, Baddress2, Bank, BankAccountFunction,
    BankAccountHolderName, BankAccountHolderType, BankName, BankNickname, Bcity, Bcountry,
    Binperson, Binphone, Binweb, BoardingBusinessFax, BoardingBusinessPhone, Bstate, Bsummary,
    Busstartdate, Bzip, CardSetup, Contacts, ContactsField, Dbaname, Ein, Email,
    ExternalPaypointId, FileContent, FileContentFtype, Highticketamt, Legalname, License,
    Licensestate, Maddress, Maddress1, Mcc, Mcity, Mcountry, Mstate, Mzip, OdpSetup,
    OdpSetupProcessingRegion, OnCreate, Orgid, OwnType, Owners, Ownership,
    PayoutAverageMonthlyVolume, PayoutAverageTicketLimit, PayoutCreditLimit,
    PayoutHighTicketAmount, PciAttestation, RecipientEmailNotification, RepCode, RepName,
    RepOffice, Resumable, RoutingAccount, Services, SignDate, SignedDocumentReference,
    SignerAcceptance, SignerAddress1, SignerCity, SignerCountry, SignerDataRequest, SignerDob,
    SignerName, SignerPhone, SignerSsn, SignerState, SignerZip, Signeraddress, Taxfillname,
    TemplateId, Ticketamt, TypeAccount, Website, Whencharged, Whendelivered, Whenprovided,
    Whenrefunded,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .boarding
        .update_application(
            352,
            &ApplicationData {
                services: None,
                annual_revenue: None,
                attachments: None,
                avgmonthly: None,
                baddress: None,
                baddress_1: None,
                bank_data: None,
                bcity: None,
                bcountry: None,
                binperson: None,
                binphone: None,
                binweb: None,
                bstate: None,
                bsummary: None,
                btype: None,
                bzip: None,
                contacts: None,
                dbaname: None,
                ein: None,
                external_paypoint_id: None,
                faxnumber: None,
                highticketamt: None,
                legalname: None,
                license: None,
                licstate: None,
                maddress: None,
                maddress_1: None,
                mcc: None,
                mcity: None,
                mcountry: None,
                mstate: None,
                mzip: None,
                org_id: None,
                ownership: None,
                payout_average_monthly_volume: None,
                payout_average_ticket_limit: None,
                payout_credit_limit: None,
                payout_high_ticket_amount: None,
                phonenumber: None,
                recipient_email: None,
                recipient_email_notification: None,
                resumable: None,
                signer: None,
                startdate: None,
                taxfillname: None,
                template_id: None,
                ticketamt: None,
                website: None,
                when_charged: None,
                when_delivered: None,
                when_provided: None,
                when_refunded: None,
                rep_code: None,
                rep_name: None,
                rep_office: None,
                on_create: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**app_id:** `i64` — Boarding application ID. 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## ChargeBacks
<details><summary><code>client.charge_backs.<a href="/src/api/resources/charge_backs/client.rs">add_response</a>(id: String, request: ResponseChargeBack) -> Result<AddResponseResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add a response to a chargeback or ACH return.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{Attachments, Email, FileContent, FileContentFtype, IdempotencyKey};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .charge_backs
        .add_response(
            1000000,
            &ResponseChargeBack {
                attachments: None,
                contact_email: None,
                contact_name: None,
                notes: None,
            },
            Some(
                RequestOptions::new()
                    .additional_header("idempotencyKey", "6B29FC40-CA47-1067-B31D-00DD010662DA"),
            ),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — ID of the chargeback or return record.
    
</dd>
</dl>

<dl>
<dd>

**attachments:** `Option<Attachments>` — Array of attached files to response.
    
</dd>
</dl>

<dl>
<dd>

**contact_email:** `Option<Email>` — Email of response submitter.
    
</dd>
</dl>

<dl>
<dd>

**contact_name:** `Option<String>` — Name of response submitter
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<String>` — Response notes
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.charge_backs.<a href="/src/api/resources/charge_backs/client.rs">get_chargeback</a>(id: String) -> Result<ChargebackQueryRecords, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a chargeback record and its details.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.charge_backs.get_chargeback(1000000, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — ID of the chargeback or return record. This is returned as `chargebackId` in the [RecievedChargeback](/developers/developer-guides/webhook-payloads#receivedChargeback) and [ReceivedAchReturn](/developers/developer-guides/webhook-payloads#receivedachreturn) webhook notifications.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.charge_backs.<a href="/src/api/resources/charge_backs/client.rs">get_chargeback_attachment</a>(id: String, file_name: String) -> Result<String, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a chargeback attachment file by its file name.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .charge_backs
        .get_chargeback_attachment(1000000, &"fileName".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — The ID of chargeback or return record.
    
</dd>
</dl>

<dl>
<dd>

**file_name:** `String` — The chargeback attachment's file name.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## CheckCapture
<details><summary><code>client.check_capture.<a href="/src/api/resources/check_capture/client.rs">check_processing</a>(request: CheckCaptureRequestBody) -> Result<CheckCaptureResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Captures a check for Remote Deposit Capture (RDC) using the provided check images and details. This endpoint handles the OCR extraction of check data including MICR, routing number, account number, and amount. See the [RDC guide](/developers/developer-guides/pay-in-rdc) for more details.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::Entry;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .check_capture
        .check_processing(
            &CheckCaptureRequestBody {
                entry_point: Entry("47abcfea12".to_string()),
                front_image: "/9j/4AAQSkZJRgABAQEASABIAAD...".to_string(),
                rear_image: "/9j/4AAQSkZJRgABAQEASABIAAD...".to_string(),
                check_amount: 12550,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry_point:** `Entry` 
    
</dd>
</dl>

<dl>
<dd>

**front_image:** `String` — Base64-encoded front check image. Must be JPEG or PNG format and less than 1MB. Image must show the entire check clearly with no partial, blurry, or illegible portions.
    
</dd>
</dl>

<dl>
<dd>

**rear_image:** `String` — Base64-encoded rear check image. Must be JPEG or PNG format and less than 1MB. Image must show the entire check clearly with no partial, blurry, or illegible portions.
    
</dd>
</dl>

<dl>
<dd>

**check_amount:** `i64` — Check amount in cents (maximum 32-bit integer value).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Cloud
<details><summary><code>client.cloud.<a href="/src/api/resources/cloud/client.rs">add_device</a>(entry: String, request: DeviceEntry) -> Result<AddDeviceResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Register a cloud device to an entrypoint. See [Devices Quickstart](/developers/developer-guides/devices-quickstart#devices-quickstart) for a complete guide.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::IdempotencyKey;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .cloud
        .add_device(
            &"8cfec329267".to_string(),
            &DeviceEntry {
                description: Some("Front Desk POS".to_string()),
                registration_code: Some("YS7DS5".to_string()),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**description:** `Option<String>` — Description or name for the device. This can be anything, but Payabli recommends entering the name of the paypoint, or some other easy to identify descriptor. If you have several devices for one paypoint, you can give them descriptions like "Cashier 1" and "Cashier 2", or "Front Desk" and "Back Office"
    
</dd>
</dl>

<dl>
<dd>

**registration_code:** `Option<String>` 

The device registration code or serial number, depending on the model.

- Ingenico devices: This is the activation code that's displayed on the device screen during setup.

- PAX A920 device: This code is the serial number on the back of the device.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.cloud.<a href="/src/api/resources/cloud/client.rs">history_device</a>(entry: String, device_id: String) -> Result<CloudQueryApiResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve the registration history for a device. 
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .cloud
        .history_device(&"8cfec329267".to_string(), &"WXGDWB".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**device_id:** `String` — ID of the cloud device. 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.cloud.<a href="/src/api/resources/cloud/client.rs">list_device</a>(entry: String, force_refresh: Option<Option<bool>>) -> Result<CloudQueryApiResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a list of cloud devices registered to an entrypoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .cloud
        .list_device(
            &"8cfec329267".to_string(),
            &ListDeviceQueryRequest {
                force_refresh: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**force_refresh:** `Option<bool>` — When `true`, the request retrieves an updated list of devices from the processor instead of returning a cached list of devices.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.cloud.<a href="/src/api/resources/cloud/client.rs">remove_device</a>(entry: String, device_id: String) -> Result<RemoveDeviceResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Remove a cloud device from an entrypoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .cloud
        .remove_device(
            &"8cfec329267".to_string(),
            &"6c361c7d-674c-44cc-b790-382b75d1xxx".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**device_id:** `String` — ID of the cloud device. 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Customer
<details><summary><code>client.customer.<a href="/src/api/resources/customer/client.rs">add_customer</a>(entry: Entrypointfield, request: CustomerData, force_customer_creation: Option<Option<bool>>, replace_existing: Option<Option<i64>>) -> Result<PayabliApiResponseCustomerQuery, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a customer in an entrypoint. An identifier is required to create customer records. Change your identifier settings in Settings > Custom Fields in PartnerHub. 
If you don't include an identifier, the record is rejected.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    CreatedAt, CustomerData, CustomerNumberNullable, CustomerStatus, Email, Entrypointfield,
    IdempotencyKey, Identifierfields, Shippingaddress, Shippingaddressadditional, Shippingcity,
    Shippingcountry, Shippingstate, Shippingzip, Timezone,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .customer
        .add_customer(
            &Entrypointfield("8cfec329267".to_string()),
            &AddCustomerRequest {
                body: CustomerData {
                    customer_number: Some(CustomerNumberNullable("12356ACB".to_string())),
                    customer_username: None,
                    customer_psw: None,
                    customer_status: None,
                    company: None,
                    firstname: Some("Irene".to_string()),
                    lastname: Some("Canizales".to_string()),
                    phone: None,
                    email: Some(Email("irene@canizalesconcrete.com".to_string())),
                    address: None,
                    address_1: Some("123 Bishop's Trail".to_string()),
                    city: Some("Mountain City".to_string()),
                    state: Some("TN".to_string()),
                    zip: Some("37612".to_string()),
                    country: Some("US".to_string()),
                    shipping_address: None,
                    shipping_address_1: None,
                    shipping_city: None,
                    shipping_state: None,
                    shipping_zip: None,
                    shipping_country: None,
                    balance: None,
                    time_zone: Some(Timezone(-5)),
                    additional_fields: None,
                    identifier_fields: Some(Identifierfields(vec![Some("email".to_string())])),
                    created_at: None,
                },
                force_customer_creation: None,
                replace_existing: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `Entrypointfield` 
    
</dd>
</dl>

<dl>
<dd>

**force_customer_creation:** `Option<bool>` — When `true`, the request creates a new customer record, regardless of whether customer identifiers match an existing customer.
    
</dd>
</dl>

<dl>
<dd>

**replace_existing:** `Option<i64>` — Flag indicating to replace existing customer with a new record. Possible values: 0 (don't replace), 1 (replace). Default is `0`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.customer.<a href="/src/api/resources/customer/client.rs">delete_customer</a>(customer_id: i64) -> Result<PayabliApiResponse00Responsedatanonobject, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a customer record.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.customer.delete_customer(998, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**customer_id:** `i64` — Payabli-generated customer ID. Maps to "Customer ID" column in PartnerHub. 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.customer.<a href="/src/api/resources/customer/client.rs">get_customer</a>(customer_id: i64) -> Result<CustomerQueryRecords, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a customer's record and details.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.customer.get_customer(998, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**customer_id:** `i64` — Payabli-generated customer ID. Maps to "Customer ID" column in PartnerHub. 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.customer.<a href="/src/api/resources/customer/client.rs">link_customer_transaction</a>(customer_id: i64, trans_id: String) -> Result<PayabliApiResponse00Responsedatanonobject, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Links a customer to a transaction by ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .customer
        .link_customer_transaction(998, &"45-as456777hhhhhhhhhh77777777-324".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**customer_id:** `i64` — Payabli-generated customer ID. Maps to "Customer ID" column in PartnerHub. 
    
</dd>
</dl>

<dl>
<dd>

**trans_id:** `String` — ReferenceId for the transaction (PaymentId).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.customer.<a href="/src/api/resources/customer/client.rs">request_consent</a>(customer_id: i64) -> Result<PayabliApiResponse00Responsedatanonobject, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Sends the consent opt-in email to the customer email address in the customer record.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.customer.request_consent(998, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**customer_id:** `i64` — Payabli-generated customer ID. Maps to "Customer ID" column in PartnerHub. 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.customer.<a href="/src/api/resources/customer/client.rs">update_customer</a>(customer_id: i64, request: CustomerData) -> Result<PayabliApiResponse00Responsedatanonobject, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a customer record. Include only the fields you want to change.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    CreatedAt, CustomerData, CustomerNumberNullable, CustomerStatus, Email, Identifierfields,
    Shippingaddress, Shippingaddressadditional, Shippingcity, Shippingcountry, Shippingstate,
    Shippingzip, Timezone,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .customer
        .update_customer(
            998,
            &CustomerData {
                customer_number: None,
                customer_username: None,
                customer_psw: None,
                customer_status: None,
                company: None,
                firstname: Some("Irene".to_string()),
                lastname: Some("Canizales".to_string()),
                phone: None,
                email: None,
                address: None,
                address_1: Some("145 Bishop's Trail".to_string()),
                city: Some("Mountain City".to_string()),
                state: Some("TN".to_string()),
                zip: Some("37612".to_string()),
                country: Some("US".to_string()),
                shipping_address: None,
                shipping_address_1: None,
                shipping_city: None,
                shipping_state: None,
                shipping_zip: None,
                shipping_country: None,
                balance: None,
                time_zone: None,
                additional_fields: None,
                identifier_fields: None,
                created_at: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**customer_id:** `i64` — Payabli-generated customer ID. Maps to "Customer ID" column in PartnerHub. 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Export
<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_applications</a>(format: ExportFormat1, org_id: i64, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export a list of boarding applications for an organization. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat1;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .export
        .export_applications(
            &ExportFormat1::Csv,
            123,
            &ExportApplicationsQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `ExportFormat1` — Format for the export, either XLSX or CSV. 
    
</dd>
</dl>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query 

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help. 

List of field names accepted:
- `createdAt` (gt, ge, lt, le, eq, ne)
- `startDate` (gt, ge, lt, le, eq, ne)
- `dbaname`  (ct, nct)
- `legalname`  (ct, nct)
- `ein`  (ct, nct)
- `address`  (ct, nct)
- `city`  (ct, nct)
- `state`  (ct, nct)
- `phone`  (ct, nct)
- `mcc`  (ct, nct)
- `owntype`  (ct, nct)
- `ownerName`  (ct, nct)
- `contactName`  (ct, nct)
- `status`  (eq, ne)
- `orgParentname`  (ct, nct)

List of comparison accepted - enclosed between parentheses:
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array
- nin => not inside array

List of parameters accepted:
- `limitRecord` : max number of records for query (default="20", "0" or negative value for all)
- `fromRecord` : initial record in query

Example: `dbaname(ct)=hoa` returns all records with a `dbaname` containing "hoa"
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_batch_details</a>(format: ExportFormat1, entry: String, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint is deprecated. Export batch details for a paypoint. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat1;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .export
        .export_batch_details(
            &ExportFormat1::Csv,
            &"8cfec329267".to_string(),
            &ExportBatchDetailsQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `ExportFormat1` — Format for the export, either XLSX or CSV. 
    
</dd>
</dl>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

**List of field names accepted:**

  - `settlementDate` (gt, ge, lt, le, eq, ne)
  - `depositDate` (gt, ge, lt, le, eq, ne)
  - `transId`  (ne, eq, ct, nct)
  - `gatewayTransId`  (ne, eq, ct, nct)
  - `method`   (in, nin, eq, ne)
  - `settledAmount`  (gt, ge, lt, le, eq, ne)
  - `operation`    (in, nin, eq, ne)
  - `source`   (in, nin, eq, ne)
  - `batchNumber`  (ct, nct, eq, ne)
  - `payaccountLastfour`   (nct, ct)
  - `payaccountType`   (ne, eq, in, nin)
  - `customerFirstname`   (ct, nct, eq, ne)
  - `customerLastname`    (ct, nct, eq, ne)
  - `customerName`   (ct, nct)
  - `customerId`  (eq, ne)
  - `customerNumber`  (ct, nct, eq, ne)
  - `customerCompanyname`    (ct, nct, eq, ne)
  - `customerAddress` (ct, nct, eq, ne)
  - `customerCity`    (ct, nct, eq, ne)
  - `customerZip` (ct, nct, eq, ne)
  - `customerState` (ct, nct, eq, ne)
  - `customerCountry` (ct, nct, eq, ne)
  - `customerPhone` (ct, nct, eq, ne)
  - `customerEmail` (ct, nct, eq, ne)
  - `customerShippingAddress` (ct, nct, eq, ne)
  - `customerShippingCity`    (ct, nct, eq, ne)
  - `customerShippingZip` (ct, nct, eq, ne)
  - `customerShippingState` (ct, nct, eq, ne)
  - `customerShippingCountry` (ct, nct, eq, ne)
  - `orgId`  (eq) *mandatory when entry=org*
  - `isHold` (eq, ne)
  - `paypointId`  (ne, eq)
  - `paypointLegal`  (ne, eq, ct, nct)
  - `paypointDba`  (ne, eq, ct, nct)
  - `orgName`  (ne, eq, ct, nct)
  - `batchId` (ct, nct, eq, neq)
  - `additional-xxx`  (ne, eq, ct, nct) where xxx is the additional field name

List of parameters accepted:
- limitRecord: max number of records for query (default="20", "0" or negative value for all)
- fromRecord: initial record in query

Example: `amount(gt)=20` return all records with amount greater than 20.00
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_batch_details_org</a>(format: ExportFormat1, org_id: i64, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint is deprecated. Export batch details for an organization. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat1;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .export
        .export_batch_details_org(
            &ExportFormat1::Csv,
            123,
            &ExportBatchDetailsOrgQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `ExportFormat1` — Format for the export, either XLSX or CSV. 
    
</dd>
</dl>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

**List of field names accepted:**

  - `settlementDate` (gt, ge, lt, le, eq, ne)
  - `depositDate` (gt, ge, lt, le, eq, ne)
  - `transId`  (ne, eq, ct, nct)
  - `gatewayTransId`  (ne, eq, ct, nct)
  - `method`   (in, nin, eq, ne)
  - `settledAmount`  (gt, ge, lt, le, eq, ne)
  - `operation`    (in, nin, eq, ne)
  - `source`   (in, nin, eq, ne)
  - `batchNumber`  (ct, nct, eq, ne)
  - `payaccountLastfour`   (nct, ct)
  - `payaccountType`   (ne, eq, in, nin)
  - `customerFirstname`   (ct, nct, eq, ne)
  - `customerLastname`    (ct, nct, eq, ne)
  - `customerName`   (ct, nct)
  - `customerId`  (eq, ne)
  - `customerNumber`  (ct, nct, eq, ne)
  - `customerCompanyname`    (ct, nct, eq, ne)
  - `customerAddress` (ct, nct, eq, ne)
  - `customerCity`    (ct, nct, eq, ne)
  - `customerZip` (ct, nct, eq, ne)
  - `customerState` (ct, nct, eq, ne)
  - `customerCountry` (ct, nct, eq, ne)
  - `customerPhone` (ct, nct, eq, ne)
  - `customerEmail` (ct, nct, eq, ne)
  - `customerShippingAddress` (ct, nct, eq, ne)
  - `customerShippingCity`    (ct, nct, eq, ne)
  - `customerShippingZip` (ct, nct, eq, ne)
  - `customerShippingState` (ct, nct, eq, ne)
  - `customerShippingCountry` (ct, nct, eq, ne)
  - `orgId`  (eq) *mandatory when entry=org*
  - `isHold` (eq, ne)
  - `paypointId`  (ne, eq)
  - `paypointLegal`  (ne, eq, ct, nct)
  - `paypointDba`  (ne, eq, ct, nct)
  - `orgName`  (ne, eq, ct, nct)
  - `batchId` (ct, nct, eq, neq)
  - `additional-xxx`  (ne, eq, ct, nct) where xxx is the additional field name

List of parameters accepted:
- limitRecord: max number of records for query (default="20", "0" or negative value for all)
- fromRecord: initial record in query

Example: `amount(gt)=20` return all records with amount greater than 20.00
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_batches</a>(format: ExportFormat1, entry: String, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export a list of batches for an entrypoint. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat1;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .export
        .export_batches(
            &ExportFormat1::Csv,
            &"8cfec329267".to_string(),
            &ExportBatchesQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `ExportFormat1` — Format for the export, either XLSX or CSV. 
    
</dd>
</dl>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

List of field names accepted:
- `batchDate` (gt, ge, lt, le, eq, ne)
- `batchNumber` (ne, eq)
- `connectorName` (ne, eq, ct, nct)
- `method` (in, nin, eq, ne)
- `batchAmount` (gt, ge, lt, le, eq, ne)
- `feeBatchAmount` (gt, ge, lt, le, eq, ne)
- `netBatchAmount` (gt, ge, lt, le, eq, ne)
- `releaseAmount` (gt, ge, lt, le, eq, ne)
- `heldAmount` (gt, ge, lt, le, eq, ne)
- `status` (in, nin, eq, ne)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)
- `paypointId` (ne, eq)
- `externalPaypointID` (ct, nct, eq, ne)
- `expectedDepositDate` (gt, ge, lt, le, eq, ne)
- `batchRecords` (gt, ge, lt, le, eq, ne)
- `transferId` (ne, eq)
- `transferDate` (gt, ge, lt, le, eq, ne)
- `grossAmount` (gt, ge, lt, le, eq, ne)
- `chargeBackAmount` (gt, ge, lt, le, eq, ne)
- `returnedAmount` (gt, ge, lt, le, eq, ne)
- `billingFeeAmount` (gt, ge, lt, le, eq, ne)
- `thirdPartyPaidAmount` (gt, ge, lt, le, eq, ne)
- `netFundedAmount` (gt, ge, lt, le, eq, ne)
- `adjustmentAmount` (gt, ge, lt, le, eq, ne)
- `processor` (ne, eq, ct, nct)
- `transferStatus` (ne, eq, in, nin)

List of parameters accepted:
- limitRecord: max number of records for query (default="20", "0" or negative value for all)
- fromRecord: initial record in query

Example: `batchAmount(gt)=20` returns all records with a `batchAmount` greater than 20.00
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_batches_org</a>(format: ExportFormat1, org_id: i64, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export a list of batches for an organization. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat1;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .export
        .export_batches_org(
            &ExportFormat1::Csv,
            123,
            &ExportBatchesOrgQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `ExportFormat1` — Format for the export, either XLSX or CSV. 
    
</dd>
</dl>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>
See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

List of field names accepted:
- `batchDate` (gt, ge, lt, le, eq, ne)
- `batchNumber` (ne, eq)
- `connectorName` (ne, eq, ct, nct)
- `method` (in, nin, eq, ne)
- `batchAmount` (gt, ge, lt, le, eq, ne)
- `feeBatchAmount` (gt, ge, lt, le, eq, ne)
- `netBatchAmount` (gt, ge, lt, le, eq, ne)
- `releaseAmount` (gt, ge, lt, le, eq, ne)
- `heldAmount` (gt, ge, lt, le, eq, ne)
- `status` (in, nin, eq, ne)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)
- `paypointId` (ne, eq)
- `externalPaypointID` (ct, nct, eq, ne)
- `expectedDepositDate` (gt, ge, lt, le, eq, ne)
- `batchRecords` (gt, ge, lt, le, eq, ne)
- `transferId` (ne, eq)
- `transferDate` (gt, ge, lt, le, eq, ne)
- `grossAmount` (gt, ge, lt, le, eq, ne)
- `chargeBackAmount` (gt, ge, lt, le, eq, ne)
- `returnedAmount` (gt, ge, lt, le, eq, ne)
- `billingFeeAmount` (gt, ge, lt, le, eq, ne)
- `thirdPartyPaidAmount` (gt, ge, lt, le, eq, ne)
- `netFundedAmount` (gt, ge, lt, le, eq, ne)
- `adjustmentAmount` (gt, ge, lt, le, eq, ne)
- `processor` (ne, eq, ct, nct)
- `transferStatus` (ne, eq, in, nin)

List of parameters accepted:
- `limitRecord`: max number of records for query (default="20", "0" or negative value for all)
- `fromRecord`: initial record in query
Example: `batchAmount(gt)=20` returns all records with a `batchAmount` greater than 20.00
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_batches_out</a>(format: ExportFormat1, entry: String, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export a list of money out batches for a paypoint. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat1;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .export
        .export_batches_out(
            &ExportFormat1::Csv,
            &"8cfec329267".to_string(),
            &ExportBatchesOutQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `ExportFormat1` — Format for the export, either XLSX or CSV. 
    
</dd>
</dl>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

List of field names accepted:
  - `batchDate` (gt, ge, lt, le, eq, ne)
  - `batchNumber` (ne, eq)
  - `batchAmount` (gt, ge, lt, le, eq, ne)
  - `status` (in, nin, eq, ne)
  - `paypointLegal` (ne, eq, ct, nct)
  - `paypointDba` (ne, eq, ct, nct)
  - `orgName` (ne, eq, ct, nct, nin, in)
  - `paypointId` (ne, eq)
  - `externalPaypointID` (ct, nct, eq, ne)
List of parameters accepted:
- limitRecord: max number of records for query (default="20", "0" or negative value for all)
- fromRecord: initial record in query

Example: `batchAmount(gt)=20` returns all records with a `batchAmount` greater than 20.00"
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_batches_out_org</a>(format: ExportFormat1, org_id: i64, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export a list of money out batches for an organization. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat1;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .export
        .export_batches_out_org(
            &ExportFormat1::Csv,
            123,
            &ExportBatchesOutOrgQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `ExportFormat1` — Format for the export, either XLSX or CSV. 
    
</dd>
</dl>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

List of field names accepted:
  - `batchDate` (gt, ge, lt, le, eq, ne)
  - `batchNumber` (ne, eq)
  - `batchAmount` (gt, ge, lt, le, eq, ne)
  - `status` (in, nin, eq, ne)
  - `paypointLegal` (ne, eq, ct, nct)
  - `paypointDba` (ne, eq, ct, nct)
  - `orgName` (ne, eq, ct, nct, nin, in)
  - `paypointId` (ne, eq)
  - `externalPaypointID` (ct, nct, eq, ne)
List of parameters accepted:
- limitRecord: max number of records for query (default="20", "0" or negative value for all)
- fromRecord: initial record in query

Example: `batchAmount(gt)=20` returns all records with a `batchAmount` greater than 20.00"
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_bills</a>(format: ExportFormat1, entry: String, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export a list of bills for an entrypoint. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat1;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .export
        .export_bills(
            &ExportFormat1::Csv,
            &"8cfec329267".to_string(),
            &ExportBillsQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `ExportFormat1` — Format for the export, either XLSX or CSV. 
    
</dd>
</dl>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query 

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help. 

List of field names accepted:
- `status` (in, nin, eq, ne)
- `billNumber` (ct, nct, eq, ne)
- `billDate` (gt, ge, lt, le, eq, ne)
- `billDueDate` (gt, ge, lt, le, eq, ne)
- `vendorNumber` (ct, nct, eq, ne)
- `vendorName` (ct, nct, eq, ne)
- `ein` (ct, nct, eq, ne)
- `paymentMethod` (ct, nct, eq, ne)
- `paymentId` (ct, nct, eq, ne)
- `paymentgroup` (ct, nct, eq, ne)
- `totalAmount` (gt, ge, lt, le, eq, ne)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)

List of comparison accepted - enclosed between parentheses:
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array separated by "|"
- nin => not inside array separated by "|"

List of parameters accepted:
- limitRecord : max number of records for query (default="20", "0" or negative value for all)
- fromRecord : initial record in query

Example: totalAmount(gt)=20  return all records with totalAmount greater than 20.00
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_bills_org</a>(format: ExportFormat1, org_id: i64, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export a list of bills for an organization. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat1;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .export
        .export_bills_org(
            &ExportFormat1::Csv,
            123,
            &ExportBillsOrgQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `ExportFormat1` — Format for the export, either XLSX or CSV. 
    
</dd>
</dl>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query 

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help. 

List of field names accepted:
- `status` (in, nin, eq, ne)
- `billNumber` (ct, nct, eq, ne)
- `billDate` (gt, ge, lt, le, eq, ne)
- `billDueDate` (gt, ge, lt, le, eq, ne)
- `vendorNumber` (ct, nct, eq, ne)
- `vendorName` (ct, nct, eq, ne)
- `ein` (ct, nct, eq, ne)
- `paymentMethod` (ct, nct, eq, ne)
- `paymentId` (ct, nct, eq, ne)
- `paymentgroup` (ct, nct, eq, ne)
- `totalAmount` (gt, ge, lt, le, eq, ne)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)

List of comparison accepted - enclosed between parentheses:
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array separated by "|"
- nin => not inside array separated by "|"

List of parameters accepted:
- limitRecord : max number of records for query (default="20", "0" or negative value for all)
- fromRecord : initial record in query

Example: totalAmount(gt)=20  return all records with totalAmount greater than 20.00
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_chargebacks</a>(format: ExportFormat1, entry: String, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export a list of chargebacks and ACH returns for an entrypoint. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat1;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .export
        .export_chargebacks(
            &ExportFormat1::Csv,
            &"8cfec329267".to_string(),
            &ExportChargebacksQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `ExportFormat1` — Format for the export, either XLSX or CSV. 
    
</dd>
</dl>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query 

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help. 

List of field names accepted:
- `chargebackDate` (gt, ge, lt, le, eq, ne)
- `transId` (ne, eq, ct, nct)
- `method` (in, nin, eq, ne)
- `netAmount` (gt, ge, lt, le, eq, ne)
- `reasonCode` (in, nin, eq, ne)
- `reason` (ct, nct, eq, ne)
- `caseNumber` (ct, nct, eq, ne)
- `status` (in, nin, eq, ne)
- `accountType` (in, nin, eq, ne)
- `payaccountLastfour` (nct, ct)
- `payaccountType` (ne, eq, in, nin)
- `customerFirstname` (ct, nct, eq, ne)
- `customerLastname` (ct, nct, eq, ne)
- `customerName` (ct, nct)
- `customerId` (eq, ne)
- `customerNumber` (ct, nct, eq, ne)
- `customerCompanyname` (ct, nct, eq, ne)
- `customerAddress` (ct, nct, eq, ne)
- `customerCity` (ct, nct, eq, ne)
- `customerZip` (ct, nct, eq, ne)
- `customerState` (ct, nct, eq, ne)
- `customerCountry` (ct, nct, eq, ne)
- `customerPhone` (ct, nct, eq, ne)
- `customerEmail` (ct, nct, eq, ne)
- `customerShippingAddress` (ct, nct, eq, ne)
- `customerShippingCity` (ct, nct, eq, ne)
- `customerShippingZip` (ct, nct, eq, ne)
- `customerShippingState` (ct, nct, eq, ne)
- `customerShippingCountry` (ct, nct, eq, ne)
- `orgId` (eq) *mandatory when entry=org*
- `paypointId` (ne, eq)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)
- `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name

List of comparison accepted - enclosed between parentheses:
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array separated by "|"
- nin => not inside array separated by "|"

List of parameters accepted:
- limitRecord : max number of records for query (default="20", "0" or negative value for all)
- fromRecord : initial record in query

Example: `netAmount(gt)=20` returns all records with a `netAmount` greater than 20.00
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_chargebacks_org</a>(format: ExportFormat1, org_id: i64, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export a list of chargebacks and ACH returns for an organization. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat1;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .export
        .export_chargebacks_org(
            &ExportFormat1::Csv,
            123,
            &ExportChargebacksOrgQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `ExportFormat1` — Format for the export, either XLSX or CSV. 
    
</dd>
</dl>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query 

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help. 

List of field names accepted:
- `chargebackDate` (gt, ge, lt, le, eq, ne)
- `transId` (ne, eq, ct, nct)
- `method` (in, nin, eq, ne)
- `netAmount` (gt, ge, lt, le, eq, ne)
- `reasonCode` (in, nin, eq, ne)
- `reason` (ct, nct, eq, ne)
- `caseNumber` (ct, nct, eq, ne)
- `status` (in, nin, eq, ne)
- `accountType` (in, nin, eq, ne)
- `payaccountLastfour` (nct, ct)
- `payaccountType` (ne, eq, in, nin)
- `customerFirstname` (ct, nct, eq, ne)
- `customerLastname` (ct, nct, eq, ne)
- `customerName` (ct, nct)
- `customerId` (eq, ne)
- `customerNumber` (ct, nct, eq, ne)
- `customerCompanyname` (ct, nct, eq, ne)
- `customerAddress` (ct, nct, eq, ne)
- `customerCity` (ct, nct, eq, ne)
- `customerZip` (ct, nct, eq, ne)
- `customerState` (ct, nct, eq, ne)
- `customerCountry` (ct, nct, eq, ne)
- `customerPhone` (ct, nct, eq, ne)
- `customerEmail` (ct, nct, eq, ne)
- `customerShippingAddress` (ct, nct, eq, ne)
- `customerShippingCity` (ct, nct, eq, ne)
- `customerShippingZip` (ct, nct, eq, ne)
- `customerShippingState` (ct, nct, eq, ne)
- `customerShippingCountry` (ct, nct, eq, ne)
- `orgId` (eq) *mandatory when entry=org*
- `paypointId` (ne, eq)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)
- `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name

List of comparison accepted - enclosed between parentheses:
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array separated by "|"
- nin => not inside array separated by "|"

List of parameters accepted:
- limitRecord : max number of records for query (default="20", "0" or negative value for all)
- fromRecord : initial record in query

Example: `netAmount(gt)=20` returns all records with a `netAmount` greater than 20.00
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_customers</a>(format: ExportFormat1, entry: String, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export a list of customers for an entrypoint. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat1;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .export
        .export_customers(
            &ExportFormat1::Csv,
            &"8cfec329267".to_string(),
            &ExportCustomersQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `ExportFormat1` — Format for the export, either XLSX or CSV. 
    
</dd>
</dl>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query.

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

**List of field names accepted:**
- `createdDate` (gt, ge, lt, le, eq, ne)
- `customernumber` (ne, eq, ct, nct)
- `firstname` (ne, eq, ct, nct)
- `lastname` (ne, eq, ct, nct)
- `name` (ct, nct)
- `address` (ne, eq, ct, nct)
- `city` (ne, eq, ct, nct)
- `country` (ne, eq, ct, nct)
- `zip` (ne, eq, ct, nct)
- `state` (ne, eq, ct, nct)
- `shippingaddress` (ne, eq, ct, nct)
- `shippingcity` (ne, eq, ct, nct)
- `shippingcountry` (ne, eq, ct, nct)
- `shippingzip` (ne, eq, ct, nct)
- `shippingstate` (ne, eq, ct, nct)
- `phone` (ne, eq, ct, nct)
- `email` (ne, eq, ct, nct)
- `company` (ne, eq, ct, nct)
- `username` (ne, eq, ct, nct)
- `balance` (gt, ge, lt, le, eq, ne)
- `status` (in, nin, eq, ne)
- `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name
- `orgId` (eq) *mandatory when entry=org*
- `paypointId` (ne, eq)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)

**List of comparison accepted - enclosed between parentheses:**
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array separated by "|"
- nin => not inside array separated by "|"

**List of parameters accepted:**
- limitRecord: max number of records for query (default="20", "0" or negative value for all)
- fromRecord: initial record in query

**Example:**
balance(gt)=20 return all records with balance greater than 20.00
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_customers_org</a>(format: ExportFormat1, org_id: i64, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Exports a list of customers for an organization. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat1;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .export
        .export_customers_org(
            &ExportFormat1::Csv,
            123,
            &ExportCustomersOrgQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `ExportFormat1` — Format for the export, either XLSX or CSV. 
    
</dd>
</dl>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query.

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

**List of field names accepted:**
- `createdDate` (gt, ge, lt, le, eq, ne)
- `customernumber` (ne, eq, ct, nct)
- `firstname` (ne, eq, ct, nct)
- `lastname` (ne, eq, ct, nct)
- `name` (ct, nct)
- `address` (ne, eq, ct, nct)
- `city` (ne, eq, ct, nct)
- `country` (ne, eq, ct, nct)
- `zip` (ne, eq, ct, nct)
- `state` (ne, eq, ct, nct)
- `shippingaddress` (ne, eq, ct, nct)
- `shippingcity` (ne, eq, ct, nct)
- `shippingcountry` (ne, eq, ct, nct)
- `shippingzip` (ne, eq, ct, nct)
- `shippingstate` (ne, eq, ct, nct)
- `phone` (ne, eq, ct, nct)
- `email` (ne, eq, ct, nct)
- `company` (ne, eq, ct, nct)
- `username` (ne, eq, ct, nct)
- `balance` (gt, ge, lt, le, eq, ne)
- `status` (in, nin, eq, ne)
- `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name
- `orgId` (eq) *mandatory when entry=org*
- `paypointId` (ne, eq)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)

**List of comparison accepted - enclosed between parentheses:**
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array separated by "|"
- nin => not inside array separated by "|"

**List of parameters accepted:**
- limitRecord: max number of records for query (default="20", "0" or negative value for all)
- fromRecord: initial record in query

**Example:**
balance(gt)=20 return all records with balance greater than 20.00
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_invoices</a>(format: ExportFormat1, entry: String, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export list of invoices for an entrypoint. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat1;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .export
        .export_invoices(
            &ExportFormat1::Csv,
            &"8cfec329267".to_string(),
            &ExportInvoicesQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `ExportFormat1` — Format for the export, either XLSX or CSV. 
    
</dd>
</dl>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query 

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help. 

List of field names accepted:
 - `invoiceDate` (gt, ge, lt, le, eq, ne)
 - `dueDate` (gt, ge, lt, le, eq, ne)
 - `sentDate` (gt, ge, lt, le, eq, ne)
 - `frequency`  (in, nin,ne, eq)
 - `invoiceType`   (eq, ne)
 - `payTerms`   (in, nin, eq, ne)
 - `paypointId`  (ne, eq)
 - `totalAmount`  (gt, ge, lt, le, eq, ne)
 - `paidAmount`  (gt, ge, lt, le, eq, ne)
 - `status`   (in, nin, eq, ne)
 - `invoiceNumber`   (ct, nct, eq, ne)
 - `purchaseOrder`   (ct, nct, eq, ne)
 - `itemProductCode` (ct, nct)
 - `itemDescription` (ct, nct)
 - `customerFirstname`   (ct, nct, eq, ne)
 - `customerLastname`    (ct, nct, eq, ne)
 - `customerName`   (ct, nct)
 - `customerId`  (eq, ne)
 - `customerNumber`  (ct, nct, eq, ne)
 - `customerCompanyname`    (ct, nct, eq, ne)
 - `customerAddress` (ct, nct, eq, ne)
 - `customerCity`    (ct, nct, eq, ne)
 - `customerZip` (ct, nct, eq, ne)
 - `customerState` (ct, nct, eq, ne)
 - `customerCountry` (ct, nct, eq, ne)
 - `customerPhone` (ct, nct, eq, ne)
 - `customerEmail` (ct, nct, eq, ne)
 - `customerShippingAddress` (ct, nct, eq, ne)
 - `customerShippingCity` (ct, nct, eq, ne)
 - `customerShippingZip` (ct, nct, eq, ne)
 - `customerShippingState` (ct, nct, eq, ne)
 - `customerShippingCountry` (ct, nct, eq, ne)
 - `orgId`  (eq) 
 - `paylinkId`  (ne, eq)
 - `paypointLegal`  (ne, eq, ct, nct)
 - `paypointDba`  (ne, eq, ct, nct)
 - `orgName`  (ne, eq, ct, nct)
 - `additional-xxx`  (ne, eq, ct, nct) where xxx is the additional field name

List of comparison accepted - enclosed between parentheses:
 - eq or empty => equal
 - gt => greater than
 - ge => greater or equal
 - lt => less than
 - le => less or equal
 - ne => not equal
 - ct => contains
 - nct => not contains
 - in => inside array
 - nin => not inside array
 
List of parameters accepted:
 - `limitRecord` : max number of records for query (default="20", "0" or negative value for all)
 - `fromRecord` : initial record in query
 
Example: `totalAmount(gt)=20` returns all records with `totalAmount` greater than 20.00
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_invoices_org</a>(format: ExportFormat1, org_id: i64, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export a list of invoices for an organization. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat1;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .export
        .export_invoices_org(
            &ExportFormat1::Csv,
            123,
            &ExportInvoicesOrgQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `ExportFormat1` — Format for the export, either XLSX or CSV. 
    
</dd>
</dl>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query 

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help. 

List of field names accepted:
 - `invoiceDate` (gt, ge, lt, le, eq, ne)
 - `dueDate` (gt, ge, lt, le, eq, ne)
 - `sentDate` (gt, ge, lt, le, eq, ne)
 - `frequency` (in, nin,ne, eq)
 - `invoiceType` (eq, ne)
 - `payTerms` (in, nin, eq, ne)
 - `paypointId` (ne, eq)
 - `totalAmount` (gt, ge, lt, le, eq, ne)
 - `paidAmount` (gt, ge, lt, le, eq, ne)
 - `status` (in, nin, eq, ne)
 - `invoiceNumber` (ct, nct, eq, ne)
 - `purchaseOrder` (ct, nct, eq, ne)
 - `itemProductCode` (ct, nct)
 - `itemDescription` (ct, nct)
 - `customerFirstname` (ct, nct, eq, ne)
 - `customerLastname` (ct, nct, eq, ne)
 - `customerName` (ct, nct)
 - `customerId` (eq, ne)
 - `customerNumber` (ct, nct, eq, ne)
 - `customerCompanyname` (ct, nct, eq, ne)
 - `customerAddress` (ct, nct, eq, ne)
 - `customerCity` (ct, nct, eq, ne)
 - `customerZip` (ct, nct, eq, ne)
 - `customerState` (ct, nct, eq, ne)
 - `customerCountry` (ct, nct, eq, ne)
 - `customerPhone` (ct, nct, eq, ne)
 - `customerEmail` (ct, nct, eq, ne)
 - `customerShippingAddress` (ct, nct, eq, ne)
 - `customerShippingCity` (ct, nct, eq, ne)
 - `customerShippingZip` (ct, nct, eq, ne)
 - `customerShippingState` (ct, nct, eq, ne)
 - `customerShippingCountry` (ct, nct, eq, ne)
 - `orgId` (eq) 
 - `paylinkId` (ne, eq)
 - `paypointLegal` (ne, eq, ct, nct)
 - `paypointDba` (ne, eq, ct, nct)
 - `orgName` (ne, eq, ct, nct)
 - `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name
 
List of comparison accepted - enclosed between parentheses:
 - eq or empty => equal
 - gt => greater than
 - ge => greater or equal
 - lt => less than
 - le => less or equal
 - ne => not equal
 - ct => contains
 - nct => not contains
 - in => inside array
 - nin => not inside array
 
List of parameters accepted:
 - limitRecord : max number of records for query (default="20", "0" or negative value for all)
 - fromRecord : initial record in query
 
Example: totalAmount(gt)=20  return all records with totalAmount greater than 20.00
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_organizations</a>(format: ExportFormat1, org_id: i64, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export a list of child organizations (suborganizations) for a parent organization.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat1;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .export
        .export_organizations(
            &ExportFormat1::Csv,
            123,
            &ExportOrganizationsQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `ExportFormat1` — Format for the export, either XLSX or CSV. 
    
</dd>
</dl>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help. 

List of field names accepted:
- `name` (ct, nct, eq, ne)
- `type` (ne, eq)
- `contactName` (ct, nct, eq, ne)
- `contactTitle` (ct, nct, eq, ne)
- `contactEmail` (ct, nct, eq, ne)
- `contactPhone` (ct, nct, eq, ne)
- `city` (ct, nct, eq, ne)
- `state` (in, nin, eq, ne)
- `address` (ct, nct, eq, ne)
- `country` (ct, nct, eq, ne)
- `zip` (ct, nct, eq, ne)
- `hasBilling` any value greater than zero is taken as TRUE otherwise is FALSE
- `hasResidual` any value greater than zero is taken as TRUE otherwise is FALSE

List of comparison accepted - enclosed between parentheses:
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array
- nin => not inside array

List of parameters accepted:
- limitRecord : max number of records for query (default="20", "0" or negative value for all)
- fromRecord : initial record in query

Example: name(ct)=hoa  return all records where name contains "hoa"
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_payout</a>(format: ExportFormat1, entry: String, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export a list of payouts and their statuses for an entrypoint. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat1;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .export
        .export_payout(
            &ExportFormat1::Csv,
            &"8cfec329267".to_string(),
            &ExportPayoutQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `ExportFormat1` — Format for the export, either XLSX or CSV. 
    
</dd>
</dl>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query.

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

List of field names accepted:
- `status` (in, nin, eq, ne)
- `transactionDate` (gt, ge, lt, le, eq, ne)
- `billNumber` (ct, nct)
- `vendorNumber` (ct, nct, eq, ne)
- `vendorName` (ct, nct, eq, ne)
- `paymentMethod` (ct, nct, eq, ne)
- `paymentId` (ct, nct, eq, ne)
- `paymentgroup` (ct, nct, eq, ne)
- `totalAmount` (gt, ge, lt, le, eq, ne)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)

List of comparison accepted - enclosed between parentheses:
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array separated by "|"
- nin => not inside array separated by "|"

List of parameters accepted:
- limitRecord: max number of records for query (default="20", "0" or negative value for all)
- fromRecord: initial record in query

Example: totalAmount(gt)=20 return all records with totalAmount greater than 20.00
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_payout_org</a>(format: ExportFormat1, org_id: i64, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export a list of payouts and their details for an organization. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat1;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .export
        .export_payout_org(
            &ExportFormat1::Csv,
            123,
            &ExportPayoutOrgQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `ExportFormat1` — Format for the export, either XLSX or CSV. 
    
</dd>
</dl>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query.

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

List of field names accepted:
- `status` (in, nin, eq, ne)
- `transactionDate` (gt, ge, lt, le, eq, ne)
- `billNumber` (ct, nct)
- `vendorNumber` (ct, nct, eq, ne)
- `vendorName` (ct, nct, eq, ne)
- `paymentMethod` (ct, nct, eq, ne)
- `paymentId` (ct, nct, eq, ne)
- `paymentgroup` (ct, nct, eq, ne)
- `totalAmount` (gt, ge, lt, le, eq, ne)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)

List of comparison accepted - enclosed between parentheses:
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array separated by "|"
- nin => not inside array separated by "|"

List of parameters accepted:
- limitRecord: max number of records for query (default="20", "0" or negative value for all)
- fromRecord: initial record in query

Example: totalAmount(gt)=20 return all records with totalAmount greater than 20.00
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_paypoints</a>(format: ExportFormat1, org_id: i64, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export a list of paypoints in an organization. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat1;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .export
        .export_paypoints(
            &ExportFormat1::Csv,
            123,
            &ExportPaypointsQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `ExportFormat1` — Format for the export, either XLSX or CSV. 
    
</dd>
</dl>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query.

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

List of field names accepted:
- `createdAt` (gt, ge, lt, le, eq, ne)
- `startDate` (gt, ge, lt, le, eq, ne)
- `dbaname` (ct, nct)
- `legalname` (ct, nct)
- `ein` (ct, nct)
- `address` (ct, nct)
- `city` (ct, nct)
- `state` (ct, nct)
- `phone` (ct, nct)
- `mcc` (ct, nct)
- `owntype` (ct, nct)
- `ownerName` (ct, nct)
- `contactName` (ct, nct)
- `orgParentname` (ct, nct)

List of comparison accepted - enclosed between parentheses:
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array
- nin => not inside array

List of parameters accepted:
- limitRecord : max number of records for query (default="20", "0" or negative value for all)
- fromRecord : initial record in query

Example: `dbaname(ct)=hoa` returns all records with `dbaname` containing "hoa"
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_settlements</a>(format: ExportFormat1, entry: String, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export a list of settled transactions for an entrypoint. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat1;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .export
        .export_settlements(
            &ExportFormat1::Csv,
            &"8cfec329267".to_string(),
            &ExportSettlementsQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `ExportFormat1` — Format for the export, either XLSX or CSV. 
    
</dd>
</dl>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query 

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

List of field names accepted:
- `settlementDate` (gt, ge, lt, le, eq, ne)
- `transId` (ne, eq, ct, nct)
- `gatewayTransId` (ne, eq, ct, nct)
- `method` (in, nin, eq, ne)
- `settledAmount` (gt, ge, lt, le, eq, ne)
- `operation` (in, nin, eq, ne)
- `source` (in, nin, eq, ne)
- `batchNumber` (ct, nct, eq, ne)
- `payaccountLastfour` (nct, ct)
- `payaccountType` (ne, eq, in, nin)
- `customerFirstname` (ct, nct, eq, ne)
- `customerLastname` (ct, nct, eq, ne)
- `customerName` (ct, nct)
- `customerId` (eq, ne)
- `customerNumber` (ct, nct, eq, ne)
- `customerCompanyname` (ct, nct, eq, ne)
- `customerAddress` (ct, nct, eq, ne)
- `customerCity` (ct, nct, eq, ne)
- `customerZip` (ct, nct, eq, ne)
- `customerState` (ct, nct, eq, ne)
- `customerCountry` (ct, nct, eq, ne)
- `customerPhone` (ct, nct, eq, ne)
- `customerEmail` (ct, nct, eq, ne)
- `customerShippingAddress` (ct, nct, eq, ne)
- `customerShippingCity` (ct, nct, eq, ne)
- `customerShippingZip` (ct, nct, eq, ne)
- `customerShippingState` (ct, nct, eq, ne)
- `customerShippingCountry` (ct, nct, eq, ne)
- `orgId` (eq) *mandatory when entry=org*
- `paypointId` (ne, eq)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)
- `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name

List of comparison accepted - enclosed between parentheses:
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array separated by "|"
- nin => not inside array separated by "|"

List of parameters accepted:
- limitRecord: max number of records for query (default="20", "0" or negative value for all)
- fromRecord: initial record in query

Example: `settledAmount(gt)=20` returns all records with a `settledAmount` greater than 20.00
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_settlements_org</a>(format: ExportFormat1, org_id: i64, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export a list of settled transactions for an organization. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat1;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .export
        .export_settlements_org(
            &ExportFormat1::Csv,
            123,
            &ExportSettlementsOrgQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `ExportFormat1` — Format for the export, either XLSX or CSV. 
    
</dd>
</dl>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query 

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

List of field names accepted:
- `settlementDate` (gt, ge, lt, le, eq, ne)
- `transId` (ne, eq, ct, nct)
- `gatewayTransId` (ne, eq, ct, nct)
- `method` (in, nin, eq, ne)
- `settledAmount` (gt, ge, lt, le, eq, ne)
- `operation` (in, nin, eq, ne)
- `source` (in, nin, eq, ne)
- `batchNumber` (ct, nct, eq, ne)
- `payaccountLastfour` (nct, ct)
- `payaccountType` (ne, eq, in, nin)
- `customerFirstname` (ct, nct, eq, ne)
- `customerLastname` (ct, nct, eq, ne)
- `customerName` (ct, nct)
- `customerId` (eq, ne)
- `customerNumber` (ct, nct, eq, ne)
- `customerCompanyname` (ct, nct, eq, ne)
- `customerAddress` (ct, nct, eq, ne)
- `customerCity` (ct, nct, eq, ne)
- `customerZip` (ct, nct, eq, ne)
- `customerState` (ct, nct, eq, ne)
- `customerCountry` (ct, nct, eq, ne)
- `customerPhone` (ct, nct, eq, ne)
- `customerEmail` (ct, nct, eq, ne)
- `customerShippingAddress` (ct, nct, eq, ne)
- `customerShippingCity` (ct, nct, eq, ne)
- `customerShippingZip` (ct, nct, eq, ne)
- `customerShippingState` (ct, nct, eq, ne)
- `customerShippingCountry` (ct, nct, eq, ne)
- `orgId` (eq) *mandatory when entry=org*
- `paypointId` (ne, eq)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)
- `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name

List of comparison accepted - enclosed between parentheses:
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array separated by "|"
- nin => not inside array separated by "|"

List of parameters accepted:
- limitRecord: max number of records for query (default="20", "0" or negative value for all)
- fromRecord: initial record in query

Example: `settledAmount(gt)=20` returns all records with a `settledAmount` greater than 20.00
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_subscriptions</a>(format: ExportFormat1, entry: String, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export a list of subscriptions for an entrypoint. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat1;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .export
        .export_subscriptions(
            &ExportFormat1::Csv,
            &"8cfec329267".to_string(),
            &ExportSubscriptionsQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `ExportFormat1` — Format for the export, either XLSX or CSV. 
    
</dd>
</dl>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

List of field names accepted:
- `startDate` (gt, ge, lt, le, eq, ne)
- `endDate` (gt, ge, lt, le, eq, ne)
- `nextDate` (gt, ge, lt, le, eq, ne)
- `frequency` (in, nin, ne, eq)
- `method` (in, nin, eq, ne)
- `totalAmount` (gt, ge, lt, le, eq, ne)
- `netAmount` (gt, ge, lt, le, eq, ne)
- `feeAmount` (gt, ge, lt, le, eq, ne)
- `status` (in, nin, eq, ne)
- `untilcancelled` (eq, ne)
- `payaccountLastfour` (nct, ct)
- `payaccountType` (ne, eq, in, nin)
- `customerFirstname` (ct, nct, eq, ne)
- `customerLastname` (ct, nct, eq, ne)
- `customerName` (ct, nct)
- `customerId` (eq, ne)
- `customerNumber` (ct, nct, eq, ne)
- `customerCompanyname` (ct, nct, eq, ne)
- `customerAddress` (ct, nct, eq, ne)
- `customerCity` (ct, nct, eq, ne)
- `customerZip` (ct, nct, eq, ne)
- `customerState` (ct, nct, eq, ne)
- `customerCountry` (ct, nct, eq, ne)
- `customerPhone` (ct, nct, eq, ne)
- `customerEmail` (ct, nct, eq, ne)
- `customerShippingAddress` (ct, nct, eq, ne)
- `customerShippingCity` (ct, nct, eq, ne)
- `customerShippingZip` (ct, nct, eq, ne)
- `customerShippingState` (ct, nct, eq, ne)
- `customerShippingCountry` (ct, nct, eq, ne)
- `orgId` (eq) 
- `paypointId` (ne, eq)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)
- `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name

List of comparison accepted - enclosed between parentheses:
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array
- nin => not inside array

List of parameters accepted:
- limitRecord : max number of records for query (default="20", "0" or negative value for all)
- fromRecord : initial record in query

Example: `netAmount(gt)=20` returns all records with a `netAmount` greater than 20.00
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_subscriptions_org</a>(format: ExportFormat1, org_id: i64, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export a list of subscriptions for an organization. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat1;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .export
        .export_subscriptions_org(
            &ExportFormat1::Csv,
            123,
            &ExportSubscriptionsOrgQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `ExportFormat1` — Format for the export, either XLSX or CSV. 
    
</dd>
</dl>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

List of field names accepted:
- `startDate` (gt, ge, lt, le, eq, ne)
- `endDate` (gt, ge, lt, le, eq, ne)
- `nextDate` (gt, ge, lt, le, eq, ne)
- `frequency` (in, nin, ne, eq)
- `method` (in, nin, eq, ne)
- `totalAmount` (gt, ge, lt, le, eq, ne)
- `netAmount` (gt, ge, lt, le, eq, ne)
- `feeAmount` (gt, ge, lt, le, eq, ne)
- `status` (in, nin, eq, ne)
- `untilcancelled` (eq, ne)
- `payaccountLastfour` (nct, ct)
- `payaccountType` (ne, eq, in, nin)
- `customerFirstname` (ct, nct, eq, ne)
- `customerLastname` (ct, nct, eq, ne)
- `customerName` (ct, nct)
- `customerId` (eq, ne)
- `customerNumber` (ct, nct, eq, ne)
- `customerCompanyname` (ct, nct, eq, ne)
- `customerAddress` (ct, nct, eq, ne)
- `customerCity` (ct, nct, eq, ne)
- `customerZip` (ct, nct, eq, ne)
- `customerState` (ct, nct, eq, ne)
- `customerCountry` (ct, nct, eq, ne)
- `customerPhone` (ct, nct, eq, ne)
- `customerEmail` (ct, nct, eq, ne)
- `customerShippingAddress` (ct, nct, eq, ne)
- `customerShippingCity` (ct, nct, eq, ne)
- `customerShippingZip` (ct, nct, eq, ne)
- `customerShippingState` (ct, nct, eq, ne)
- `customerShippingCountry` (ct, nct, eq, ne)
- `orgId` (eq) 
- `paypointId` (ne, eq)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)
- `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name

List of comparison accepted - enclosed between parentheses:
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array
- nin => not inside array

List of parameters accepted:
- limitRecord : max number of records for query (default="20", "0" or negative value for all)
- fromRecord : initial record in query

Example: `netAmount(gt)=20` returns all records with a `netAmount` greater than 20.00
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_transactions</a>(format: ExportFormat1, entry: String, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export a list of transactions for an entrypoint in a file in XLXS or CSV format. Use filters to limit results. If you don't specify a date range in the request, the last two months of data are returned.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat1;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .export
        .export_transactions(
            &ExportFormat1::Csv,
            &"8cfec329267".to_string(),
            &ExportTransactionsQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `ExportFormat1` — Format for the export, either XLSX or CSV. 
    
</dd>
</dl>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query 

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help. 

List of field names accepted:
- `transactionDate` (gt, ge, lt, le, eq, ne)
- `transId` (ne, eq, ct, nct)
- `gatewayTransId` (ne, eq, ct, nct)
- `orderId` (ne, eq)
- `idTrans` (ne, eq)
- `orgId` (ne, eq)
- `paypointId` (ne, eq)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)
- `method` (in, nin, eq, ne)
- `totalAmount` (gt, ge, lt, le, eq, ne)
- `netAmount` (gt, ge, lt, le, eq, ne)
- `feeAmount` (gt, ge, lt, le, eq, ne)
- `operation` (in, nin, eq, ne)
- `source` (in, nin, eq, ne)
- `status` (in, nin, eq, ne)
- `settlementStatus` (in, nin, eq, ne)
- `batchNumber` (nct, ct)
- `payaccountLastfour` (nct, ct)
- `payaccountType` (ne, eq, in, nin)
- `customerFirstname` (ct, nct, eq, ne)
- `customerLastname` (ct, nct, eq, ne)
- `customerName` (ct, nct)
- `customerId` (eq, ne)
- `customerNumber` (ct, nct, eq, ne)
- `customerCompanyname` (ct, nct, eq, ne)
- `customerAddress` (ct, nct, eq, ne)
- `customerCity` (ct, nct, eq, ne)
- `customerZip` (ct, nct, eq, ne)
- `customerState` (ct, nct, eq, ne)
- `customerCountry` (ct, nct, eq, ne)
- `customerPhone` (ct, nct, eq, ne)
- `customerEmail` (ct, nct, eq, ne)
- `customerShippingAddress` (ct, nct, eq, ne)
- `customerShippingCity` (ct, nct, eq, ne)
- `customerShippingZip` (ct, nct, eq, ne)
- `customerShippingState` (ct, nct, eq, ne)
- `customerShippingCountry` (ct, nct, eq, ne)
- `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name

List of comparison accepted - enclosed between parentheses:
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array
- nin => not inside array

List of parameters accepted:
- limitRecord : max number of records for query (default="20", "0" or negative value for all)
- fromRecord : initial record in query

Example: `netAmount(gt)=20` returns all records with a `netAmount` greater than 20.00
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_transactions_org</a>(format: ExportFormat1, org_id: i64, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export a list of transactions for an org in a file in XLSX or CSV format. Use filters to limit results. If you don't specify a date range in the request, the last two months of data are returned.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat1;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .export
        .export_transactions_org(
            &ExportFormat1::Csv,
            123,
            &ExportTransactionsOrgQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `ExportFormat1` — Format for the export, either XLSX or CSV. 
    
</dd>
</dl>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query 

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help. 

List of field names accepted:
- `transactionDate` (gt, ge, lt, le, eq, ne)
- `transId` (ne, eq, ct, nct)
- `gatewayTransId` (ne, eq, ct, nct)
- `orderId` (ne, eq)
- `idTrans` (ne, eq)
- `orgId` (ne, eq)
- `paypointId` (ne, eq)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)
- `method` (in, nin, eq, ne)
- `totalAmount` (gt, ge, lt, le, eq, ne)
- `netAmount` (gt, ge, lt, le, eq, ne)
- `feeAmount` (gt, ge, lt, le, eq, ne)
- `operation` (in, nin, eq, ne)
- `source` (in, nin, eq, ne)
- `status` (in, nin, eq, ne)
- `settlementStatus` (in, nin, eq, ne)
- `batchNumber` (nct, ct)
- `payaccountLastfour` (nct, ct)
- `payaccountType` (ne, eq, in, nin)
- `customerFirstname` (ct, nct, eq, ne)
- `customerLastname` (ct, nct, eq, ne)
- `customerName` (ct, nct)
- `customerId` (eq, ne)
- `customerNumber` (ct, nct, eq, ne)
- `customerCompanyname` (ct, nct, eq, ne)
- `customerAddress` (ct, nct, eq, ne)
- `customerCity` (ct, nct, eq, ne)
- `customerZip` (ct, nct, eq, ne)
- `customerState` (ct, nct, eq, ne)
- `customerCountry` (ct, nct, eq, ne)
- `customerPhone` (ct, nct, eq, ne)
- `customerEmail` (ct, nct, eq, ne)
- `customerShippingAddress` (ct, nct, eq, ne)
- `customerShippingCity` (ct, nct, eq, ne)
- `customerShippingZip` (ct, nct, eq, ne)
- `customerShippingState` (ct, nct, eq, ne)
- `customerShippingCountry` (ct, nct, eq, ne)
- `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name

List of comparison accepted - enclosed between parentheses:
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array
- nin => not inside array

List of parameters accepted:
- limitRecord : max number of records for query (default="20", "0" or negative value for all)
- fromRecord : initial record in query

Example: `netAmount(gt)=20` returns all records with a `netAmount` greater than 20.00
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_transfer_details</a>(format: ExportFormat1, entry: String, transfer_id: String, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export a list of transfer details for an entrypoint. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat1;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .export
        .export_transfer_details(
            &ExportFormat1::Csv,
            &"8cfec329267".to_string(),
            1000000,
            &ExportTransferDetailsQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                sort_by: Some("desc(field_name)".to_string()),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `ExportFormat1` — Format for the export, either XLSX or CSV. 
    
</dd>
</dl>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**transfer_id:** `String` — Transfer identifier.
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help. 

List of field names accepted:

  - `grossAmount` (gt, ge, lt, le, eq, ne)

  - `chargeBackAmount` (gt, ge, lt, le, eq, ne)

  - `returnedAmount` (gt, ge, lt, le, eq, ne)

  - `billingFeeAmount` (gt, ge, lt, le, eq, ne)

  - `thirdPartyPaidAmount` (gt, ge, lt, le, eq, ne)

  - `netFundedAmount` (gt, ge, lt, le, eq, ne)

  - `adjustmentAmount` (gt, ge, lt, le, eq, ne)

  - `transactionId` (eq, ne, in, nin)

  - `category` (eq, ne, ct, nct)

  - `type` (eq, ne, in, nin)

  - `method` (eq, ne, in, nin)
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_transfers</a>(entry: String, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a list of transfers for an entrypoint. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .export
        .export_transfers(
            &"8cfec329267".to_string(),
            &ExportTransfersQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                sort_by: Some("desc(field_name)".to_string()),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help. 

List of field names accepted:
  - `transferDate` (gt, ge, lt, le, eq, ne)

  - `grossAmount` (gt, ge, lt, le, eq, ne)

  - `chargeBackAmount` (gt, ge, lt, le, eq, ne)

  - `returnedAmount` (gt, ge, lt, le, eq, ne)

  - `billingFeeAmount` (gt, ge, lt, le, eq, ne)

  - `thirdPartyPaidAmount` (gt, ge, lt, le, eq, ne)

  - `netFundedAmount` (gt, ge, lt, le, eq, ne)

  - `adjustmentAmount` (gt, ge, lt, le, eq, ne)

  - `processor` (ne, eq, ct, nct)

  - `transferStatus` (ne, eq, in, nin)

  - `batchNumber` (ne, eq, ct, nct)

  - `batchId` (ne, eq, in, nin)
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_vendors</a>(format: ExportFormat1, entry: String, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export a list of vendors for an entrypoint. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat1;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .export
        .export_vendors(
            &ExportFormat1::Csv,
            &"8cfec329267".to_string(),
            &ExportVendorsQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `ExportFormat1` — Format for the export, either XLSX or CSV. 
    
</dd>
</dl>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query.

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

List of field names accepted:
- `method` (in, nin, eq, ne)
- `enrollmentStatus` (in, nin, eq, ne)
- `status` (in, nin, eq, ne)
- `vendorNumber` (ct, nct, eq, ne)
- `name` (ct, nct, eq, ne)
- `ein` (ct, nct, eq, ne)
- `phone` (ct, nct, eq, ne)
- `email` (ct, nct, eq, ne)
- `address` (ct, nct, eq, ne)
- `city` (ct, nct, eq, ne)
- `state` (ct, nct, eq, ne)
- `country` (ct, nct, eq, ne)
- `zip` (ct, nct, eq, ne)
- `mcc` (ct, nct, eq, ne)
- `locationCode` (ct, nct, eq, ne)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)

List of comparison accepted - enclosed between parentheses:
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array separated by "|"
- nin => not inside array separated by "|"

List of parameters accepted:
- limitRecord : max number of records for query (default="20", "0" or negative value for all)
- fromRecord : initial record in query

Example: `netAmount(gt)=20` returns all records with a `netAmount` greater than 20.00
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.export.<a href="/src/api/resources/export/client.rs">export_vendors_org</a>(format: ExportFormat1, org_id: i64, columns_export: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export a list of vendors for an organization. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat1;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .export
        .export_vendors_org(
            &ExportFormat1::Csv,
            123,
            &ExportVendorsOrgQueryRequest {
                columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
                from_record: Some(251),
                limit_record: Some(1000),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**format:** `ExportFormat1` — Format for the export, either XLSX or CSV. 
    
</dd>
</dl>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**columns_export:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query.

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

List of field names accepted:
- `method` (in, nin, eq, ne)
- `enrollmentStatus` (in, nin, eq, ne)
- `status` (in, nin, eq, ne)
- `vendorNumber` (ct, nct, eq, ne)
- `name` (ct, nct, eq, ne)
- `ein` (ct, nct, eq, ne)
- `phone` (ct, nct, eq, ne)
- `email` (ct, nct, eq, ne)
- `address` (ct, nct, eq, ne)
- `city` (ct, nct, eq, ne)
- `state` (ct, nct, eq, ne)
- `country` (ct, nct, eq, ne)
- `zip` (ct, nct, eq, ne)
- `mcc` (ct, nct, eq, ne)
- `locationCode` (ct, nct, eq, ne)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)

List of comparison accepted - enclosed between parentheses:
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array separated by "|"
- nin => not inside array separated by "|"

List of parameters accepted:
- limitRecord : max number of records for query (default="20", "0" or negative value for all)
- fromRecord : initial record in query

Example: `netAmount(gt)=20` returns all records with a `netAmount` greater than 20.00
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## HostedPaymentPages
<details><summary><code>client.hosted_payment_pages.<a href="/src/api/resources/hosted_payment_pages/client.rs">load_page</a>(entry: String, subdomain: String) -> Result<PayabliPages, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Loads all of a payment page's details including `pageIdentifier` and `validationCode`. This endpoint requires an `application` API token.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .hosted_payment_pages
        .load_page(
            &"8cfec329267".to_string(),
            &"pay-your-fees-1".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**subdomain:** `String` — Payment page identifier. The subdomain value is the last part of the payment page URL. For example, in`https://paypages-sandbox.payabli.com/513823dc10/pay-your-fees-1`, the subdomain is `pay-your-fees-1`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hosted_payment_pages.<a href="/src/api/resources/hosted_payment_pages/client.rs">new_page</a>(entry: String, request: PayabliPages) -> Result<PayabliApiResponse00Responsedatanonobject, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>


Creates a new payment page for a paypoint. 
Note: this operation doesn't create a new paypoint, just a payment page for an existing paypoint. Paypoints are created by the Payabli team when a boarding application is approved.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    AdditionalData, AmountElement, AutoElement, ButtonElement, ButtonElementSize, ContactElement,
    DisplayProperty, Element, Enabled, FileContent, FileContentFtype, Finishtype, FrequencyList,
    HeaderElement, IdempotencyKey, InvoiceElement, LabelElement, MethodElement,
    MethodElementSettings, MethodElementSettingsApplePay, MethodElementSettingsApplePayButtonStyle,
    MethodElementSettingsApplePayButtonType, MethodElementSettingsApplePayLanguage, MethodsList,
    NoteElement, Order, PageContent, PageElement, PageIdentifier, PageSetting, PayCategory,
    PayabliCredentials, PayabliPages, PayorElement, PayorFields, ReceiptContent, SettingElement,
    Subdomain,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hosted_payment_pages
        .new_page(
            &"8cfec329267".to_string(),
            &PayabliPages {
                additional_data: None,
                credentials: None,
                last_access: None,
                page_content: None,
                page_identifier: None,
                page_settings: None,
                published: None,
                receipt_content: None,
                subdomain: None,
                total_amount: None,
                validation_code: None,
            },
            Some(
                RequestOptions::new()
                    .additional_header("idempotencyKey", "6B29FC40-CA47-1067-B31D-00DD010662DA"),
            ),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.hosted_payment_pages.<a href="/src/api/resources/hosted_payment_pages/client.rs">save_page</a>(entry: String, subdomain: String, request: PayabliPages) -> Result<PayabliApiResponse00Responsedatanonobject, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates a payment page in a paypoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    AdditionalData, AmountElement, AutoElement, ButtonElement, ButtonElementSize, ContactElement,
    DisplayProperty, Element, Enabled, FileContent, FileContentFtype, Finishtype, FrequencyList,
    HeaderElement, InvoiceElement, LabelElement, MethodElement, MethodElementSettings,
    MethodElementSettingsApplePay, MethodElementSettingsApplePayButtonStyle,
    MethodElementSettingsApplePayButtonType, MethodElementSettingsApplePayLanguage, MethodsList,
    NoteElement, Order, PageContent, PageElement, PageIdentifier, PageSetting, PayCategory,
    PayabliCredentials, PayabliPages, PayorElement, PayorFields, ReceiptContent, SettingElement,
    Subdomain,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .hosted_payment_pages
        .save_page(
            &"8cfec329267".to_string(),
            &"pay-your-fees-1".to_string(),
            &PayabliPages {
                additional_data: None,
                credentials: None,
                last_access: None,
                page_content: None,
                page_identifier: None,
                page_settings: None,
                published: None,
                receipt_content: None,
                subdomain: None,
                total_amount: None,
                validation_code: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**subdomain:** `String` — Payment page identifier. The subdomain value is the last part of the payment page URL. For example, in`https://paypages-sandbox.payabli.com/513823dc10/pay-your-fees-1`, the subdomain is `pay-your-fees-1`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Import
<details><summary><code>client.import.<a href="/src/api/resources/import/client.rs">import_bills</a>(entry: String) -> Result<PayabliApiResponseImport, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Import a list of bills from a CSV file. See the [Import Guide](/developers/developer-guides/bills-add#import-bills) for more help and an example file.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .import
        .import_bills(
            &"8cfec329267".to_string(),
            &ImportBillsRequest {
                file: b"test file content".to_vec(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.import.<a href="/src/api/resources/import/client.rs">import_customer</a>(entry: Entrypointfield, replace_existing: Option<Option<i64>>) -> Result<PayabliApiResponseImport, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Import a list of customers from a CSV file. See the [Import Guide](/developers/developer-guides/entities-customers#import-customers) for more help and example files.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::Entrypointfield;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .import
        .import_customer(
            &Entrypointfield("8cfec329267".to_string()),
            &ImportCustomerRequest {
                file: b"test file content".to_vec(),
                replace_existing: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `Entrypointfield` 
    
</dd>
</dl>

<dl>
<dd>

**replace_existing:** `Option<i64>` — Flag indicating to replace existing customer with a new record. Possible values: 0 (do not replace), 1 (replace). Default is 0
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.import.<a href="/src/api/resources/import/client.rs">import_vendor</a>(entry: Entrypointfield) -> Result<PayabliApiResponseImport, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Import a list of vendors from a CSV file. See the [Import Guide](/developers/developer-guides/entities-vendors#import-vendors) for more help and example files.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::Entrypointfield;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .import
        .import_vendor(
            &Entrypointfield("8cfec329267".to_string()),
            &ImportVendorRequest {
                file: b"test file content".to_vec(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `Entrypointfield` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Invoice
<details><summary><code>client.invoice.<a href="/src/api/resources/invoice/client.rs">add_invoice</a>(entry: String, request: InvoiceDataRequest, force_customer_creation: Option<Option<ForceCustomerCreation>>) -> Result<InvoiceResponseWithoutData, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates an invoice in an entrypoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    AdditionalData, AdditionalDataString, Attachments, BillData, BillDataPaymentTerms, BillItem,
    BillOptions, BillingAddressAddtlNullable, BillingAddressNullable, BillingCityNullable,
    BillingCountryNullable, BillingStateNullable, BillingZip, CustomerId, CustomerNumberNullable,
    Datenullable, Discount, DutyAmount, Email, FileContent, FileContentFtype,
    ForceCustomerCreation, FreightAmount, Frequency, IdempotencyKey, Identifierfields,
    InvoiceAmount, InvoiceDataRequest, InvoiceNumber, InvoiceType, Invoicestatus,
    ItemCommodityCode, ItemDescription, ItemProductCode, ItemProductName, ItemUnitofMeasure,
    PayorDataRequest, PhoneNumber, PurchaseOrder, ShippingFromZip, Shippingaddress,
    Shippingaddressadditional, Shippingcity, Shippingcountry, Shippingstate, Shippingzip,
    SummaryCommodityCode, Tax, TermsConditions,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .invoice
        .add_invoice(
            &"8cfec329267".to_string(),
            &AddInvoiceRequest {
                body: InvoiceDataRequest {
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
                        customer_id: None,
                        customer_number: Some(CustomerNumberNullable("3".to_string())),
                        first_name: Some("Tamara".to_string()),
                        identifier_fields: None,
                        last_name: Some("Bagratoni".to_string()),
                        shipping_address_1: None,
                        shipping_address_2: None,
                        shipping_city: None,
                        shipping_country: None,
                        shipping_state: None,
                        shipping_zip: None,
                    }),
                    invoice_data: Some(BillData {
                        additional_data: None,
                        attachments: None,
                        company: None,
                        discount: Some(Discount(Some(10.0))),
                        duty_amount: None,
                        first_name: None,
                        freight_amount: None,
                        frequency: Some(Frequency::OneTime),
                        invoice_amount: Some(InvoiceAmount(982.37)),
                        invoice_date: Some(Datenullable(Some(
                            NaiveDate::parse_from_str("2025-10-19", "%Y-%m-%d").unwrap(),
                        ))),
                        invoice_due_date: None,
                        invoice_end_date: None,
                        invoice_number: Some(InvoiceNumber("INV-3".to_string())),
                        invoice_status: Some(Invoicestatus(1)),
                        invoice_type: Some(InvoiceType(0)),
                        items: Some(vec![
                            BillItem {
                                item_categories: None,
                                item_commodity_code: None,
                                item_cost: 100.0,
                                item_description: Some(ItemDescription(
                                    "Consultation for Georgian tours".to_string(),
                                )),
                                item_mode: Some(1),
                                item_product_code: None,
                                item_product_name: Some(ItemProductName(
                                    "Adventure Consult".to_string(),
                                )),
                                item_qty: Some(1),
                                item_tax_amount: None,
                                item_tax_rate: None,
                                item_total_amount: Some(1.0),
                                item_unit_of_measure: None,
                            },
                            BillItem {
                                item_categories: None,
                                item_commodity_code: None,
                                item_cost: 882.37,
                                item_description: Some(ItemDescription(
                                    "Deposit for trip planning".to_string(),
                                )),
                                item_mode: None,
                                item_product_code: None,
                                item_product_name: Some(ItemProductName("Deposit ".to_string())),
                                item_qty: Some(1),
                                item_tax_amount: None,
                                item_tax_rate: None,
                                item_total_amount: Some(1.0),
                                item_unit_of_measure: None,
                            },
                        ]),
                        last_name: None,
                        notes: None,
                        payment_terms: None,
                        purchase_order: None,
                        shipping_address_1: None,
                        shipping_address_2: None,
                        shipping_city: None,
                        shipping_country: None,
                        shipping_email: None,
                        shipping_from_zip: None,
                        shipping_phone: None,
                        shipping_state: None,
                        shipping_zip: None,
                        summary_commodity_code: None,
                        tax: None,
                        terms_conditions: None,
                    }),
                    scheduled_options: None,
                },
                force_customer_creation: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**force_customer_creation:** `Option<ForceCustomerCreation>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.invoice.<a href="/src/api/resources/invoice/client.rs">delete_attached_from_invoice</a>(id_invoice: i64, filename: String) -> Result<InvoiceResponseWithoutData, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes an invoice that's attached to a file.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .invoice
        .delete_attached_from_invoice(23548884, &"0_Bill.pdf".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id_invoice:** `i64` — Invoice ID
    
</dd>
</dl>

<dl>
<dd>

**filename:** `String` 

The filename in Payabli. Filename is `zipName` in response to a request to `/api/Invoice/{idInvoice}`. Here, the filename is `0_Bill.pdf``. 
"DocumentsRef": {
  "zipfile": "inva_269.zip",
  "filelist": [
    {
      "originalName": "Bill.pdf",
      "zipName": "0_Bill.pdf",
      "descriptor": null
    }
  ]
}
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.invoice.<a href="/src/api/resources/invoice/client.rs">delete_invoice</a>(id_invoice: i64) -> Result<InvoiceResponseWithoutData, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes a single invoice from an entrypoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.invoice.delete_invoice(23548884, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id_invoice:** `i64` — Invoice ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.invoice.<a href="/src/api/resources/invoice/client.rs">edit_invoice</a>(id_invoice: i64, request: InvoiceDataRequest, force_customer_creation: Option<Option<bool>>) -> Result<InvoiceResponseWithoutData, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates details for a single invoice in an entrypoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    AdditionalData, AdditionalDataString, Attachments, BillData, BillDataPaymentTerms, BillItem,
    BillOptions, BillingAddressAddtlNullable, BillingAddressNullable, BillingCityNullable,
    BillingCountryNullable, BillingStateNullable, BillingZip, CustomerId, CustomerNumberNullable,
    Datenullable, Discount, DutyAmount, Email, FileContent, FileContentFtype, FreightAmount,
    Frequency, Identifierfields, InvoiceAmount, InvoiceDataRequest, InvoiceNumber, InvoiceType,
    Invoicestatus, ItemCommodityCode, ItemDescription, ItemProductCode, ItemProductName,
    ItemUnitofMeasure, PayorDataRequest, PhoneNumber, PurchaseOrder, ShippingFromZip,
    Shippingaddress, Shippingaddressadditional, Shippingcity, Shippingcountry, Shippingstate,
    Shippingzip, SummaryCommodityCode, Tax, TermsConditions,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .invoice
        .edit_invoice(
            332,
            &EditInvoiceRequest {
                body: InvoiceDataRequest {
                    customer_data: None,
                    invoice_data: Some(BillData {
                        additional_data: None,
                        attachments: None,
                        company: None,
                        discount: None,
                        duty_amount: None,
                        first_name: None,
                        freight_amount: None,
                        frequency: None,
                        invoice_amount: Some(InvoiceAmount(982.37)),
                        invoice_date: Some(Datenullable(Some(
                            NaiveDate::parse_from_str("2025-10-19", "%Y-%m-%d").unwrap(),
                        ))),
                        invoice_due_date: None,
                        invoice_end_date: None,
                        invoice_number: Some(InvoiceNumber("INV-6".to_string())),
                        invoice_status: None,
                        invoice_type: None,
                        items: Some(vec![BillItem {
                            item_categories: None,
                            item_commodity_code: None,
                            item_cost: 882.37,
                            item_description: Some(ItemDescription(
                                "Deposit for trip planning".to_string(),
                            )),
                            item_mode: None,
                            item_product_code: None,
                            item_product_name: Some(ItemProductName("Deposit".to_string())),
                            item_qty: Some(1),
                            item_tax_amount: None,
                            item_tax_rate: None,
                            item_total_amount: None,
                            item_unit_of_measure: None,
                        }]),
                        last_name: None,
                        notes: None,
                        payment_terms: None,
                        purchase_order: None,
                        shipping_address_1: None,
                        shipping_address_2: None,
                        shipping_city: None,
                        shipping_country: None,
                        shipping_email: None,
                        shipping_from_zip: None,
                        shipping_phone: None,
                        shipping_state: None,
                        shipping_zip: None,
                        summary_commodity_code: None,
                        tax: None,
                        terms_conditions: None,
                    }),
                    scheduled_options: None,
                },
                force_customer_creation: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id_invoice:** `i64` — Invoice ID
    
</dd>
</dl>

<dl>
<dd>

**force_customer_creation:** `Option<bool>` — When `true`, the request creates a new customer record, regardless of whether customer identifiers match an existing customer.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.invoice.<a href="/src/api/resources/invoice/client.rs">get_attached_file_from_invoice</a>(id_invoice: i64, filename: String, return_object: Option<Option<bool>>) -> Result<FileContent, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a file attached to an invoice.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .invoice
        .get_attached_file_from_invoice(
            1,
            &"filename".to_string(),
            &GetAttachedFileFromInvoiceQueryRequest {
                return_object: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id_invoice:** `i64` — Invoice ID
    
</dd>
</dl>

<dl>
<dd>

**filename:** `String` 

The filename in Payabli. Filename is `zipName` in the response to a request to `/api/Invoice/{idInvoice}`. Here, the filename is `0_Bill.pdf``. 
```
  "DocumentsRef": {
    "zipfile": "inva_269.zip",
    "filelist": [
      {
        "originalName": "Bill.pdf",
        "zipName": "0_Bill.pdf",
        "descriptor": null
      }
    ]
  }
  ```
    
</dd>
</dl>

<dl>
<dd>

**return_object:** `Option<bool>` — When `true`, the request returns the file content as a Base64-encoded string.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.invoice.<a href="/src/api/resources/invoice/client.rs">get_invoice</a>(id_invoice: i64) -> Result<GetInvoiceRecord, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a single invoice by ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.invoice.get_invoice(23548884, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id_invoice:** `i64` — Invoice ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.invoice.<a href="/src/api/resources/invoice/client.rs">get_invoice_number</a>(entry: String) -> Result<InvoiceNumberResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the next available invoice number for a paypoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .invoice
        .get_invoice_number(&"8cfec329267".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.invoice.<a href="/src/api/resources/invoice/client.rs">list_invoices</a>(entry: String, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryInvoiceResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a list of invoices for an entrypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .invoice
        .list_invoices(
            &"8cfec329267".to_string(),
            &ListInvoicesQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

List of field names accepted:

- `invoiceDate` (gt, ge, lt, le, eq, ne)
- `dueDate` (gt, ge, lt, le, eq, ne)
- `sentDate` (gt, ge, lt, le, eq, ne)
- `frequency` (in, nin,ne, eq)
- `invoiceType` (eq, ne)
- `payTerms` (in, nin, eq, ne)
- `paypointId` (ne, eq)
- `totalAmount` (gt, ge, lt, le, eq, ne)
- `paidAmount` (gt, ge, lt, le, eq, ne)
- `status` (in, nin, eq, ne)
- `invoiceNumber` (ct, nct, eq, ne)
- `purchaseOrder` (ct, nct, eq, ne)
- `itemProductCode` (ct, nct)
- `itemDescription` (ct, nct)
- `customerFirstname` (ct, nct, eq, ne)
- `customerLastname` (ct, nct, eq, ne)
- `customerName` (ct, nct)
- `customerId` (eq, ne)
- `customerNumber` (ct, nct, eq, ne)
- `customerCompanyname` (ct, nct, eq, ne)
- `customerAddress` (ct, nct, eq, ne)
- `customerCity` (ct, nct, eq, ne)
- `customerZip` (ct, nct, eq, ne)
- `customerState` (ct, nct, eq, ne)
- `customerCountry` (ct, nct, eq, ne)
- `customerPhone` (ct, nct, eq, ne)
- `customerEmail` (ct, nct, eq, ne)
- `customerShippingAddress` (ct, nct, eq, ne)
- `customerShippingCity` (ct, nct, eq, ne)
- `customerShippingZip` (ct, nct, eq, ne)
- `customerShippingState` (ct, nct, eq, ne)
- `customerShippingCountry` (ct, nct, eq, ne)
- `orgId` (eq)
- `paylinkId` (ne, eq)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)
- `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name

List of comparison accepted - enclosed between parentheses:
  
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array
- nin => not inside array

List of parameters accepted:
- limitRecord : max number of records for query (default="20", "0" or negative value for all)
- fromRecord : initial record in query

Example: totalAmount(gt)=20 return all records with totalAmount greater than 20.00
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.invoice.<a href="/src/api/resources/invoice/client.rs">list_invoices_org</a>(org_id: i64, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryInvoiceResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a list of invoices for an org. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .invoice
        .list_invoices_org(
            123,
            &ListInvoicesOrgQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

List of field names accepted:

- `invoiceDate` (gt, ge, lt, le, eq, ne)
- `dueDate` (gt, ge, lt, le, eq, ne)
- `sentDate` (gt, ge, lt, le, eq, ne)
- `frequency` (in, nin,ne, eq)
- `invoiceType` (eq, ne)
- `payTerms` (in, nin, eq, ne)
- `paypointId` (ne, eq)
- `totalAmount` (gt, ge, lt, le, eq, ne)
- `paidAmount` (gt, ge, lt, le, eq, ne)
- `status` (in, nin, eq, ne)
- `invoiceNumber` (ct, nct, eq, ne)
- `purchaseOrder` (ct, nct, eq, ne)
- `itemProductCode` (ct, nct)
- `itemDescription` (ct, nct)
- `customerFirstname` (ct, nct, eq, ne)
- `customerLastname` (ct, nct, eq, ne)
- `customerName` (ct, nct)
- `customerId` (eq, ne)
- `customerNumber` (ct, nct, eq, ne)
- `customerCompanyname` (ct, nct, eq, ne)
- `customerAddress` (ct, nct, eq, ne)
- `customerCity` (ct, nct, eq, ne)
- `customerZip` (ct, nct, eq, ne)
- `customerState` (ct, nct, eq, ne)
- `customerCountry` (ct, nct, eq, ne)
- `customerPhone` (ct, nct, eq, ne)
- `customerEmail` (ct, nct, eq, ne)
- `customerShippingAddress` (ct, nct, eq, ne)
- `customerShippingCity` (ct, nct, eq, ne)
- `customerShippingZip` (ct, nct, eq, ne)
- `customerShippingState` (ct, nct, eq, ne)
- `customerShippingCountry` (ct, nct, eq, ne)
- `orgId` (eq)
- `paylinkId` (ne, eq)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)
- `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name

List of comparison accepted - enclosed between parentheses:

- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array
- nin => not inside array

List of parameters accepted:
- limitRecord : max number of records for query (default="20", "0" or negative value for all)
- fromRecord : initial record in query

Example: totalAmount(gt)=20 return all records with totalAmount greater than 20.00
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.invoice.<a href="/src/api/resources/invoice/client.rs">send_invoice</a>(id_invoice: i64, attachfile: Option<Option<bool>>, mail_2: Option<Option<String>>) -> Result<SendInvoiceResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Sends an invoice from an entrypoint via email.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .invoice
        .send_invoice(
            23548884,
            &SendInvoiceQueryRequest {
                attachfile: Some(true),
                mail_2: Some("tamara@example.com".to_string()),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id_invoice:** `i64` — Invoice ID
    
</dd>
</dl>

<dl>
<dd>

**attachfile:** `Option<bool>` — When `true`, attaches a PDF version of invoice to the email.
    
</dd>
</dl>

<dl>
<dd>

**mail_2:** `Option<String>` — Email address where the invoice will be sent to. If this parameter isn't included, Payabli uses the email address on file for the customer owner of the invoice.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.invoice.<a href="/src/api/resources/invoice/client.rs">get_invoice_pdf</a>(id_invoice: i64) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Export a single invoice in PDF format.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.invoice.get_invoice_pdf(23548884, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id_invoice:** `i64` — Invoice ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## LineItem
<details><summary><code>client.line_item.<a href="/src/api/resources/line_item/client.rs">add_item</a>(entry: String, request: LineItem) -> Result<PayabliApiResponse6, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Adds products and services to an entrypoint's catalog. These are used as line items for invoicing and transactions. In the response, "responseData" displays the item's code.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    ItemCommodityCode, ItemDescription, ItemProductCode, ItemProductName, ItemUnitofMeasure,
    LineItem,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .line_item
        .add_item(
            &"47cae3d74".to_string(),
            &LineItem {
                item_categories: None,
                item_commodity_code: Some(ItemCommodityCode("010".to_string())),
                item_cost: 12.45,
                item_description: Some(ItemDescription("Deposit for materials".to_string())),
                item_mode: Some(0),
                item_product_code: Some(ItemProductCode("M-DEPOSIT".to_string())),
                item_product_name: Some(ItemProductName("Materials deposit".to_string())),
                item_qty: 1,
                item_unit_of_measure: Some(ItemUnitofMeasure("SqFt".to_string())),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.line_item.<a href="/src/api/resources/line_item/client.rs">delete_item</a>(line_item_id: i64) -> Result<DeleteItemResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes an item.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.line_item.delete_item(700, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**line_item_id:** `i64` — ID for the line item (also known as a product, service, or item).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.line_item.<a href="/src/api/resources/line_item/client.rs">get_item</a>(line_item_id: i64) -> Result<LineItemQueryRecord, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Gets an item by ID. 
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.line_item.get_item(700, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**line_item_id:** `i64` — ID for the line item (also known as a product, service, or item).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.line_item.<a href="/src/api/resources/line_item/client.rs">list_line_items</a>(entry: String, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryResponseItems, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a list of line items and their details from an entrypoint. Line items are also known as items, products, and services. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .line_item
        .list_line_items(
            &"8cfec329267".to_string(),
            &ListLineItemsQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 


Collection of field names, conditions, and values used to filter the query
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20

</Info>
See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

List of field names accepted:

  - `categories` (ct, nct)
  - `code` (ne, eq, ct, nct)
  - `commodityCode` (ne, eq, ct, nct)
  - `createdDate` (gt, ge, lt, le, eq, ne)
  - `description` (ne, eq, ct, nct)
  - `externalPaypointID` (ct, nct, ne, eq)
  - `mode` (eq, ne)
  - `name` (ne, eq, ct, nct)
  - `orgName` (ne, eq, ct, nct)
  - `paypointDba` (ne, eq, ct, nct)
  - `paypointId` (ne, eq)
  - `paypointLegal` (ne, eq, ct, nct)
  - `quantity` (gt, ge, lt, le, eq, ne)
  - `uom` (ne, eq, ct, nct)
  - `updatedDate` (gt, ge, lt, le, eq, ne)
  - `value` (gt, ge, lt, le, eq, ne)

List of comparison accepted - enclosed between parentheses:

- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array separated by "|"
- nin => not inside array separated by "|"

List of parameters accepted:
- limitRecord : max number of records for query (default="20", "0" or negative value for all)
- fromRecord : initial record in query

Example: name(ct)=john return all records with name containing john
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.line_item.<a href="/src/api/resources/line_item/client.rs">update_item</a>(line_item_id: i64, request: LineItem) -> Result<PayabliApiResponse6, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates an item.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    ItemCommodityCode, ItemDescription, ItemProductCode, ItemProductName, ItemUnitofMeasure,
    LineItem,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .line_item
        .update_item(
            700,
            &LineItem {
                item_categories: None,
                item_commodity_code: None,
                item_cost: 12.45,
                item_description: None,
                item_mode: None,
                item_product_code: None,
                item_product_name: None,
                item_qty: 1,
                item_unit_of_measure: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**line_item_id:** `i64` — ID for the line item (also known as a product, service, or item).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## MoneyIn
<details><summary><code>client.money_in.<a href="/src/api/resources/money_in/client.rs">authorize</a>(request: TransRequestBody, force_customer_creation: Option<Option<ForceCustomerCreation>>) -> Result<AuthResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Authorize a card transaction. This returns an authorization code and reserves funds for the merchant. Authorized transactions aren't flagged for settlement until [captured](/api-reference/moneyin/capture-an-authorized-transaction).
Only card transactions can be authorized. This endpoint can't be used for ACH transactions.
<Tip>
  Consider migrating to the [v2 Authorize endpoint](/developers/api-reference/moneyinV2/authorize-a-transaction) to take advantage of unified response codes and improved response consistency.
</Tip>
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    Accountid, AchHolder, AchHolderType, AchSecCode, Achaccount, Achaccounttype, Achrouting,
    AdditionalData, AdditionalDataString, Attachments, BillData, BillDataPaymentTerms, BillItem,
    BillingAddressAddtlNullable, BillingAddressNullable, BillingCityNullable,
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
        .authorize(
            &AuthorizeRequest {
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
                force_customer_creation: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**force_customer_creation:** `Option<ForceCustomerCreation>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.money_in.<a href="/src/api/resources/money_in/client.rs">capture</a>(trans_id: String, amount: f64) -> Result<CaptureResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

<Warning>
  This endpoint is deprecated and will be sunset on November 24, 2025. Migrate to [POST `/capture/{transId}`](/api-reference/moneyin/capture-an-authorized-transaction)`.
</Warning>
  
  Capture an [authorized
transaction](/api-reference/moneyin/authorize-a-transaction) to complete the transaction and move funds from the customer to merchant account.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .capture(
            &"10-7d9cd67d-2d5d-4cd7-a1b7-72b8b201ec13".to_string(),
            0.0,
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**trans_id:** `String` — ReferenceId for the transaction (PaymentId).
    
</dd>
</dl>

<dl>
<dd>

**amount:** `f64` — Amount to be captured. The amount can't be greater the original total amount of the transaction. `0` captures the total amount authorized in the transaction. Partial captures aren't supported.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.money_in.<a href="/src/api/resources/money_in/client.rs">capture_auth</a>(trans_id: String, request: CaptureRequest) -> Result<CaptureResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Capture an [authorized transaction](/api-reference/moneyin/authorize-a-transaction) to complete the transaction and move funds from the customer to merchant account. 

You can use this endpoint to capture both full and partial amounts of the original authorized transaction. See [Capture an authorized transaction](/developers/developer-guides/pay-in-auth-and-capture) for more information about this endpoint.

<Tip>
Consider migrating to the [v2 Capture endpoint](/developers/api-reference/moneyinV2/capture-an-authorized-transaction) to take advantage of unified response codes and improved response consistency.
</Tip>
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{CapturePaymentDetails, CaptureRequest};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .money_in
        .capture_auth(
            &"10-7d9cd67d-2d5d-4cd7-a1b7-72b8b201ec13".to_string(),
            &CaptureRequest {
                payment_details: CapturePaymentDetails {
                    total_amount: 105.0,
                    service_fee: Some(5.0),
                },
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**trans_id:** `String` — ReferenceId for the transaction (PaymentId).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.money_in.<a href="/src/api/resources/money_in/client.rs">credit</a>(request: RequestCredit, force_customer_creation: Option<Option<ForceCustomerCreation>>) -> Result<PayabliApiResponse0, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Make a temporary microdeposit in a customer account to verify the customer's ownership and access to the target account. Reverse the microdeposit with `reverseCredit`.

This feature must be enabled by Payabli on a per-merchant basis. Contact support for help. 
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    Accountid, AchHolder, AchSecCode, Achaccount, Achaccounttype, Achrouting, AdditionalData,
    BillingAddressAddtlNullable, BillingAddressNullable, BillingCityNullable,
    BillingCountryNullable, BillingStateNullable, BillingZip, CustomerId, CustomerNumberNullable,
    Email, Entrypointfield, ForceCustomerCreation, IdempotencyKey, Identifierfields, OrderId,
    Orderdescription, PaymentDetailCredit, PayorDataRequest, PhoneNumber,
    RequestCreditPaymentMethod, Shippingaddress, Shippingaddressadditional, Shippingcity,
    Shippingcountry, Shippingstate, Shippingzip, Source, Subdomain,
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
        .credit(
            &RequestCredit {
                customer_data: PayorDataRequest {
                    additional_data: None,
                    billing_address_1: Some(BillingAddressNullable(
                        "5127 Linkwood ave".to_string(),
                    )),
                    billing_address_2: None,
                    billing_city: None,
                    billing_country: None,
                    billing_email: None,
                    billing_phone: None,
                    billing_state: None,
                    billing_zip: None,
                    company: None,
                    customer_id: None,
                    customer_number: Some(CustomerNumberNullable("100".to_string())),
                    first_name: None,
                    identifier_fields: None,
                    last_name: None,
                    shipping_address_1: None,
                    shipping_address_2: None,
                    shipping_city: None,
                    shipping_country: None,
                    shipping_state: None,
                    shipping_zip: None,
                },
                entrypoint: Some(Entrypointfield("my-entrypoint".to_string())),
                payment_details: PaymentDetailCredit {
                    currency: None,
                    service_fee: Some(0.0),
                    total_amount: 1.0,
                },
                payment_method: RequestCreditPaymentMethod {
                    ach_account: Some(Achaccount("88354454".to_string())),
                    ach_account_type: Some(Achaccounttype::Checking),
                    ach_code: None,
                    ach_holder: Some(AchHolder("John Smith".to_string())),
                    ach_routing: Some(Achrouting("021000021".to_string())),
                    method: "ach".to_string(),
                },
                force_customer_creation: None,
                account_id: None,
                order_description: None,
                order_id: None,
                source: None,
                subdomain: None,
            },
            Some(
                RequestOptions::new()
                    .additional_header("idempotencyKey", "6B29FC40-CA47-1067-B31D-00DD010662DA"),
            ),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<Accountid>` 
    
</dd>
</dl>

<dl>
<dd>

**customer_data:** `PayorDataRequest` — Object describing the customer/payor.
    
</dd>
</dl>

<dl>
<dd>

**entrypoint:** `Option<Entrypointfield>` 
    
</dd>
</dl>

<dl>
<dd>

**order_description:** `Option<Orderdescription>` 
    
</dd>
</dl>

<dl>
<dd>

**order_id:** `Option<OrderId>` 
    
</dd>
</dl>

<dl>
<dd>

**payment_details:** `PaymentDetailCredit` 
    
</dd>
</dl>

<dl>
<dd>

**payment_method:** `RequestCreditPaymentMethod` — Object describing the ACH payment method to use for transaction.
    
</dd>
</dl>

<dl>
<dd>

**source:** `Option<Source>` 
    
</dd>
</dl>

<dl>
<dd>

**subdomain:** `Option<Subdomain>` 
    
</dd>
</dl>

<dl>
<dd>

**force_customer_creation:** `Option<ForceCustomerCreation>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.money_in.<a href="/src/api/resources/money_in/client.rs">details</a>(trans_id: String) -> Result<TransactionQueryRecordsCustomer, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a processed transaction's details.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .details(&"45-as456777hhhhhhhhhh77777777-324".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**trans_id:** `String` — ReferenceId for the transaction (PaymentId).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.money_in.<a href="/src/api/resources/money_in/client.rs">getpaid</a>(request: TransRequestBody, ach_validation: Option<Option<AchValidation>>, force_customer_creation: Option<Option<ForceCustomerCreation>>, include_details: Option<Option<bool>>) -> Result<PayabliApiResponseGetPaid, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Make a single transaction. This method authorizes and captures a payment in one step.

  <Tip>
  Consider migrating to the [v2 Make a transaction endpoint](/developers/api-reference/moneyinV2/make-a-transaction) to take advantage of unified response codes and improved response consistency.
  </Tip>
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**ach_validation:** `Option<AchValidation>` 
    
</dd>
</dl>

<dl>
<dd>

**force_customer_creation:** `Option<ForceCustomerCreation>` 
    
</dd>
</dl>

<dl>
<dd>

**include_details:** `Option<bool>` — When `true`, transactionDetails object is returned in the response. See a full example of the `transactionDetails` object in the [Transaction integration guide](/developers/developer-guides/money-in-transaction-add#includedetailstrue-response).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.money_in.<a href="/src/api/resources/money_in/client.rs">reverse</a>(trans_id: String, amount: f64) -> Result<ReverseResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

A reversal either refunds or voids a transaction independent of the transaction's settlement status. Send a reversal request for a transaction, and Payabli automatically determines whether it's a refund or void. You don't need to know whether the transaction is settled or not.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .reverse(
            &"10-3ffa27df-b171-44e0-b251-e95fbfc7a723".to_string(),
            0.0,
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**trans_id:** `String` — ReferenceId for the transaction (PaymentId).
    
</dd>
</dl>

<dl>
<dd>

**amount:** `f64` 


Amount to reverse from original transaction, minus any service fees charged on the original transaction.

The amount provided can't be greater than the original total amount of the transaction, minus service fees. For example, if a transaction was $90 plus a $10 service fee, you can reverse up to $90. 

An amount equal to zero will refunds the total amount authorized minus any service fee.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.money_in.<a href="/src/api/resources/money_in/client.rs">refund</a>(trans_id: String, amount: f64) -> Result<RefundResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Refund a transaction that has settled and send money back to the account holder. If a transaction hasn't been settled, void it instead.

  <Tip>
  Consider migrating to the [v2 Refund endpoint](/developers/api-reference/moneyinV2/refund-a-settled-transaction) to take advantage of unified response codes and improved response consistency.
  </Tip>
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .refund(
            &"10-3ffa27df-b171-44e0-b251-e95fbfc7a723".to_string(),
            0.0,
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**trans_id:** `String` — ReferenceId for the transaction (PaymentId).
    
</dd>
</dl>

<dl>
<dd>

**amount:** `f64` 


Amount to refund from original transaction, minus any service fees charged on the original transaction. 

The amount provided can't be greater than the original total amount of the transaction, minus service fees. For example, if a transaction was \$90 plus a \$10 service fee, you can refund up to \$90.

An amount equal to zero will refund the total amount authorized minus any service fee.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.money_in.<a href="/src/api/resources/money_in/client.rs">refund_with_instructions</a>(trans_id: String, request: RequestRefund) -> Result<RefundWithInstructionsResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Refunds a settled transaction with split instructions.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    IdempotencyKey, IpAddress, OrderId, Orderdescription, PaymentCategories, RefundDetail, Source,
    SplitFundingRefundContent,
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
        .refund_with_instructions(
            &"10-3ffa27df-b171-44e0-b251-e95fbfc7a723".to_string(),
            &RequestRefund {
                amount: Some(100.0),
                order_description: Some(Orderdescription("Materials deposit".to_string())),
                refund_details: Some(RefundDetail {
                    categories: None,
                    split_refunding: Some(vec![
                        SplitFundingRefundContent {
                            account_id: Some("187-342".to_string()),
                            amount: Some(60.0),
                            description: Some("Refunding undelivered materials".to_string()),
                            origination_entry_point: Some("7f1a381696".to_string()),
                        },
                        SplitFundingRefundContent {
                            account_id: Some("187-343".to_string()),
                            amount: Some(40.0),
                            description: Some(
                                "Refunding deposit for undelivered materials".to_string(),
                            ),
                            origination_entry_point: Some("7f1a381696".to_string()),
                        },
                    ]),
                }),
                source: Some(Source("api".to_string())),
                ipaddress: None,
                order_id: None,
            },
            Some(
                RequestOptions::new()
                    .additional_header("idempotencyKey", "8A29FC40-CA47-1067-B31D-00DD010662DB"),
            ),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**trans_id:** `String` — ReferenceId for the transaction (PaymentId).
    
</dd>
</dl>

<dl>
<dd>

**amount:** `Option<f64>` 


Amount to refund from original transaction, minus any service fees charged on the original transaction. 

The amount provided can't be greater than the original total amount of the transaction, minus service fees. For example, if a transaction was $90 plus a $10 service fee, you can refund up to $90. 

An amount equal to zero will refund the total amount authorized minus any service fee.
    
</dd>
</dl>

<dl>
<dd>

**ipaddress:** `Option<IpAddress>` 
    
</dd>
</dl>

<dl>
<dd>

**order_description:** `Option<Orderdescription>` 
    
</dd>
</dl>

<dl>
<dd>

**order_id:** `Option<OrderId>` 
    
</dd>
</dl>

<dl>
<dd>

**refund_details:** `Option<RefundDetail>` 
    
</dd>
</dl>

<dl>
<dd>

**source:** `Option<Source>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.money_in.<a href="/src/api/resources/money_in/client.rs">reverse_credit</a>(trans_id: String) -> Result<PayabliApiResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Reverse microdeposits that are used to verify customer account ownership and access. The `transId` value is returned in the success response for the original credit transaction made with `api/MoneyIn/makecredit`.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .reverse_credit(&"45-as456777hhhhhhhhhh77777777-324".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**trans_id:** `String` — ReferenceId for the transaction (PaymentId).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.money_in.<a href="/src/api/resources/money_in/client.rs">send_receipt_2_trans</a>(trans_id: String, email: Option<Option<String>>) -> Result<ReceiptResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Send a payment receipt for a transaction.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .send_receipt_2_trans(
            &"45-as456777hhhhhhhhhh77777777-324".to_string(),
            &SendReceipt2TransQueryRequest {
                email: Some("example@email.com".to_string()),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**trans_id:** `String` — ReferenceId for the transaction (PaymentId).
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<String>` 

Email address where the payment receipt should be sent. 

If not provided, the email address on file for the user owner of the transaction is used.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.money_in.<a href="/src/api/resources/money_in/client.rs">validate</a>(request: RequestPaymentValidate) -> Result<ValidateResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Validates a card number without running a transaction or authorizing a charge.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    Accountid, Cardexp, Cardholder, Cardnumber, Cardzip, Entrypointfield, IdempotencyKey, OrderId,
    Orderdescription, RequestPaymentValidatePaymentMethod,
    RequestPaymentValidatePaymentMethodMethod,
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
        .validate(
            &RequestPaymentValidate {
                entry_point: Entrypointfield("entry132".to_string()),
                payment_method: RequestPaymentValidatePaymentMethod {
                    method: RequestPaymentValidatePaymentMethodMethod::Card,
                    cardnumber: Cardnumber("4360000001000005".to_string()),
                    cardexp: Cardexp("12/29".to_string()),
                    cardzip: Cardzip("14602-8328".to_string()),
                    card_holder: Cardholder("Dianne Becker-Smith".to_string()),
                },
                account_id: None,
                order_description: None,
                order_id: None,
            },
            Some(
                RequestOptions::new()
                    .additional_header("idempotencyKey", "6B29FC40-CA47-1067-B31D-00DD010662DA"),
            ),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**account_id:** `Option<Accountid>` 
    
</dd>
</dl>

<dl>
<dd>

**entry_point:** `Entrypointfield` 
    
</dd>
</dl>

<dl>
<dd>

**order_description:** `Option<Orderdescription>` 
    
</dd>
</dl>

<dl>
<dd>

**order_id:** `Option<OrderId>` 
    
</dd>
</dl>

<dl>
<dd>

**payment_method:** `RequestPaymentValidatePaymentMethod` — Object describing payment method to use for transaction.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.money_in.<a href="/src/api/resources/money_in/client.rs">void</a>(trans_id: String) -> Result<VoidResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Cancel a transaction that hasn't been settled yet. Voiding non-captured authorizations prevents future captures. If a transaction has been settled, refund it instead.

  <Tip>
  Consider migrating to the [v2 Void endpoint](/developers/api-reference/moneyinV2/void-a-transaction) to take advantage of unified response codes and improved response consistency.
  </Tip>
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .void(&"10-3ffa27df-b171-44e0-b251-e95fbfc7a723".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**trans_id:** `String` — ReferenceId for the transaction (PaymentId).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.money_in.<a href="/src/api/resources/money_in/client.rs">getpaidv_2</a>(request: TransRequestBody, ach_validation: Option<Option<AchValidation>>, force_customer_creation: Option<Option<ForceCustomerCreation>>) -> Result<V2TransactionResponseWrapper, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Make a single transaction. This method authorizes and captures a payment in one step. This is the v2 version of the `api/MoneyIn/getpaid` endpoint, and returns the unified response format. See [Pay In unified response codes reference](/developers/references/pay-in-unified-response-codes) for more information.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .getpaidv_2(
            &Getpaidv2Request {
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
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**ach_validation:** `Option<AchValidation>` 
    
</dd>
</dl>

<dl>
<dd>

**force_customer_creation:** `Option<ForceCustomerCreation>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.money_in.<a href="/src/api/resources/money_in/client.rs">authorizev_2</a>(request: TransRequestBody, force_customer_creation: Option<Option<ForceCustomerCreation>>) -> Result<V2TransactionResponseWrapper, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Authorize a card transaction. This returns an authorization code and reserves funds for the merchant. Authorized transactions aren't flagged for settlement until captured. This is the v2 version of the `api/MoneyIn/authorize` endpoint, and returns the unified response format. See [Pay In unified response codes reference](/developers/references/pay-in-unified-response-codes) for more information.

**Note**: Only card transactions can be authorized. This endpoint can't be used for ACH transactions.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    Accountid, AchHolder, AchHolderType, AchSecCode, Achaccount, Achaccounttype, Achrouting,
    AdditionalData, AdditionalDataString, Attachments, BillData, BillDataPaymentTerms, BillItem,
    BillingAddressAddtlNullable, BillingAddressNullable, BillingCityNullable,
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
        .authorizev_2(
            &Authorizev2Request {
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
                force_customer_creation: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**force_customer_creation:** `Option<ForceCustomerCreation>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.money_in.<a href="/src/api/resources/money_in/client.rs">capturev_2</a>(trans_id: String, request: CaptureRequest) -> Result<V2TransactionResponseWrapper, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Capture an authorized transaction to complete the transaction and move funds from the customer to merchant account. This is the v2 version of the `api/MoneyIn/capture/{transId}` endpoint, and returns the unified response format. See [Pay In unified response codes reference](/developers/references/pay-in-unified-response-codes) for more information.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{CapturePaymentDetails, CaptureRequest};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .money_in
        .capturev_2(
            &"10-7d9cd67d-2d5d-4cd7-a1b7-72b8b201ec13".to_string(),
            &CaptureRequest {
                payment_details: CapturePaymentDetails {
                    total_amount: 105.0,
                    service_fee: Some(5.0),
                },
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**trans_id:** `String` — ReferenceId for the transaction (PaymentId).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.money_in.<a href="/src/api/resources/money_in/client.rs">refundv_2</a>(trans_id: String) -> Result<V2TransactionResponseWrapper, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Give a full refund for a transaction that has settled and send money back to the account holder. To perform a partial refund, see [Partially refund a transaction](developers/api-reference/moneyinV2/partial-refund-a-settled-transaction).

This is the v2 version of the refund endpoint, and returns the unified response format. See [Pay In unified response codes reference](/developers/references/pay-in-unified-response-codes) for more information.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .refundv_2(&"10-3ffa27df-b171-44e0-b251-e95fbfc7a723".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**trans_id:** `String` — ReferenceId for the transaction (PaymentId).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.money_in.<a href="/src/api/resources/money_in/client.rs">refundv_2_amount</a>(trans_id: String, amount: f64) -> Result<V2TransactionResponseWrapper, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Refund a transaction that has settled and send money back to the account holder. If `amount` is omitted or set to 0, performs a full refund. When a non-zero `amount` is provided, this endpoint performs a partial refund.

This is the v2 version of the refund endpoint, and returns the unified response format. See [Pay In unified response codes reference](/developers/references/pay-in-unified-response-codes) for more information.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .refundv_2_amount(
            &"10-3ffa27df-b171-44e0-b251-e95fbfc7a723".to_string(),
            0.0,
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**trans_id:** `String` — ReferenceId for the transaction (PaymentId).
    
</dd>
</dl>

<dl>
<dd>

**amount:** `f64` — Amount to refund from original transaction, minus any service fees charged on the original transaction. If omitted or set to 0, performs a full refund.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.money_in.<a href="/src/api/resources/money_in/client.rs">voidv_2</a>(trans_id: String) -> Result<V2TransactionResponseWrapper, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Cancel a transaction that hasn't been settled yet. Voiding non-captured authorizations prevents future captures. This is the v2 version of the `api/MoneyIn/void/{transId}` endpoint, and returns the unified response format. See [Pay In unified response codes reference](/developers/references/pay-in-unified-response-codes) for more information.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .voidv_2(&"10-3ffa27df-b171-44e0-b251-e95fbfc7a723".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**trans_id:** `String` — ReferenceId for the transaction (PaymentId).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## MoneyOut
<details><summary><code>client.money_out.<a href="/src/api/resources/money_out/client.rs">authorize_out</a>(request: AuthorizePayoutBody, allow_duplicated_bills: Option<Option<bool>>, do_not_create_bills: Option<Option<bool>>, force_vendor_creation: Option<Option<bool>>) -> Result<AuthCapturePayoutResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Authorizes transaction for payout. Authorized transactions aren't flagged for settlement until captured. Use `referenceId` returned in the response to capture the transaction. 
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    AccountNumber, Accountid, AccountingField, AchHolderType, AchSecCode, AdditionalData,
    AdditionalDataString, AddressAddtlNullable, AddressNullable, Attachments,
    AuthorizePaymentMethod, AuthorizePayoutBody, BankAccountHolderName, BankName, BillId, Comments,
    Contacts, ContactsField, Datenullable, Discount, Email, Entrypointfield, FileContent,
    FileContentFtype, IdempotencyKey, Initiator, InvoiceNumber, LocationCode, LotNumber, Mcc,
    NetAmountstring, OrderId, Orderdescription, PayeeName, Remitaddress1, Remitaddress2, Remitcity,
    Remitcountry, Remitstate, Remitzip, RequestOutAuthorizeInvoiceData,
    RequestOutAuthorizePaymentDetails, RequestOutAuthorizeVendorBillingData,
    RequestOutAuthorizeVendorData, RoutingAccount, Source, StoredMethodUsageType, Subdomain,
    Subscriptionid, Terms, TypeAccount, VendorCheckNumber, VendorEin, VendorName1, VendorName2,
    VendorNumber, VendorPaymentMethod, VendorPhone, Vendorid, Vendorstatus,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .money_out
        .authorize_out(
            &AuthorizeOutRequest {
                body: AuthorizePayoutBody {
                    entry_point: Entrypointfield("48acde49".to_string()),
                    source: None,
                    order_id: None,
                    order_description: Some(Orderdescription("Window Painting".to_string())),
                    payment_method: AuthorizePaymentMethod {
                        method: "managed".to_string(),
                        ach_holder: None,
                        ach_routing: None,
                        ach_account: None,
                        ach_account_type: None,
                        ach_code: None,
                        ach_holder_type: None,
                        stored_method_id: None,
                        initiator: None,
                        stored_method_usage_type: None,
                    },
                    payment_details: RequestOutAuthorizePaymentDetails {
                        check_number: None,
                        currency: None,
                        service_fee: None,
                        total_amount: Some(47.0),
                        unbundled: Some(false),
                    },
                    vendor_data: RequestOutAuthorizeVendorData {
                        vendor_number: Some(VendorNumber("7895433".to_string())),
                        name_1: None,
                        name_2: None,
                        ein: None,
                        phone: None,
                        email: None,
                        address_1: None,
                        city: None,
                        state: None,
                        zip: None,
                        country: None,
                        mcc: None,
                        contacts: None,
                        billing_data: None,
                        vendor_status: None,
                        remit_address_1: None,
                        remit_address_2: None,
                        remit_city: None,
                        remit_state: None,
                        remit_zip: None,
                        remit_country: None,
                        customer_vendor_account: None,
                        custom_field_1: None,
                        custom_field_2: None,
                        additional_data: None,
                        address_2: None,
                        internal_reference_id: None,
                        location_code: None,
                        payee_name_1: None,
                        payee_name_2: None,
                        payment_method: None,
                        vendor_id: None,
                    },
                    invoice_data: vec![RequestOutAuthorizeInvoiceData {
                        invoice_number: None,
                        net_amount: None,
                        invoice_date: None,
                        due_date: None,
                        comments: None,
                        lot_number: None,
                        bill_id: Some(BillId(54323)),
                        discount: None,
                        terms: None,
                        accounting_field_1: None,
                        accounting_field_2: None,
                        additional_data: None,
                        attachments: None,
                    }],
                    account_id: None,
                    subdomain: None,
                    subscription_id: None,
                },
                allow_duplicated_bills: None,
                do_not_create_bills: None,
                force_vendor_creation: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**allow_duplicated_bills:** `Option<bool>` — When `true`, the authorization bypasses the requirement for unique bills, identified by vendor invoice number. This allows you to make more than one payout authorization for a bill, like a split payment.
    
</dd>
</dl>

<dl>
<dd>

**do_not_create_bills:** `Option<bool>` — When `true`, Payabli won't automatically create a bill for this payout transaction.
    
</dd>
</dl>

<dl>
<dd>

**force_vendor_creation:** `Option<bool>` — When `true`, the request creates a new vendor record, regardless of whether the vendor already exists.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.money_out.<a href="/src/api/resources/money_out/client.rs">cancel_all_out</a>(request: Vec<String>) -> Result<CaptureAllOutResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Cancels an array of payout transactions.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .money_out
        .cancel_all_out(
            &vec!["2-29".to_string(), "2-28".to_string(), "2-27".to_string()],
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.money_out.<a href="/src/api/resources/money_out/client.rs">cancel_out_get</a>(reference_id: String) -> Result<PayabliApiResponse0000, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Cancel a payout transaction by ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .money_out
        .cancel_out_get(&"129-219".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**reference_id:** `String` — The ID for the payout transaction. 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.money_out.<a href="/src/api/resources/money_out/client.rs">cancel_out_delete</a>(reference_id: String) -> Result<PayabliApiResponse0000, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Cancel a payout transaction by ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .money_out
        .cancel_out_delete(&"129-219".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**reference_id:** `String` — The ID for the payout transaction. 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.money_out.<a href="/src/api/resources/money_out/client.rs">capture_all_out</a>(request: Vec<String>) -> Result<CaptureAllOutResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Captures an array of authorized payout transactions for settlement. The maximum number of transactions that can be captured in a single request is 500.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::IdempotencyKey;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .money_out
        .capture_all_out(
            &vec!["2-29".to_string(), "2-28".to_string(), "2-27".to_string()],
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.money_out.<a href="/src/api/resources/money_out/client.rs">capture_out</a>(reference_id: String) -> Result<AuthCapturePayoutResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Captures a single authorized payout transaction by ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::IdempotencyKey;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .money_out
        .capture_out(&"129-219".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**reference_id:** `String` — The ID for the payout transaction. 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.money_out.<a href="/src/api/resources/money_out/client.rs">payout_details</a>(trans_id: String) -> Result<BillDetailResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns details for a processed money out transaction.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .money_out
        .payout_details(&"45-as456777hhhhhhhhhh77777777-324".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**trans_id:** `String` — ReferenceId for the transaction (PaymentId).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.money_out.<a href="/src/api/resources/money_out/client.rs">v_card_get</a>(card_token: String) -> Result<VCardGetResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves vCard details for a single card in an entrypoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .money_out
        .v_card_get(&"20230403315245421165".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**card_token:** `String` — ID for a virtual card.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.money_out.<a href="/src/api/resources/money_out/client.rs">send_v_card_link</a>(request: SendVCardLinkRequest) -> Result<OperationResult, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Sends a virtual card link via email to the vendor associated with the `transId`.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .money_out
        .send_v_card_link(
            &SendVCardLinkRequest {
                trans_id: "01K33Z6YQZ6GD5QVKZ856MJBSC".to_string(),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**trans_id:** `String` — The transaction ID of the virtual card payout. The ID is returned as `ReferenceId` in the response when you authorize a payout with POST /MoneyOut/authorize.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.money_out.<a href="/src/api/resources/money_out/client.rs">get_check_image</a>(asset_name: String) -> Result<String, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve the image of a check associated with a processed transaction. 
The check image is returned in the response body as a base64-encoded string. 
The check image is only available for payouts that have been processed.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .money_out
        .get_check_image(
            &"check133832686289732320_01JKBNZ5P32JPTZY8XXXX000000.pdf".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**asset_name:** `String` 

Name of the check asset to retrieve. This is returned as `filename` in the `CheckData` object 
in the response when you make a GET request to `/MoneyOut/details/{transId}`.
```
    "CheckData": {
      "ftype": "PDF",
      "filename": "check133832686289732320_01JKBNZ5P32JPTZY8XXXX000000.pdf",
      "furl": "",
      "fContent": ""
  }
```
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Notification
<details><summary><code>client.notification.<a href="/src/api/resources/notification/client.rs">add_notification</a>(request: AddNotificationRequest) -> Result<PayabliApiResponseNotifications, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Create a new notification or autogenerated report. 
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    AddNotificationRequest, KeyValueDuo, NotificationReportRequest,
    NotificationReportRequestContent, NotificationReportRequestContentFileFormat,
    NotificationReportRequestContentReportName, NotificationReportRequestFrequency,
    NotificationReportRequestMethod, NotificationStandardRequest,
    NotificationStandardRequestContent, NotificationStandardRequestContentEventType,
    NotificationStandardRequestFrequency, NotificationStandardRequestMethod, Ownerid, Ownertype,
    Statusnotification, Timezone,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .notification
        .add_notification(
            &AddNotificationRequest::NotificationStandardRequest(NotificationStandardRequest {
                content: Some(NotificationStandardRequestContent {
                    event_type: Some(
                        NotificationStandardRequestContentEventType::CreatedApplication,
                    ),
                    internal_data: None,
                    transaction_id: None,
                    web_header_parameters: None,
                }),
                frequency: NotificationStandardRequestFrequency::Untilcancelled,
                method: NotificationStandardRequestMethod::Web,
                owner_id: Some(Ownerid("236".to_string())),
                owner_type: Ownertype(0),
                status: Some(Statusnotification(1)),
                target: "https://webhook.site/2871b8f8-edc7-441a-b376-98d8c8e33275".to_string(),
            }),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.notification.<a href="/src/api/resources/notification/client.rs">delete_notification</a>(n_id: String) -> Result<PayabliApiResponseNotifications, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes a single notification or autogenerated report.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .notification
        .delete_notification(&"1717".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**n_id:** `String` — Notification ID. 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.notification.<a href="/src/api/resources/notification/client.rs">get_notification</a>(n_id: String) -> Result<NotificationQueryRecord, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a single notification or autogenerated report's details.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .notification
        .get_notification(&"1717".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**n_id:** `String` — Notification ID. 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.notification.<a href="/src/api/resources/notification/client.rs">update_notification</a>(n_id: String, request: UpdateNotificationRequest) -> Result<PayabliApiResponseNotifications, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a notification or autogenerated report. 
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    KeyValueDuo, NotificationReportRequest, NotificationReportRequestContent,
    NotificationReportRequestContentFileFormat, NotificationReportRequestContentReportName,
    NotificationReportRequestFrequency, NotificationReportRequestMethod,
    NotificationStandardRequest, NotificationStandardRequestContent,
    NotificationStandardRequestContentEventType, NotificationStandardRequestFrequency,
    NotificationStandardRequestMethod, Ownerid, Ownertype, Statusnotification, Timezone,
    UpdateNotificationRequest,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .notification
        .update_notification(
            &"1717".to_string(),
            &UpdateNotificationRequest::NotificationStandardRequest(NotificationStandardRequest {
                content: Some(NotificationStandardRequestContent {
                    event_type: Some(NotificationStandardRequestContentEventType::ApprovedPayment),
                    internal_data: None,
                    transaction_id: None,
                    web_header_parameters: None,
                }),
                frequency: NotificationStandardRequestFrequency::Untilcancelled,
                method: NotificationStandardRequestMethod::Email,
                owner_id: Some(Ownerid("136".to_string())),
                owner_type: Ownertype(0),
                status: Some(Statusnotification(1)),
                target: "newemail@email.com".to_string(),
            }),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**n_id:** `String` — Notification ID. 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.notification.<a href="/src/api/resources/notification/client.rs">get_report_file</a>(id: String) -> Result<File, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Gets a copy of a generated report by ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.notification.get_report_file(1000000, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id:** `String` — Report ID
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Notificationlogs
<details><summary><code>client.notificationlogs.<a href="/src/api/resources/notificationlogs/client.rs">search_notification_logs</a>(request: NotificationLogSearchRequest, page_size: Option<Option<Pagesize>>, page: Option<Option<i64>>) -> Result<Vec<NotificationLog>, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Search notification logs with filtering and pagination.
  - Start date and end date cannot be more than 30 days apart
  - Either `orgId` or `paypointId` must be provided

This endpoint requires the `notifications_create` OR `notifications_read` permission.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{NotificationLogSearchRequest, Pagesize};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .notificationlogs
        .search_notification_logs(
            &SearchNotificationLogsRequest {
                page_size: Some(Pagesize(20)),
                body: NotificationLogSearchRequest {
                    start_date: DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
                        .unwrap()
                        .with_timezone(&Utc),
                    end_date: DateTime::parse_from_rfc3339("2024-01-31T23:59:59Z")
                        .unwrap()
                        .with_timezone(&Utc),
                    notification_event: Some("ActivatedMerchant".to_string()),
                    succeeded: Some(true),
                    org_id: Some(12345),
                    paypoint_id: None,
                },
                page: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**page_size:** `Option<Pagesize>` 
    
</dd>
</dl>

<dl>
<dd>

**page:** `Option<i64>` — The page number to retrieve. Defaults to 1 if not provided.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.notificationlogs.<a href="/src/api/resources/notificationlogs/client.rs">get_notification_log</a>(uuid: String) -> Result<NotificationLogDetail, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get detailed information for a specific notification log entry.
This endpoint requires the `notifications_create` OR `notifications_read` permission.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .notificationlogs
        .get_notification_log(
            &Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**uuid:** `String` — The notification log entry.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.notificationlogs.<a href="/src/api/resources/notificationlogs/client.rs">retry_notification_log</a>(uuid: String) -> Result<NotificationLogDetail, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retry sending a specific notification.

**Permissions:** notifications_create
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .notificationlogs
        .retry_notification_log(
            &Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**uuid:** `String` — Unique id
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.notificationlogs.<a href="/src/api/resources/notificationlogs/client.rs">bulk_retry_notification_logs</a>(request: BulkRetryRequest) -> Result<(), ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retry sending multiple notifications (maximum 50 IDs).
This is an async process, so use the search endpoint again to check the notification status.

This endpoint requires the `notifications_create` permission.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::BulkRetryRequest;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .notificationlogs
        .bulk_retry_notification_logs(
            &BulkRetryRequest(vec![
                Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000").unwrap(),
                Uuid::parse_str("550e8400-e29b-41d4-a716-446655440001").unwrap(),
                Uuid::parse_str("550e8400-e29b-41d4-a716-446655440002").unwrap(),
            ]),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Ocr
<details><summary><code>client.ocr.<a href="/src/api/resources/ocr/client.rs">ocr_document_form</a>(type_result: TypeResult, request: FileContentImageOnly) -> Result<PayabliApiResponseOcr, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Use this endpoint to upload an image file for OCR processing. The accepted file formats include PDF, JPG, JPEG, PNG, and GIF. Specify the desired type of result (either 'bill' or 'invoice') in the path parameter `typeResult`. The response will contain the OCR processing results, including extracted data such as bill number, vendor information, bill items, and more.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{FileContentFtype, FileContentImageOnly, TypeResult};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ocr
        .ocr_document_form(
            &TypeResult("typeResult".to_string()),
            &FileContentImageOnly {
                ftype: None,
                filename: None,
                furl: None,
                f_content: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**type_result:** `TypeResult` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.ocr.<a href="/src/api/resources/ocr/client.rs">ocr_document_json</a>(type_result: TypeResult, request: FileContentImageOnly) -> Result<PayabliApiResponseOcr, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Use this endpoint to submit a Base64-encoded image file for OCR processing. The accepted file formats include PDF, JPG, JPEG, PNG, and GIF. Specify the desired type of result (either 'bill' or 'invoice') in the path parameter `typeResult`. The response will contain the OCR processing results, including extracted data such as bill number, vendor information, bill items, and more.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{FileContentFtype, FileContentImageOnly, TypeResult};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .ocr
        .ocr_document_json(
            &TypeResult("typeResult".to_string()),
            &FileContentImageOnly {
                ftype: None,
                filename: None,
                furl: None,
                f_content: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**type_result:** `TypeResult` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Organization
<details><summary><code>client.organization.<a href="/src/api/resources/organization/client.rs">add_organization</a>(request: AddOrganizationRequest) -> Result<AddOrganizationResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates an organization under a parent organization. This is also referred to as a suborganization.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    Achaccount, Achrouting, AdditionalDataString, BillingAddressNullable, BillingCityNullable,
    BillingCountryNullable, BillingStateNullable, BillingZip, Contacts, ContactsField, Email,
    Enabled, FileContent, FileContentFtype, IdempotencyKey, Instrument, OrgParentId, Orgaddress,
    Orgcity, Orgcountry, Orgentryname, Orgidstring, Orgname, Orgstate, Orgtimezone, Orgtype,
    Orgwebsite, Orgzip, ReplyToEmail, ServiceCost,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .organization
        .add_organization(
            &AddOrganizationRequest {
                billing_info: Some(Instrument {
                    ach_account: Achaccount("123123123".to_string()),
                    ach_routing: Achrouting("123123123".to_string()),
                    billing_address: Some(BillingAddressNullable("123 Walnut Street".to_string())),
                    billing_city: Some(BillingCityNullable("Johnson City".to_string())),
                    billing_country: Some(BillingCountryNullable("US".to_string())),
                    billing_state: Some(BillingStateNullable("TN".to_string())),
                    billing_zip: Some(BillingZip("37615".to_string())),
                }),
                contacts: Some(ContactsField(Some(vec![Contacts {
                    contact_email: Some(Email("herman@hermanscoatings.com".to_string())),
                    contact_name: Some("Herman Martinez".to_string()),
                    contact_phone: Some("3055550000".to_string()),
                    contact_title: Some("Owner".to_string()),
                    additional_data: None,
                }]))),
                has_billing: Some(true),
                has_residual: Some(true),
                org_address: Some(Orgaddress("123 Walnut Street".to_string())),
                org_city: Some(Orgcity("Johnson City".to_string())),
                org_country: Some(Orgcountry("US".to_string())),
                org_entry_name: Some(Orgentryname("pilgrim-planner".to_string())),
                org_id: Some(Orgidstring("123".to_string())),
                org_logo: Some(FileContent {
                    f_content: Some("TXkgdGVzdCBmaWxlHJ==...".to_string()),
                    filename: Some("my-doc.pdf".to_string()),
                    ftype: Some(FileContentFtype::Pdf),
                    furl: Some("https://mysite.com/my-doc.pdf".to_string()),
                }),
                org_name: Orgname("Pilgrim Planner".to_string()),
                org_parent_id: Some(OrgParentId(236)),
                org_state: Some(Orgstate("TN".to_string())),
                org_timezone: Some(Orgtimezone(-5)),
                org_type: Orgtype(0),
                org_website: Some(Orgwebsite("www.pilgrimageplanner.com".to_string())),
                org_zip: Some(Orgzip("37615".to_string())),
                reply_to_email: ReplyToEmail("email@example.com".to_string()),
                services: None,
            },
            Some(
                RequestOptions::new()
                    .additional_header("idempotencyKey", "6B29FC40-CA47-1067-B31D-00DD010662DA"),
            ),
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**services:** `Option<Vec<ServiceCost>>` 
    
</dd>
</dl>

<dl>
<dd>

**billing_info:** `Option<Instrument>` 
    
</dd>
</dl>

<dl>
<dd>

**contacts:** `Option<ContactsField>` 
    
</dd>
</dl>

<dl>
<dd>

**has_billing:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**has_residual:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**org_address:** `Option<Orgaddress>` 
    
</dd>
</dl>

<dl>
<dd>

**org_city:** `Option<Orgcity>` 
    
</dd>
</dl>

<dl>
<dd>

**org_country:** `Option<Orgcountry>` 
    
</dd>
</dl>

<dl>
<dd>

**org_entry_name:** `Option<Orgentryname>` 
    
</dd>
</dl>

<dl>
<dd>

**org_id:** `Option<Orgidstring>` 
    
</dd>
</dl>

<dl>
<dd>

**org_logo:** `Option<FileContent>` 
    
</dd>
</dl>

<dl>
<dd>

**org_name:** `Orgname` 
    
</dd>
</dl>

<dl>
<dd>

**org_parent_id:** `Option<OrgParentId>` 
    
</dd>
</dl>

<dl>
<dd>

**org_state:** `Option<Orgstate>` 
    
</dd>
</dl>

<dl>
<dd>

**org_timezone:** `Option<Orgtimezone>` 
    
</dd>
</dl>

<dl>
<dd>

**org_type:** `Orgtype` 
    
</dd>
</dl>

<dl>
<dd>

**org_website:** `Option<Orgwebsite>` 
    
</dd>
</dl>

<dl>
<dd>

**org_zip:** `Option<Orgzip>` 
    
</dd>
</dl>

<dl>
<dd>

**reply_to_email:** `ReplyToEmail` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.organization.<a href="/src/api/resources/organization/client.rs">delete_organization</a>(org_id: i64) -> Result<DeleteOrganizationResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete an organization by ID. 
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.organization.delete_organization(123, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.organization.<a href="/src/api/resources/organization/client.rs">edit_organization</a>(org_id: i64, request: OrganizationData) -> Result<EditOrganizationResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates an organization's details by ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    Achaccount, Achrouting, AdditionalDataString, BillingAddressNullable, BillingCityNullable,
    BillingCountryNullable, BillingStateNullable, BillingZip, Contacts, ContactsField, Email,
    Enabled, FileContent, FileContentFtype, Instrument, OrgParentId, Orgaddress, Orgcity,
    Orgcountry, Orgentryname, Orgidstring, Orgname, Orgstate, Orgtimezone, Orgtype, Orgwebsite,
    Orgzip, ReplyToEmail, ServiceCost,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .organization
        .edit_organization(
            123,
            &OrganizationData {
                contacts: Some(ContactsField(Some(vec![Contacts {
                    contact_email: Some(Email("herman@hermanscoatings.com".to_string())),
                    contact_name: Some("Herman Martinez".to_string()),
                    contact_phone: Some("3055550000".to_string()),
                    contact_title: Some("Owner".to_string()),
                    additional_data: None,
                }]))),
                org_address: Some(Orgaddress("123 Walnut Street".to_string())),
                org_city: Some(Orgcity("Johnson City".to_string())),
                org_country: Some(Orgcountry("US".to_string())),
                org_entry_name: Some(Orgentryname("pilgrim-planner".to_string())),
                organization_data_org_id: Some(Orgidstring("123".to_string())),
                org_name: Some(Orgname("Pilgrim Planner".to_string())),
                org_state: Some(Orgstate("TN".to_string())),
                org_timezone: Some(Orgtimezone(-5)),
                org_type: Some(Orgtype(0)),
                org_website: Some(Orgwebsite("www.pilgrimageplanner.com".to_string())),
                org_zip: Some(Orgzip("37615".to_string())),
                services: None,
                billing_info: None,
                has_billing: None,
                has_residual: None,
                org_logo: None,
                org_parent_id: None,
                reply_to_email: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**services:** `Option<Vec<ServiceCost>>` 
    
</dd>
</dl>

<dl>
<dd>

**billing_info:** `Option<Instrument>` 
    
</dd>
</dl>

<dl>
<dd>

**contacts:** `Option<ContactsField>` 
    
</dd>
</dl>

<dl>
<dd>

**has_billing:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**has_residual:** `Option<bool>` 
    
</dd>
</dl>

<dl>
<dd>

**org_address:** `Option<Orgaddress>` 
    
</dd>
</dl>

<dl>
<dd>

**org_city:** `Option<Orgcity>` 
    
</dd>
</dl>

<dl>
<dd>

**org_country:** `Option<Orgcountry>` 
    
</dd>
</dl>

<dl>
<dd>

**org_entry_name:** `Option<Orgentryname>` 
    
</dd>
</dl>

<dl>
<dd>

**organization_data_org_id:** `Option<Orgidstring>` 
    
</dd>
</dl>

<dl>
<dd>

**org_logo:** `Option<FileContent>` 
    
</dd>
</dl>

<dl>
<dd>

**org_name:** `Option<Orgname>` 
    
</dd>
</dl>

<dl>
<dd>

**org_parent_id:** `Option<OrgParentId>` 
    
</dd>
</dl>

<dl>
<dd>

**org_state:** `Option<Orgstate>` 
    
</dd>
</dl>

<dl>
<dd>

**org_timezone:** `Option<Orgtimezone>` 
    
</dd>
</dl>

<dl>
<dd>

**org_type:** `Option<Orgtype>` 
    
</dd>
</dl>

<dl>
<dd>

**org_website:** `Option<Orgwebsite>` 
    
</dd>
</dl>

<dl>
<dd>

**org_zip:** `Option<Orgzip>` 
    
</dd>
</dl>

<dl>
<dd>

**reply_to_email:** `Option<ReplyToEmail>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.organization.<a href="/src/api/resources/organization/client.rs">get_basic_organization</a>(entry: String) -> Result<OrganizationQueryRecord, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Gets an organization's basic information by entry name (entrypoint identifier).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .organization
        .get_basic_organization(&"8cfec329267".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.organization.<a href="/src/api/resources/organization/client.rs">get_basic_organization_by_id</a>(org_id: i64) -> Result<OrganizationQueryRecord, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Gets an organizations basic details by org ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .organization
        .get_basic_organization_by_id(123, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.organization.<a href="/src/api/resources/organization/client.rs">get_organization</a>(org_id: i64) -> Result<OrganizationQueryRecord, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves details for an organization by ID. 
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.organization.get_organization(123, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.organization.<a href="/src/api/resources/organization/client.rs">get_settings_organization</a>(org_id: i64) -> Result<SettingsQueryRecord, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves an organization's settings.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .organization
        .get_settings_organization(123, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## PaymentLink
<details><summary><code>client.payment_link.<a href="/src/api/resources/payment_link/client.rs">add_pay_link_from_invoice</a>(id_invoice: i64, request: PaymentPageRequestBody, amount_fixed: Option<Option<bool>>, mail_2: Option<Option<String>>) -> Result<PayabliApiResponsePaymentLinks, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Generates a payment link for an invoice from the invoice ID. 
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    ContactElement, Element, Enabled, FileContent, FileContentFtype, HeaderElement, IdempotencyKey,
    InvoiceElement, LabelElement, MethodElement, MethodElementSettings,
    MethodElementSettingsApplePay, MethodElementSettingsApplePayButtonStyle,
    MethodElementSettingsApplePayButtonType, MethodElementSettingsApplePayLanguage, MethodsList,
    NoteElement, Order, PageElement, PagelinkSetting, PaymentPageRequestBody, PayorElement,
    PayorFields,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.payment_link.add_pay_link_from_invoice(23548884, &AddPayLinkFromInvoiceRequest {
        mail_2: Some("jo@example.com; ceo@example.com".to_string()),
        body: PaymentPageRequestBody {
            contact_us: Some(ContactElement {
                email_label: Some("Email".to_string()),
                enabled: Some(Enabled(true)),
                header: Some("Contact Us".to_string()),
                order: Some(Order(0)),
                payment_icons: Some(true),
                phone_label: Some("Phone".to_string())
            }),
            invoices: Some(InvoiceElement {
                enabled: Some(Enabled(true)),
                invoice_link: Some(LabelElement {
                    enabled: Some(Enabled(true)),
                    label: Some("View Invoice".to_string()),
                    order: Some(Order(0))
                }),
                order: Some(Order(0)),
                view_invoice_details: Some(LabelElement {
                    enabled: Some(Enabled(true)),
                    label: Some("Invoice Details".to_string()),
                    order: Some(Order(0))
                })
            }),
            logo: Some(Element {
                enabled: Some(Enabled(true)),
                order: Some(Order(0))
            }),
            message_before_paying: Some(LabelElement {
                enabled: Some(Enabled(true)),
                label: Some("Please review your payment details".to_string()),
                order: Some(Order(0))
            }),
            notes: Some(NoteElement {
                enabled: Some(Enabled(true)),
                header: Some("Additional Notes".to_string()),
                order: Some(Order(0)),
                placeholder: Some("Enter any additional notes here".to_string()),
                value: Some("".to_string())
            }),
            page: Some(PageElement {
                description: Some("Complete your payment securely".to_string()),
                enabled: Some(Enabled(true)),
                header: Some("Payment Page".to_string()),
                order: Some(Order(0))
            }),
            payment_button: Some(LabelElement {
                enabled: Some(Enabled(true)),
                label: Some("Pay Now".to_string()),
                order: Some(Order(0))
            }),
            payment_methods: Some(MethodElement {
                all_methods_checked: Some(true),
                enabled: Some(Enabled(true)),
                header: Some("Payment Methods".to_string()),
                methods: Some(MethodsList {
                    amex: Some(true),
                    apple_pay: Some(true),
                    google_pay: None,
                    discover: Some(true),
                    e_check: Some(true),
                    mastercard: Some(true),
                    visa: Some(true)
                }),
                order: Some(Order(0)),
                settings: Some(MethodElementSettings {
                    apple_pay: Some(MethodElementSettingsApplePay {
                        button_style: Some(MethodElementSettingsApplePayButtonStyle::Black),
                        button_type: Some(MethodElementSettingsApplePayButtonType::Pay),
                        language: Some(MethodElementSettingsApplePayLanguage::EnUs)
                    })
                })
            }),
            payor: Some(PayorElement {
                enabled: Some(Enabled(true)),
                fields: Some(vec![PayorFields {
                    display: Some(true),
                    fixed: Some(true),
                    identifier: Some(true),
                    label: Some("Full Name".to_string()),
                    name: Some("fullName".to_string()),
                    order: Some(Order(0)),
                    required: Some(true),
                    validation: Some("alpha".to_string()),
                    value: Some("".to_string()),
                    width: Some(0)
                }]),
                header: Some("Payor Information".to_string()),
                order: Some(Order(0))
            }),
            review: Some(HeaderElement {
                enabled: Some(Enabled(true)),
                header: Some("Review Payment".to_string()),
                order: Some(Order(0))
            }),
            settings: Some(PagelinkSetting {
                color: Some("#000000".to_string()),
                custom_css_url: Some("https://example.com/custom.css".to_string()),
                language: Some("en".to_string()),
                page_logo: Some(FileContent {
                    f_content: Some("PHN2ZyB2aWV3Qm94PSIwIDAgODAwIDEwMDAiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyI+CiAgPCEtLSBCYWNrZ3JvdW5kIC0tPgogIDxyZWN0IHdpZHRoPSI4MDAiIGhlaWdodD0iMTAwMCIgZmlsbD0id2hpdGUiLz4KICAKICA8IS0tIENvbXBhbnkgSGVhZGVyIC0tPgogIDx0ZXh0IHg9IjQwIiB5PSI2MCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjI0IiBmb250LXdlaWdodD0iYm9sZCIgZmlsbD0iIzJjM2U1MCI+R3J1enlhIEFkdmVudHVyZSBPdXRmaXR0ZXJzPC90ZXh0PgogIDxsaW5lIHgxPSI0MCIgeTE9IjgwIiB4Mj0iNzYwIiB5Mj0iODAiIHN0cm9rZT0iIzJjM2U1MCIgc3Ryb2tlLXdpZHRoPSIyIi8+CiAgCiAgPCEtLSBDb21wYW55IERldGFpbHMgLS0+CiAgPHRleHQgeD0iNDAiIHk9IjExMCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmaWxsPSIjMzQ0OTVlIj4xMjMgTW91bnRhaW4gVmlldyBSb2FkPC90ZXh0PgogIDx0ZXh0IHg9IjQwIiB5PSIxMzAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+VGJpbGlzaSwgR2VvcmdpYSAwMTA1PC90ZXh0PgogIDx0ZXh0IHg9IjQwIiB5PSIxNTAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+VGVsOiArOTk1IDMyIDEyMyA0NTY3PC90ZXh0PgogIDx0ZXh0IHg9IjQwIiB5PSIxNzAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+RW1haWw6IGluZm9AZ3J1enlhYWR2ZW50dXJlcy5jb208L3RleHQ+CgogIDwhLS0gSW52b2ljZSBUaXRsZSAtLT4KICA8dGV4dCB4PSI2MDAiIHk9IjExMCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjI0IiBmb250LXdlaWdodD0iYm9sZCIgZmlsbD0iIzJjM2U1MCI+SU5WT0lDRTwvdGV4dD4KICA8dGV4dCB4PSI2MDAiIHk9IjE0MCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmaWxsPSIjMzQ0OTVlIj5EYXRlOiAxMi8xMS8yMDI0PC90ZXh0PgogIDx0ZXh0IHg9IjYwMCIgeT0iMTYwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZpbGw9IiMzNDQ5NWUiPkludm9pY2UgIzogR1JaLTIwMjQtMTEyMzwvdGV4dD4KCiAgPCEtLSBCaWxsIFRvIFNlY3Rpb24gLS0+CiAgPHRleHQgeD0iNDAiIHk9IjIyMCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE2IiBmb250LXdlaWdodD0iYm9sZCIgZmlsbD0iIzJjM2U1MCI+QklMTCBUTzo8L3RleHQ+CiAgPHJlY3QgeD0iNDAiIHk9IjIzNSIgd2lkdGg9IjMwMCIgaGVpZ2h0PSI4MCIgZmlsbD0iI2Y3ZjlmYSIvPgogIDx0ZXh0IHg9IjUwIiB5PSIyNjAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+W0N1c3RvbWVyIE5hbWVdPC90ZXh0PgogIDx0ZXh0IHg9IjUwIiB5PSIyODAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+W0FkZHJlc3MgTGluZSAxXTwvdGV4dD4KICA8dGV4dCB4PSI1MCIgeT0iMzAwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZpbGw9IiMzNDQ5NWUiPltDaXR5LCBDb3VudHJ5XTwvdGV4dD4KCiAgPCEtLSBUYWJsZSBIZWFkZXJzIC0tPgogIDxyZWN0IHg9IjQwIiB5PSIzNDAiIHdpZHRoPSI3MjAiIGhlaWdodD0iMzAiIGZpbGw9IiMyYzNlNTAiLz4KICA8dGV4dCB4PSI1MCIgeT0iMzYwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZvbnQtd2VpZ2h0PSJib2xkIiBmaWxsPSJ3aGl0ZSI+RGVzY3JpcHRpb248L3RleHQ+CiAgPHRleHQgeD0iNDUwIiB5PSIzNjAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZm9udC13ZWlnaHQ9ImJvbGQiIGZpbGw9IndoaXRlIj5RdWFudGl0eTwvdGV4dD4KICA8dGV4dCB4PSI1NTAiIHk9IjM2MCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmb250LXdlaWdodD0iYm9sZCIgZmlsbD0id2hpdGUiPlJhdGU8L3RleHQ+CiAgPHRleHQgeD0iNjgwIiB5PSIzNjAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZm9udC13ZWlnaHQ9ImJvbGQiIGZpbGw9IndoaXRlIj5BbW91bnQ8L3RleHQ+CgogIDwhLS0gVGFibGUgUm93cyAtLT4KICA8cmVjdCB4PSI0MCIgeT0iMzcwIiB3aWR0aD0iNzIwIiBoZWlnaHQ9IjMwIiBmaWxsPSIjZjdmOWZhIi8+CiAgPHRleHQgeD0iNTAiIHk9IjM5MCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmaWxsPSIjMzQ0OTVlIj5Nb3VudGFpbiBDbGltYmluZyBFcXVpcG1lbnQgUmVudGFsPC90ZXh0PgogIDx0ZXh0IHg9IjQ1MCIgeT0iMzkwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZpbGw9IiMzNDQ5NWUiPjE8L3RleHQ+CiAgPHRleHQgeD0iNTUwIiB5PSIzOTAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+JDI1MC4wMDwvdGV4dD4KICA8dGV4dCB4PSI2ODAiIHk9IjM5MCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmaWxsPSIjMzQ0OTVlIj4kMjUwLjAwPC90ZXh0PgoKICA8cmVjdCB4PSI0MCIgeT0iNDAwIiB3aWR0aD0iNzIwIiBoZWlnaHQ9IjMwIiBmaWxsPSJ3aGl0ZSIvPgogIDx0ZXh0IHg9IjUwIiB5PSI0MjAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+R3VpZGVkIFRyZWsgUGFja2FnZSAtIDIgRGF5czwvdGV4dD4KICA8dGV4dCB4PSI0NTAiIHk9IjQyMCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmaWxsPSIjMzQ0OTVlIj4xPC90ZXh0PgogIDx0ZXh0IHg9IjU1MCIgeT0iNDIwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZpbGw9IiMzNDQ5NWUiPiQ0MDAuMDA8L3RleHQ+CiAgPHRleHQgeD0iNjgwIiB5PSI0MjAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+JDQwMC4wMDwvdGV4dD4KCiAgPHJlY3QgeD0iNDAiIHk9IjQzMCIgd2lkdGg9IjcyMCIgaGVpZ2h0PSIzMCIgZmlsbD0iI2Y3ZjlmYSIvPgogIDx0ZXh0IHg9IjUwIiB5PSI0NTAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+U2FmZXR5IEVxdWlwbWVudCBQYWNrYWdlPC90ZXh0PgogIDx0ZXh0IHg9IjQ1MCIgeT0iNDUwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZpbGw9IiMzNDQ5NWUiPjE8L3RleHQ+CiAgPHRleHQgeD0iNTUwIiB5PSI0NTAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+JDE1MC4wMDwvdGV4dD4KICA8dGV4dCB4PSI2ODAiIHk9IjQ1MCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmaWxsPSIjMzQ0OTVlIj4kMTUwLjAwPC90ZXh0PgoKICA8IS0tIFRvdGFscyAtLT4KICA8bGluZSB4MT0iNDAiIHkxPSI0ODAiIHgyPSI3NjAiIHkyPSI0ODAiIHN0cm9rZT0iIzJjM2U1MCIgc3Ryb2tlLXdpZHRoPSIxIi8+CiAgPHRleHQgeD0iNTUwIiB5PSI1MTAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZm9udC13ZWlnaHQ9ImJvbGQiIGZpbGw9IiMzNDQ5NWUiPlN1YnRvdGFsOjwvdGV4dD4KICA8dGV4dCB4PSI2ODAiIHk9IjUxMCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmaWxsPSIjMzQ0OTVlIj4kODAwLjAwPC90ZXh0PgogIDx0ZXh0IHg9IjU1MCIgeT0iNTM1IiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZvbnQtd2VpZ2h0PSJib2xkIiBmaWxsPSIjMzQ0OTVlIj5UYXggKDE4JSk6PC90ZXh0PgogIDx0ZXh0IHg9IjY4MCIgeT0iNTM1IiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZpbGw9IiMzNDQ5NWUiPiQxNDQuMDA8L3RleHQ+CiAgPHRleHQgeD0iNTUwIiB5PSI1NzAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNiIgZm9udC13ZWlnaHQ9ImJvbGQiIGZpbGw9IiMyYzNlNTAiPlRvdGFsOjwvdGV4dD4KICA8dGV4dCB4PSI2ODAiIHk9IjU3MCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE2IiBmb250LXdlaWdodD0iYm9sZCIgZmlsbD0iIzJjM2U1MCI+JDk0NC4wMDwvdGV4dD4KCiAgPCEtLSBQYXltZW50IFRlcm1zIC0tPgogIDx0ZXh0IHg9IjQwIiB5PSI2NDAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNiIgZm9udC13ZWlnaHQ9ImJvbGQiIGZpbGw9IiMyYzNlNTAiPlBheW1lbnQgVGVybXM8L3RleHQ+CiAgPHRleHQgeD0iNDAiIHk9IjY3MCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjE0IiBmaWxsPSIjMzQ0OTVlIj5QYXltZW50IGlzIGR1ZSB3aXRoaW4gMzAgZGF5czwvdGV4dD4KICA8dGV4dCB4PSI0MCIgeT0iNjkwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZpbGw9IiMzNDQ5NWUiPlBsZWFzZSBpbmNsdWRlIGludm9pY2UgbnVtYmVyIG9uIHBheW1lbnQ8L3RleHQ+CgogIDwhLS0gQmFuayBEZXRhaWxzIC0tPgogIDx0ZXh0IHg9IjQwIiB5PSI3MzAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNiIgZm9udC13ZWlnaHQ9ImJvbGQiIGZpbGw9IiMyYzNlNTAiPkJhbmsgRGV0YWlsczwvdGV4dD4KICA8dGV4dCB4PSI0MCIgeT0iNzYwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZpbGw9IiMzNDQ5NWUiPkJhbms6IEJhbmsgb2YgR2VvcmdpYTwvdGV4dD4KICA8dGV4dCB4PSI0MCIgeT0iNzgwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTQiIGZpbGw9IiMzNDQ5NWUiPklCQU46IEdFMTIzNDU2Nzg5MDEyMzQ1Njc4PC90ZXh0PgogIDx0ZXh0IHg9IjQwIiB5PSI4MDAiIGZvbnQtZmFtaWx5PSJBcmlhbCIgZm9udC1zaXplPSIxNCIgZmlsbD0iIzM0NDk1ZSI+U1dJRlQ6IEJBR0FHRTIyPC90ZXh0PgoKICA8IS0tIEZvb3RlciAtLT4KICA8bGluZSB4MT0iNDAiIHkxPSI5MDAiIHgyPSI3NjAiIHkyPSI5MDAiIHN0cm9rZT0iIzJjM2U1MCIgc3Ryb2tlLXdpZHRoPSIxIi8+CiAgPHRleHQgeD0iNDAiIHk9IjkzMCIgZm9udC1mYW1pbHk9IkFyaWFsIiBmb250LXNpemU9IjEyIiBmaWxsPSIjN2Y4YzhkIj5UaGFuayB5b3UgZm9yIGNob29zaW5nIEdydXp5YSBBZHZlbnR1cmUgT3V0Zml0dGVyczwvdGV4dD4KICA8dGV4dCB4PSI0MCIgeT0iOTUwIiBmb250LWZhbWlseT0iQXJpYWwiIGZvbnQtc2l6ZT0iMTIiIGZpbGw9IiM3ZjhjOGQiPnd3dy5ncnV6eWFhZHZlbnR1cmVzLmNvbTwvdGV4dD4KPC9zdmc+Cg==".to_string()),
                    filename: Some("logo.jpg".to_string()),
                    ftype: Some(FileContentFtype::Jpg),
                    furl: Some("".to_string())
                }),
                redirect_after_approve: Some(true),
                redirect_after_approve_url: Some("https://example.com/success".to_string())
            })
        },
        amount_fixed: None
    }, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id_invoice:** `i64` — Invoice ID
    
</dd>
</dl>

<dl>
<dd>

**amount_fixed:** `Option<bool>` — Indicates whether customer can modify the payment amount. A value of `true` means the amount isn't modifiable, a value `false` means the payor can modify the amount to pay.
    
</dd>
</dl>

<dl>
<dd>

**mail_2:** `Option<String>` — List of recipient email addresses. When there is more than one, separate them by a semicolon (;).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payment_link.<a href="/src/api/resources/payment_link/client.rs">add_pay_link_from_bill</a>(bill_id: i64, request: PaymentPageRequestBody, amount_fixed: Option<Option<bool>>, mail_2: Option<Option<String>>) -> Result<PayabliApiResponsePaymentLinks, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Generates a payment link for a bill from the bill ID. 
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    ContactElement, Element, Enabled, FileContent, FileContentFtype, HeaderElement, IdempotencyKey,
    InvoiceElement, LabelElement, MethodElement, MethodElementSettings,
    MethodElementSettingsApplePay, MethodElementSettingsApplePayButtonStyle,
    MethodElementSettingsApplePayButtonType, MethodElementSettingsApplePayLanguage, MethodsList,
    NoteElement, Order, PageElement, PagelinkSetting, PaymentPageRequestBody, PayorElement,
    PayorFields,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .payment_link
        .add_pay_link_from_bill(
            23548884,
            &AddPayLinkFromBillRequest {
                mail_2: Some("jo@example.com; ceo@example.com".to_string()),
                body: PaymentPageRequestBody {
                    contact_us: Some(ContactElement {
                        email_label: Some("Email".to_string()),
                        enabled: Some(Enabled(true)),
                        header: Some("Contact Us".to_string()),
                        order: Some(Order(0)),
                        payment_icons: Some(true),
                        phone_label: Some("Phone".to_string()),
                    }),
                    invoices: None,
                    logo: Some(Element {
                        enabled: Some(Enabled(true)),
                        order: Some(Order(0)),
                    }),
                    message_before_paying: Some(LabelElement {
                        enabled: Some(Enabled(true)),
                        label: Some("Please review your payment details".to_string()),
                        order: Some(Order(0)),
                    }),
                    notes: Some(NoteElement {
                        enabled: Some(Enabled(true)),
                        header: Some("Additional Notes".to_string()),
                        order: Some(Order(0)),
                        placeholder: Some("Enter any additional notes here".to_string()),
                        value: Some("".to_string()),
                    }),
                    page: Some(PageElement {
                        description: Some("Get paid securely".to_string()),
                        enabled: Some(Enabled(true)),
                        header: Some("Payment Page".to_string()),
                        order: Some(Order(0)),
                    }),
                    payment_button: Some(LabelElement {
                        enabled: Some(Enabled(true)),
                        label: Some("Pay Now".to_string()),
                        order: Some(Order(0)),
                    }),
                    payment_methods: Some(MethodElement {
                        all_methods_checked: Some(true),
                        enabled: Some(Enabled(true)),
                        header: Some("Payment Methods".to_string()),
                        methods: Some(MethodsList {
                            amex: Some(true),
                            apple_pay: Some(true),
                            google_pay: None,
                            discover: Some(true),
                            e_check: Some(true),
                            mastercard: Some(true),
                            visa: Some(true),
                        }),
                        order: Some(Order(0)),
                        settings: None,
                    }),
                    payor: Some(PayorElement {
                        enabled: Some(Enabled(true)),
                        fields: Some(vec![PayorFields {
                            display: Some(true),
                            fixed: Some(true),
                            identifier: Some(true),
                            label: Some("Full Name".to_string()),
                            name: Some("fullName".to_string()),
                            order: Some(Order(0)),
                            required: Some(true),
                            validation: Some("alpha".to_string()),
                            value: Some("".to_string()),
                            width: Some(0),
                        }]),
                        header: Some("Payor Information".to_string()),
                        order: Some(Order(0)),
                    }),
                    review: Some(HeaderElement {
                        enabled: Some(Enabled(true)),
                        header: Some("Review Payment".to_string()),
                        order: Some(Order(0)),
                    }),
                    settings: Some(PagelinkSetting {
                        color: Some("#000000".to_string()),
                        custom_css_url: None,
                        language: Some("en".to_string()),
                        page_logo: None,
                        redirect_after_approve: None,
                        redirect_after_approve_url: None,
                    }),
                },
                amount_fixed: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**bill_id:** `i64` — The Payabli ID for the bill.
    
</dd>
</dl>

<dl>
<dd>

**amount_fixed:** `Option<bool>` — Indicates whether customer can modify the payment amount. A value of `true` means the amount isn't modifiable, a value `false` means the payor can modify the amount to pay.
    
</dd>
</dl>

<dl>
<dd>

**mail_2:** `Option<String>` — List of recipient email addresses. When there is more than one, separate them by a semicolon (;).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payment_link.<a href="/src/api/resources/payment_link/client.rs">delete_pay_link_from_id</a>(pay_link_id: String) -> Result<PayabliApiResponsePaymentLinks, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes a payment link by ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .payment_link
        .delete_pay_link_from_id(&"payLinkId".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pay_link_id:** `String` — ID for the payment link.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payment_link.<a href="/src/api/resources/payment_link/client.rs">get_pay_link_from_id</a>(paylink_id: String) -> Result<GetPayLinkFromIdResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a payment link by ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .payment_link
        .get_pay_link_from_id(&"paylinkId".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**paylink_id:** `String` — ID for payment link
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payment_link.<a href="/src/api/resources/payment_link/client.rs">push_pay_link_from_id</a>(pay_link_id: String, request: PushPayLinkRequest) -> Result<PayabliApiResponsePaymentLinks, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Send a payment link to the specified email addresses or phone numbers.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{PushPayLinkRequest, PushPayLinkRequestEmail, PushPayLinkRequestSms};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .payment_link
        .push_pay_link_from_id(
            &"payLinkId".to_string(),
            &PushPayLinkRequest::Sms {
                data: PushPayLinkRequestSms {},
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pay_link_id:** `String` — ID for the payment link.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payment_link.<a href="/src/api/resources/payment_link/client.rs">refresh_pay_link_from_id</a>(pay_link_id: String, amount_fixed: Option<Option<bool>>) -> Result<PayabliApiResponsePaymentLinks, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Refresh a payment link's content after an update.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .payment_link
        .refresh_pay_link_from_id(
            &"payLinkId".to_string(),
            &RefreshPayLinkFromIdQueryRequest { amount_fixed: None },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pay_link_id:** `String` — ID for the payment link.
    
</dd>
</dl>

<dl>
<dd>

**amount_fixed:** `Option<bool>` — Indicates whether customer can modify the payment amount. A value of `true` means the amount isn't modifiable, a value `false` means the payor can modify the amount to pay.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payment_link.<a href="/src/api/resources/payment_link/client.rs">send_pay_link_from_id</a>(pay_link_id: String, attachfile: Option<Option<bool>>, mail_2: Option<Option<String>>) -> Result<PayabliApiResponsePaymentLinks, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Sends a payment link to the specified email addresses. 
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .payment_link
        .send_pay_link_from_id(
            &"payLinkId".to_string(),
            &SendPayLinkFromIdQueryRequest {
                mail_2: Some("jo@example.com; ceo@example.com".to_string()),
                attachfile: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pay_link_id:** `String` — ID for the payment link.
    
</dd>
</dl>

<dl>
<dd>

**attachfile:** `Option<bool>` — When `true`, attaches a PDF version of invoice to the email.
    
</dd>
</dl>

<dl>
<dd>

**mail_2:** `Option<String>` — List of recipient email addresses. When there is more than one, separate them by a semicolon (;).
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payment_link.<a href="/src/api/resources/payment_link/client.rs">update_pay_link_from_id</a>(pay_link_id: String, request: PayLinkUpdateData) -> Result<PayabliApiResponsePaymentLinks, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates a payment link's details.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    ContactElement, Element, Enabled, FileContent, FileContentFtype, HeaderElement, LabelElement,
    MethodElement, MethodElementSettings, MethodElementSettingsApplePay,
    MethodElementSettingsApplePayButtonStyle, MethodElementSettingsApplePayButtonType,
    MethodElementSettingsApplePayLanguage, MethodsList, NoteElement, Order, PageElement,
    PagelinkSetting,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .payment_link
        .update_pay_link_from_id(
            &"332-c277b704-1301".to_string(),
            &PayLinkUpdateData {
                notes: Some(NoteElement {
                    enabled: Some(Enabled(true)),
                    header: Some("Additional Notes".to_string()),
                    order: Some(Order(0)),
                    placeholder: Some("Enter any additional notes here".to_string()),
                    value: Some("".to_string()),
                }),
                payment_button: Some(LabelElement {
                    enabled: Some(Enabled(true)),
                    label: Some("Pay Now".to_string()),
                    order: Some(Order(0)),
                }),
                contact_us: None,
                logo: None,
                message_before_paying: None,
                page: None,
                payment_methods: None,
                review: None,
                settings: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**pay_link_id:** `String` — ID for the payment link.
    
</dd>
</dl>

<dl>
<dd>

**contact_us:** `Option<ContactElement>` — ContactUs section of payment link page
    
</dd>
</dl>

<dl>
<dd>

**logo:** `Option<Element>` — Logo section of payment link page
    
</dd>
</dl>

<dl>
<dd>

**message_before_paying:** `Option<LabelElement>` — Message section of payment link page
    
</dd>
</dl>

<dl>
<dd>

**notes:** `Option<NoteElement>` — Notes section of payment link page
    
</dd>
</dl>

<dl>
<dd>

**page:** `Option<PageElement>` — Page header section of payment link page
    
</dd>
</dl>

<dl>
<dd>

**payment_button:** `Option<LabelElement>` — Payment button section of payment link page
    
</dd>
</dl>

<dl>
<dd>

**payment_methods:** `Option<MethodElement>` — Payment methods section of payment link page
    
</dd>
</dl>

<dl>
<dd>

**review:** `Option<HeaderElement>` — Review section of payment link page
    
</dd>
</dl>

<dl>
<dd>

**settings:** `Option<PagelinkSetting>` — Settings section of payment link page
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payment_link.<a href="/src/api/resources/payment_link/client.rs">add_pay_link_from_bill_lot_number</a>(lot_number: String, request: PaymentPageRequestBody, entry_point: Option<Entry>, vendor_number: Option<String>, mail_2: Option<Option<String>>, amount_fixed: Option<Option<String>>) -> Result<PayabliApiResponsePaymentLinks, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Generates a vendor payment link for a specific bill lot number. This allows you to pay all bills with the same lot number for a vendor with a single payment link.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    ContactElement, Element, Enabled, Entry, FileContent, FileContentFtype, HeaderElement,
    InvoiceElement, LabelElement, MethodElement, MethodElementSettings,
    MethodElementSettingsApplePay, MethodElementSettingsApplePayButtonStyle,
    MethodElementSettingsApplePayButtonType, MethodElementSettingsApplePayLanguage, MethodsList,
    NoteElement, Order, PageElement, PagelinkSetting, PaymentPageRequestBody, PayorElement,
    PayorFields,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .payment_link
        .add_pay_link_from_bill_lot_number(
            &"LOT-2024-001".to_string(),
            &AddPayLinkFromBillLotNumberRequest {
                entry_point: Entry("billing".to_string()),
                vendor_number: "VENDOR-123".to_string(),
                mail_2: Some("customer@example.com; billing@example.com".to_string()),
                amount_fixed: Some("true".to_string()),
                body: PaymentPageRequestBody {
                    contact_us: Some(ContactElement {
                        email_label: Some("Email".to_string()),
                        enabled: Some(Enabled(true)),
                        header: Some("Contact Us".to_string()),
                        order: Some(Order(0)),
                        payment_icons: Some(true),
                        phone_label: Some("Phone".to_string()),
                    }),
                    invoices: None,
                    logo: Some(Element {
                        enabled: Some(Enabled(true)),
                        order: Some(Order(0)),
                    }),
                    message_before_paying: Some(LabelElement {
                        enabled: Some(Enabled(true)),
                        label: Some("Please review your payment details".to_string()),
                        order: Some(Order(0)),
                    }),
                    notes: Some(NoteElement {
                        enabled: Some(Enabled(true)),
                        header: Some("Additional Notes".to_string()),
                        order: Some(Order(0)),
                        placeholder: Some("Enter any additional notes here".to_string()),
                        value: Some("".to_string()),
                    }),
                    page: Some(PageElement {
                        description: Some("Get paid securely".to_string()),
                        enabled: Some(Enabled(true)),
                        header: Some("Payment Page".to_string()),
                        order: Some(Order(0)),
                    }),
                    payment_button: Some(LabelElement {
                        enabled: Some(Enabled(true)),
                        label: Some("Pay Now".to_string()),
                        order: Some(Order(0)),
                    }),
                    payment_methods: Some(MethodElement {
                        all_methods_checked: Some(true),
                        enabled: Some(Enabled(true)),
                        header: Some("Payment Methods".to_string()),
                        methods: Some(MethodsList {
                            amex: Some(true),
                            apple_pay: Some(true),
                            google_pay: None,
                            discover: Some(true),
                            e_check: Some(true),
                            mastercard: Some(true),
                            visa: Some(true),
                        }),
                        order: Some(Order(0)),
                        settings: None,
                    }),
                    payor: Some(PayorElement {
                        enabled: Some(Enabled(true)),
                        fields: Some(vec![PayorFields {
                            display: Some(true),
                            fixed: Some(true),
                            identifier: Some(true),
                            label: Some("Full Name".to_string()),
                            name: Some("fullName".to_string()),
                            order: Some(Order(0)),
                            required: Some(true),
                            validation: Some("alpha".to_string()),
                            value: Some("".to_string()),
                            width: Some(0),
                        }]),
                        header: Some("Payor Information".to_string()),
                        order: Some(Order(0)),
                    }),
                    review: Some(HeaderElement {
                        enabled: Some(Enabled(true)),
                        header: Some("Review Payment".to_string()),
                        order: Some(Order(0)),
                    }),
                    settings: Some(PagelinkSetting {
                        color: Some("#000000".to_string()),
                        custom_css_url: None,
                        language: Some("en".to_string()),
                        page_logo: None,
                        redirect_after_approve: None,
                        redirect_after_approve_url: None,
                    }),
                },
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**lot_number:** `String` — Lot number of the bills to pay. All bills with this lot number will be included.
    
</dd>
</dl>

<dl>
<dd>

**entry_point:** `Entry` 
    
</dd>
</dl>

<dl>
<dd>

**vendor_number:** `String` — The vendor number for the vendor being paid with this payment link.
    
</dd>
</dl>

<dl>
<dd>

**mail_2:** `Option<String>` — List of recipient email addresses. When there is more than one, separate them by a semicolon (;).
    
</dd>
</dl>

<dl>
<dd>

**amount_fixed:** `Option<String>` — Indicates whether customer can modify the payment amount. A value of `true` means the amount isn't modifiable, a value `false` means the payor can modify the amount to pay.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## PaymentMethodDomain
<details><summary><code>client.payment_method_domain.<a href="/src/api/resources/payment_method_domain/client.rs">add_payment_method_domain</a>(request: AddPaymentMethodDomainRequest) -> Result<AddPaymentMethodDomainApiResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Add a payment method domain to an organization or paypoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    AddPaymentMethodDomainRequestApplePay, AddPaymentMethodDomainRequestGooglePay, DomainName,
    EntityId, EntityType, IsEnabled,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .payment_method_domain
        .add_payment_method_domain(
            &AddPaymentMethodDomainRequest {
                apple_pay: Some(AddPaymentMethodDomainRequestApplePay {
                    is_enabled: Some(IsEnabled(true)),
                }),
                google_pay: Some(AddPaymentMethodDomainRequestGooglePay {
                    is_enabled: Some(IsEnabled(true)),
                }),
                domain_name: Some(DomainName("checkout.example.com".to_string())),
                entity_id: Some(EntityId(109)),
                entity_type: Some(EntityType("paypoint".to_string())),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**apple_pay:** `Option<AddPaymentMethodDomainRequestApplePay>` — Apple Pay configuration information.
    
</dd>
</dl>

<dl>
<dd>

**google_pay:** `Option<AddPaymentMethodDomainRequestGooglePay>` — Google Pay configuration information.
    
</dd>
</dl>

<dl>
<dd>

**domain_name:** `Option<DomainName>` 
    
</dd>
</dl>

<dl>
<dd>

**entity_id:** `Option<EntityId>` 
    
</dd>
</dl>

<dl>
<dd>

**entity_type:** `Option<EntityType>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payment_method_domain.<a href="/src/api/resources/payment_method_domain/client.rs">cascade_payment_method_domain</a>(domain_id: String) -> Result<PaymentMethodDomainGeneralResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Cascades a payment method domain to all child entities. All paypoints and suborganization under this parent will inherit this domain and its settings.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .payment_method_domain
        .cascade_payment_method_domain(&"pmd_b8237fa45c964d8a9ef27160cd42b8c5".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**domain_id:** `String` — The payment method domain's ID in Payabli.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payment_method_domain.<a href="/src/api/resources/payment_method_domain/client.rs">delete_payment_method_domain</a>(domain_id: String) -> Result<DeletePaymentMethodDomainResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a payment method domain. You can't delete an inherited domain, you must delete a domain at the organization level.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .payment_method_domain
        .delete_payment_method_domain(&"pmd_b8237fa45c964d8a9ef27160cd42b8c5".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**domain_id:** `String` — The payment method domain's ID in Payabli.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payment_method_domain.<a href="/src/api/resources/payment_method_domain/client.rs">get_payment_method_domain</a>(domain_id: String) -> Result<PaymentMethodDomainApiResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get the details for a payment method domain.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .payment_method_domain
        .get_payment_method_domain(&"pmd_b8237fa45c964d8a9ef27160cd42b8c5".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**domain_id:** `String` — The payment method domain's ID in Payabli.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payment_method_domain.<a href="/src/api/resources/payment_method_domain/client.rs">list_payment_method_domains</a>(entity_id: Option<Option<String>>, entity_type: Option<Option<String>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>) -> Result<ListPaymentMethodDomainsResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get a list of payment method domains that belong to a PSP, organization, or paypoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .payment_method_domain
        .list_payment_method_domains(
            &ListPaymentMethodDomainsQueryRequest {
                entity_id: Some(1147),
                entity_type: Some("paypoint".to_string()),
                from_record: None,
                limit_record: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entity_id:** `Option<String>` 

Identifier for the organization or paypoint. 
- For organization, provide the organization ID - For paypoint, provide the paypoint ID
    
</dd>
</dl>

<dl>
<dd>

**entity_type:** `Option<String>` 

The type of entity. Valid values: 
  - organization
  - paypoint
  - psp
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — Number of records to skip. Defaults to `0`.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records for query response. Defaults to `20`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payment_method_domain.<a href="/src/api/resources/payment_method_domain/client.rs">update_payment_method_domain</a>(domain_id: String, request: UpdatePaymentMethodDomainRequest) -> Result<PaymentMethodDomainGeneralResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Update a payment method domain's configuration values.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{IsEnabled, UpdatePaymentMethodDomainRequestWallet};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .payment_method_domain
        .update_payment_method_domain(
            &"pmd_b8237fa45c964d8a9ef27160cd42b8c5".to_string(),
            &UpdatePaymentMethodDomainRequest {
                apple_pay: Some(UpdatePaymentMethodDomainRequestWallet {
                    is_enabled: Some(IsEnabled(false)),
                }),
                google_pay: Some(UpdatePaymentMethodDomainRequestWallet {
                    is_enabled: Some(IsEnabled(false)),
                }),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**domain_id:** `String` — The payment method domain's ID in Payabli.
    
</dd>
</dl>

<dl>
<dd>

**apple_pay:** `Option<UpdatePaymentMethodDomainRequestWallet>` 
    
</dd>
</dl>

<dl>
<dd>

**google_pay:** `Option<UpdatePaymentMethodDomainRequestWallet>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.payment_method_domain.<a href="/src/api/resources/payment_method_domain/client.rs">verify_payment_method_domain</a>(domain_id: String) -> Result<PaymentMethodDomainGeneralResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Verify a new payment method domain. If verification is successful, Apple Pay is automatically activated for the domain.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .payment_method_domain
        .verify_payment_method_domain(&"pmd_b8237fa45c964d8a9ef27160cd42b8c5".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**domain_id:** `String` — The payment method domain's ID in Payabli.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Paypoint
<details><summary><code>client.paypoint.<a href="/src/api/resources/paypoint/client.rs">get_basic_entry</a>(entry: String) -> Result<GetBasicEntryResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Gets the basic details for a paypoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .paypoint
        .get_basic_entry(&"8cfec329267".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.paypoint.<a href="/src/api/resources/paypoint/client.rs">get_basic_entry_by_id</a>(id_paypoint: String) -> Result<GetBasicEntryByIdResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the basic details for a paypoint by ID. 
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .paypoint
        .get_basic_entry_by_id(&"198".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id_paypoint:** `String` — Paypoint ID. You can find this value by querying `/api/Query/paypoints/{orgId}`
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.paypoint.<a href="/src/api/resources/paypoint/client.rs">get_entry_config</a>(entry: String, entrypages: Option<Option<String>>) -> Result<GetEntryConfigResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Gets the details for a single paypoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .paypoint
        .get_entry_config(
            &"8cfec329267".to_string(),
            &GetEntryConfigQueryRequest { entrypages: None },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**entrypages:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.paypoint.<a href="/src/api/resources/paypoint/client.rs">get_page</a>(entry: String, subdomain: String) -> Result<PayabliPages, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Gets the details for single payment page for a paypoint. 
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .paypoint
        .get_page(
            &"8cfec329267".to_string(),
            &"pay-your-fees-1".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**subdomain:** `String` — Payment page identifier. The subdomain value is the last portion of the payment page URL. For example, in`https://paypages-sandbox.payabli.com/513823dc10/pay-your-fees-1`, the subdomain is `pay-your-fees-1`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.paypoint.<a href="/src/api/resources/paypoint/client.rs">remove_page</a>(entry: String, subdomain: String) -> Result<PayabliApiResponseGeneric2Part, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes a payment page in a paypoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .paypoint
        .remove_page(
            &"8cfec329267".to_string(),
            &"pay-your-fees-1".to_string(),
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**subdomain:** `String` — Payment page identifier. The subdomain value is the last portion of the payment page URL. For example, in`https://paypages-sandbox.payabli.com/513823dc10/pay-your-fees-1`, the subdomain is `pay-your-fees-1`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.paypoint.<a href="/src/api/resources/paypoint/client.rs">save_logo</a>(entry: String, request: FileContent) -> Result<PayabliApiResponse00Responsedatanonobject, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates a paypoint logo. 
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{FileContent, FileContentFtype};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .paypoint
        .save_logo(
            &"8cfec329267".to_string(),
            &FileContent {
                f_content: None,
                filename: None,
                ftype: None,
                furl: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.paypoint.<a href="/src/api/resources/paypoint/client.rs">settings_page</a>(entry: String) -> Result<SettingsQueryRecord, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves an paypoint's basic settings like custom fields, identifiers, and invoicing settings.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .paypoint
        .settings_page(&"8cfec329267".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.paypoint.<a href="/src/api/resources/paypoint/client.rs">migrate</a>(request: PaypointMoveRequest) -> Result<MigratePaypointResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Migrates a paypoint to a new parent organization.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{Entrypointfield, NotificationRequest, PaypointMoveRequest, WebHeaderParameter};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .paypoint
        .migrate(
            &PaypointMoveRequest {
                entry_point: Entrypointfield("473abc123def".to_string()),
                new_parent_organization_id: 123,
                notification_request: Some(NotificationRequest {
                    notification_url: "https://webhook-test.yoursie.com".to_string(),
                    web_header_parameters: Some(vec![WebHeaderParameter {
                        key: "testheader".to_string(),
                        value: "1234567890".to_string(),
                    }]),
                }),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Query
<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_batch_details</a>(entry: Entry, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryBatchesDetailResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a list of batches and their details, including settled and
unsettled transactions for a paypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response. 
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{Entry, ExportFormat};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_batch_details(
            &Entry("8cfec329267".to_string()),
            &ListBatchDetailsQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `Entry` 
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 


Collection of field names, conditions, and values used to filter the query. 
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.

**List of field names accepted:**

- `settlementDate` (gt, ge, lt, le, eq, ne)
- `depositDate` (gt, ge, lt, le, eq, ne)
- `transId`  (ne, eq, ct, nct)
- `gatewayTransId`  (ne, eq, ct, nct)
- `method`   (in, nin, eq, ne)
- `settledAmount`  (gt, ge, lt, le, eq, ne)
- `operation`    (in, nin, eq, ne)
- `source`   (in, nin, eq, ne)
- `batchNumber`  (ct, nct, eq, ne)
- `payaccountLastfour`   (nct, ct)
- `payaccountType`   (ne, eq, in, nin)
- `customerFirstname`   (ct, nct, eq, ne)
- `customerLastname`    (ct, nct, eq, ne)
- `customerName`   (ct, nct)
- `customerId`  (eq, ne)
- `customerNumber`  (ct, nct, eq, ne)
- `customerCompanyname`    (ct, nct, eq, ne)
- `customerAddress` (ct, nct, eq, ne)
- `customerCity`    (ct, nct, eq, ne)
- `customerZip` (ct, nct, eq, ne)
- `customerState` (ct, nct, eq, ne)
- `customerCountry` (ct, nct, eq, ne)
- `customerPhone` (ct, nct, eq, ne)
- `customerEmail` (ct, nct, eq, ne)
- `customerShippingAddress` (ct, nct, eq, ne)
- `customerShippingCity`    (ct, nct, eq, ne)
- `customerShippingZip` (ct, nct, eq, ne)
- `customerShippingState` (ct, nct, eq, ne)
- `customerShippingCountry` (ct, nct, eq, ne)
- `orgId`  (eq) *mandatory when entry=org*
- `isHold` (eq, ne)
- `paypointId`  (ne, eq)
- `paypointLegal`  (ne, eq, ct, nct)
- `paypointDba`  (ne, eq, ct, nct)
- `orgName`  (ne, eq, ct, nct)
- `batchId` (ct, nct, eq, neq)
- `additional-xxx`  (ne, eq, ct, nct) where xxx is the additional field name

**List of comparison accepted:**
- `eq` or empty => equal
- `gt` => greater than
- `ge` => greater or equal
- `lt` => less than
- `le` => less or equal
- `ne` => not equal
- `ct` => contains
- `nct` => not contains
- `in` => inside array separated by "|"
- `nin` => not inside array separated by "|"

**List of parameters accepted:**

- `limitRecord`: max number of records for query (default="20", "0" or negative value for all)
- `fromRecord`: initial record in query

Example: `settledAmount(gt)=20` returns all records with a `settledAmount` greater than 20.00.
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_batch_details_org</a>(org_id: i64, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryResponseSettlements, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a list of batches and their details, including settled and unsettled transactions for an organization. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_batch_details_org(
            123,
            &ListBatchDetailsOrgQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 


Collection of field names, conditions, and values used to filter the query. 
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>
See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.

**List of field names accepted:**

- `settlementDate` (gt, ge, lt, le, eq, ne)
- `depositDate` (gt, ge, lt, le, eq, ne)
- `transId`  (ne, eq, ct, nct)
- `gatewayTransId`  (ne, eq, ct, nct)
- `method`   (in, nin, eq, ne)
- `settledAmount`  (gt, ge, lt, le, eq, ne)
- `operation`    (in, nin, eq, ne)
- `source`   (in, nin, eq, ne)
- `batchNumber`  (ct, nct, eq, ne)
- `payaccountLastfour`   (nct, ct)
- `payaccountType`   (ne, eq, in, nin)
- `customerFirstname`   (ct, nct, eq, ne)
- `customerLastname`    (ct, nct, eq, ne)
- `customerName`   (ct, nct)
- `customerId`  (eq, ne)
- `customerNumber`  (ct, nct, eq, ne)
- `customerCompanyname`    (ct, nct, eq, ne)
- `customerAddress` (ct, nct, eq, ne)
- `customerCity`    (ct, nct, eq, ne)
- `customerZip` (ct, nct, eq, ne)
- `customerState` (ct, nct, eq, ne)
- `customerCountry` (ct, nct, eq, ne)
- `customerPhone` (ct, nct, eq, ne)
- `customerEmail` (ct, nct, eq, ne)
- `customerShippingAddress` (ct, nct, eq, ne)
- `customerShippingCity`    (ct, nct, eq, ne)
- `customerShippingZip` (ct, nct, eq, ne)
- `customerShippingState` (ct, nct, eq, ne)
- `customerShippingCountry` (ct, nct, eq, ne)
- `orgId`  (eq) *mandatory when entry=org*
- `isHold` (eq, ne)
- `paypointId`  (ne, eq)
- `paypointLegal`  (ne, eq, ct, nct)
- `paypointDba`  (ne, eq, ct, nct)
- `orgName`  (ne, eq, ct, nct)
- `batchId` (ct, nct, eq, neq)
- `additional-xxx`  (ne, eq, ct, nct) where xxx is the additional field name

**List of comparison accepted:**
- `eq` or empty => equal
- `gt` => greater than
- `ge` => greater or equal
- `lt` => less than
- `le` => less or equal
- `ne` => not equal
- `ct` => contains
- `nct` => not contains
- `in` => inside array separated by "|"
- `nin` => not inside array separated by "|"

**List of parameters accepted:**

- `limitRecord`: max number of records for query (default="20", "0" or negative value for all)
- `fromRecord`: initial record in query

Example: `settledAmount(gt)=20` returns all records with a `settledAmount` greater than 20.00.
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_batches</a>(entry: Entry, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryBatchesResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a list of batches for a paypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{Entry, ExportFormat};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_batches(
            &Entry("8cfec329267".to_string()),
            &ListBatchesQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `Entry` 
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query. 
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.

**List of field names accepted:**

- `batchDate` (gt, ge, lt, le, eq, ne)
- `batchNumber` (ne, eq)
- `method` (in, nin, eq, ne)
- `connectorName` (ne, eq, ct, nct)
- `batchAmount` (gt, ge, lt, le, eq, ne)
- `feeBatchAmount` (gt, ge, lt, le, eq, ne)
- `netBatchAmount` (gt, ge, lt, le, eq, ne)
- `releaseAmount` (gt, ge, lt, le, eq, ne)
- `heldAmount` (gt, ge, lt, le, eq, ne)
- `status` (in, nin, eq, ne)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)
- `paypointId` (ne, eq)
- `externalPaypointID` (ct, nct, eq, ne)
- `expectedDepositDate` (gt, ge, lt, le, eq, ne)
- `depositDate` (gt, ge, lt, le, eq, ne)
- `batchRecords` (gt, ge, lt, le, eq, ne)
- `transferId` (ne, eq)
- `transferDate` (gt, ge, lt, le, eq, ne)
- `grossAmount` (gt, ge, lt, le, eq, ne)
- `chargeBackAmount` (gt, ge, lt, le, eq, ne)
- `returnedAmount` (gt, ge, lt, le, eq, ne)
- `billingFeeAmount` (gt, ge, lt, le, eq, ne)
- `thirdPartyPaidAmount` (gt, ge, lt, le, eq, ne)
- `netFundedAmount` (gt, ge, lt, le, eq, ne)
- `adjustmentAmount` (gt, ge, lt, le, eq, ne)
- `processor` (ne, eq, ct, nct)
- `transferStatus` (ne, eq, in, nin)

**List of parameters accepted:**
- `limitRecord`: max number of records for query (default="20", "0" or negative value for all)
- `fromRecord`: initial record in query

Example: `batchAmount(gt)=20` returns all records with a `batchAmount` greater than 20.00
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_batches_org</a>(org_id: i64, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryBatchesResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a list of batches for an org. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_batches_org(
            123,
            &ListBatchesOrgQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query. 
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.

**List of field names accepted:**

- `batchDate` (gt, ge, lt, le, eq, ne)
- `batchNumber` (ne, eq)
- `method` (in, nin, eq, ne)
- `connectorName` (ne, eq, ct, nct)
- `batchAmount` (gt, ge, lt, le, eq, ne)
- `feeBatchAmount` (gt, ge, lt, le, eq, ne)
- `netBatchAmount` (gt, ge, lt, le, eq, ne)
- `releaseAmount` (gt, ge, lt, le, eq, ne)
- `heldAmount` (gt, ge, lt, le, eq, ne)
- `status` (in, nin, eq, ne)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)
- `paypointId` (ne, eq)
- `externalPaypointID` (ct, nct, eq, ne)
- `expectedDepositDate` (gt, ge, lt, le, eq, ne)
- `depositDate` (gt, ge, lt, le, eq, ne)
- `batchRecords` (gt, ge, lt, le, eq, ne)
- `transferId` (ne, eq)
- `transferDate` (gt, ge, lt, le, eq, ne)
- `grossAmount` (gt, ge, lt, le, eq, ne)
- `chargeBackAmount` (gt, ge, lt, le, eq, ne)
- `returnedAmount` (gt, ge, lt, le, eq, ne)
- `billingFeeAmount` (gt, ge, lt, le, eq, ne)
- `thirdPartyPaidAmount` (gt, ge, lt, le, eq, ne)
- `netFundedAmount` (gt, ge, lt, le, eq, ne)
- `adjustmentAmount` (gt, ge, lt, le, eq, ne)
- `processor` (ne, eq, ct, nct)
- `transferStatus` (ne, eq, in, nin)

**List of parameters accepted:**
- `limitRecord`: max number of records for query (default="20", "0" or negative value for all)
- `fromRecord`: initial record in query

Example: `batchAmount(gt)=20` returns all records with a `batchAmount` greater than 20.00      
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_batches_out</a>(entry: Entry, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryBatchesOutResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a list of MoneyOut batches for a paypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{Entry, ExportFormat};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_batches_out(
            &Entry("8cfec329267".to_string()),
            &ListBatchesOutQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `Entry` 
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 


Collection of field names, conditions, and values used to filter the query. See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.

**List of field names accepted**:

- `batchDate` (gt, ge, lt, le, eq, ne)
- `batchNumber` (ne, eq)
- `batchAmount` (gt, ge, lt, le, eq, ne)
- `parentOrgId` (ne, eq, nin, in)
- `status` (in, nin, eq, ne)
- `orgId` (eq)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)
- `paypointId` (ne, eq)
- `externalPaypointID` (ct, nct, eq, ne)
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_batches_out_org</a>(org_id: i64, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryBatchesOutResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a list of MoneyOut batches for an org. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_batches_out_org(
            123,
            &ListBatchesOutOrgQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 


Collection of field names, conditions, and values used to filter the query. 
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.

**List of field names accepted**:

- `batchDate` (gt, ge, lt, le, eq, ne)
- `batchNumber` (ne, eq)
- `batchAmount` (gt, ge, lt, le, eq, ne)
- `parentOrgId` (ne, eq, nin, in)
- `status` (in, nin, eq, ne)
- `orgId` (eq)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)
- `paypointId` (ne, eq)
- `externalPaypointID` (ct, nct, eq, ne)
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_chargebacks</a>(entry: Entry, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryChargebacksResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a list of chargebacks and returned transactions for a paypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{Entry, ExportFormat};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_chargebacks(
            &Entry("8cfec329267".to_string()),
            &ListChargebacksQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `Entry` 
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query.
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>
See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

**List of field names accepted:**
- `chargebackDate` (gt, ge, lt, le, eq, ne)
- `transId`  (ne, eq, ct, nct)
- `method`   (in, nin, eq, ne)
- `netAmount`  (gt, ge, lt, le, eq, ne)
- `reasonCode`   (in, nin, eq, ne)
- `reason`  (ct, nct, eq, ne)
- `replyDate` (gt, ge, lt, le, eq, ne)
- `caseNumber`  (ct, nct, eq, ne)
- `status`   (in, nin, eq, ne)
- `accountType`   (in, nin, eq, ne)
- `payaccountLastfour`   (nct, ct)
- `payaccountType`   (ne, eq, in, nin)
- `customerFirstname`   (ct, nct, eq, ne)
- `customerLastname`    (ct, nct, eq, ne)
- `customerName`   (ct, nct)
- `customerId`  (eq, ne)
- `customerNumber`  (ct, nct, eq, ne)
- `customerCompanyname`    (ct, nct, eq, ne)
- `customerAddress` (ct, nct, eq, ne)
- `customerCity`    (ct, nct, eq, ne)
- `customerZip` (ct, nct, eq, ne)
- `customerState` (ct, nct, eq, ne)
- `customerCountry` (ct, nct, eq, ne)
- `customerPhone` (ct, nct, eq, ne)
- `customerEmail` (ct, nct, eq, ne)
- `customerShippingAddress` (ct, nct, eq, ne)
- `customerShippingCity`    (ct, nct, eq, ne)
- `customerShippingZip` (ct, nct, eq, ne)
- `customerShippingState` (ct, nct, eq, ne)
- `customerShippingCountry` (ct, nct, eq, ne)
- `orgId`  (eq) *mandatory when entry=org*
- `paypointId`  (ne, eq)
- `paypointLegal`  (ne, eq, ct, nct)
- `paypointDba`  (ne, eq, ct, nct)
- `orgName`  (ne, eq, ct, nct)
- `additional-xxx`  (ne, eq, ct, nct) where xxx is the additional field name

**List of comparison accepted - enclosed between parentheses:**
- `eq` or empty => equal
- `gt` => greater than
- `ge` => greater or equal
- `lt` => less than
- `le` => less or equal
- `ne` => not equal
- `ct` => contains
- `nct` => not contains
- `in` => inside array separated by "|"
- `nin` => not inside array separated by "|"

**List of parameters accepted:**
- `limitRecord`: max number of records for query (default="20", "0" or negative value for all)
- `fromRecord`: initial record in query

Example: `netAmount(gt)=20` returns all records with a `netAmount` greater than 20.00
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_chargebacks_org</a>(org_id: i64, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryChargebacksResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a list of chargebacks and returned transactions for an org. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_chargebacks_org(
            123,
            &ListChargebacksOrgQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query.

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info> See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

**List of field names accepted:**

- `chargebackDate` (gt, ge, lt, le, eq, ne)
- `transId`  (ne, eq, ct, nct)
- `method`   (in, nin, eq, ne)
- `netAmount`  (gt, ge, lt, le, eq, ne)
- `reasonCode`   (in, nin, eq, ne)
- `reason`  (ct, nct, eq, ne)
- `replyDate` (gt, ge, lt, le, eq, ne)
- `caseNumber`  (ct, nct, eq, ne)
- `status`   (in, nin, eq, ne)
- `accountType`   (in, nin, eq, ne)
- `payaccountLastfour`   (nct, ct)
- `payaccountType`   (ne, eq, in, nin)
- `customerFirstname`   (ct, nct, eq, ne)
- `customerLastname`    (ct, nct, eq, ne)
- `customerName`   (ct, nct)
- `customerId`  (eq, ne)
- `customerNumber`  (ct, nct, eq, ne)
- `customerCompanyname`    (ct, nct, eq, ne)
- `customerAddress` (ct, nct, eq, ne)
- `customerCity`    (ct, nct, eq, ne)
- `customerZip` (ct, nct, eq, ne)
- `customerState` (ct, nct, eq, ne)
- `customerCountry` (ct, nct, eq, ne)
- `customerPhone` (ct, nct, eq, ne)
- `customerEmail` (ct, nct, eq, ne)
- `customerShippingAddress` (ct, nct, eq, ne)
- `customerShippingCity`    (ct, nct, eq, ne)
- `customerShippingZip` (ct, nct, eq, ne)
- `customerShippingState` (ct, nct, eq, ne)
- `customerShippingCountry` (ct, nct, eq, ne)
- `orgId`  (eq) *mandatory when entry=org*
- `paypointId`  (ne, eq)
- `paypointLegal`  (ne, eq, ct, nct)
- `paypointDba`  (ne, eq, ct, nct)
- `orgName`  (ne, eq, ct, nct)
- `additional-xxx`  (ne, eq, ct, nct) where xxx is the additional field name

**List of comparison accepted - enclosed between parentheses:**

- `eq` or empty => equal
- `gt` => greater than
- `ge` => greater or equal
- `lt` => less than
- `le` => less or equal
- `ne` => not equal
- `ct` => contains
- `nct` => not contains
- `in` => inside array separated by "|"
- `nin` => not inside array separated by "|"

**List of parameters accepted:**
- `limitRecord`: max number of records for query (default="20", "0" or negative value for all)
- `fromRecord`: initial record in query

Example: `netAmount(gt)=20` returns all records with a `netAmount` greater than 20.00
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_customers</a>(entry: Entry, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryCustomerResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a list of customers for a paypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{Entry, ExportFormat};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_customers(
            &Entry("8cfec329267".to_string()),
            &ListCustomersQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `Entry` 
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query. 
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>
See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more details.

**List of Accepted Field Names:**

- `createdDate` (gt, ge, lt, le, eq, ne)
- `customernumber` (ne, eq, ct, nct)
- `firstname` (ne, eq, ct, nct)
- `lastname` (ne, eq, ct, nct)
- `name` (ct, nct)
- `address` (ne, eq, ct, nct)
- `city` (ne, eq, ct, nct)
- `country` (ne, eq, ct, nct)
- `zip` (ne, eq, ct, nct)
- `state` (ne, eq, ct, nct)
- `shippingaddress` (ne, eq, ct, nct)
- `shippingcity` (ne, eq, ct, nct)
- `shippingcountry` (ne, eq, ct, nct)
- `shippingzip` (ne, eq, ct, nct)
- `shippingstate` (ne, eq, ct, nct)
- `phone` (ne, eq, ct, nct)
- `email` (ne, eq, ct, nct)
- `company` (ne, eq, ct, nct)
- `username` (ne, eq, ct, nct)
- `balance` (gt, ge, lt, le, eq, ne)
- `status` (in, nin, eq, ne)
- `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name
- `orgId` (eq) *mandatory when entry=org*
- `paypointId` (ne, eq)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)

**List of Accepted Comparisons:**

- `eq` or empty => equal
- `gt` => greater than
- `ge` => greater or equal
- `lt` => less than
- `le` => less or equal
- `ne` => not equal
- `ct` => contains
- `nct` => not contains
- `in` => inside array separated by "|"
- `nin` => not inside array separated by "|"

**Accepted Parameters:**
- `limitRecord`: Max number of records for query (default="20", "0" or negative value for all)
- `fromRecord`: Initial record in query

**Example Usage:**
`balance(gt)=20` will return all records with a balance greater than 20.00.
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_customers_org</a>(org_id: i64, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryCustomerResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a list of customers for an org. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_customers_org(
            123,
            &ListCustomersOrgQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query. 
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>
See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more details.

**List of Accepted Field Names:**

- `createdDate` (gt, ge, lt, le, eq, ne)
- `customernumber` (ne, eq, ct, nct)
- `firstname` (ne, eq, ct, nct)
- `lastname` (ne, eq, ct, nct)
- `name` (ct, nct)
- `address` (ne, eq, ct, nct)
- `city` (ne, eq, ct, nct)
- `country` (ne, eq, ct, nct)
- `zip` (ne, eq, ct, nct)
- `state` (ne, eq, ct, nct)
- `shippingaddress` (ne, eq, ct, nct)
- `shippingcity` (ne, eq, ct, nct)
- `shippingcountry` (ne, eq, ct, nct)
- `shippingzip` (ne, eq, ct, nct)
- `shippingstate` (ne, eq, ct, nct)
- `phone` (ne, eq, ct, nct)
- `email` (ne, eq, ct, nct)
- `company` (ne, eq, ct, nct)
- `username` (ne, eq, ct, nct)
- `balance` (gt, ge, lt, le, eq, ne)
- `status` (in, nin, eq, ne)
- `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name
- `orgId` (eq) *mandatory when entry=org*
- `paypointId` (ne, eq)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)

**List of Accepted Comparisons:**

- `eq` or empty => equal
- `gt` => greater than
- `ge` => greater or equal
- `lt` => less than
- `le` => less or equal
- `ne` => not equal
- `ct` => contains
- `nct` => not contains
- `in` => inside array separated by "|"
- `nin` => not inside array separated by "|"

**Accepted Parameters:**
- `limitRecord`: Max number of records for query (default="20", "0" or negative value for all)
- `fromRecord`: Initial record in query

**Example Usage:**
`balance(gt)=20` will return all records with a balance greater than 20.00.
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_notification_reports</a>(entry: Entry, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryResponseNotificationReports, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a list of all reports generated in the last 60 days for a single entrypoint. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::Entry;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_notification_reports(
            &Entry("8cfec329267".to_string()),
            &ListNotificationReportsQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `Entry` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

List of field names accepted:
- `reportName` (ct, nct, eq, ne)
- `createdAt` (gt, ge, lt, le, eq, ne)

List of comparison accepted - enclosed between parentheses:
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array
- nin => not inside array

List of parameters accepted:
- limitRecord : max number of records for query (default="20", "0" or negative value for all)
- fromRecord : initial record in query

Example: reportName(ct)=tr  return all records containing the string "tr"
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_notification_reports_org</a>(org_id: i64, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryResponseNotificationReports, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a list of all reports generated in the last 60 days for an organization. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .query
        .list_notification_reports_org(
            123,
            &ListNotificationReportsOrgQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query <Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>
See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

List of field names accepted:
- `reportName` (ct, nct, eq, ne)
- `createdAt` (gt, ge, lt, le, eq, ne)

List of comparison accepted - enclosed between parentheses:
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array
- nin => not inside array

List of parameters accepted:
- limitRecord : max number of records for query (default="20", "0" or negative value for all)
- fromRecord : initial record in query

Example: reportName(ct)=tr  return all records containing the string "tr"
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_notifications</a>(entry: Entry, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryResponseNotifications, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a list of notifications for an entrypoint. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::Entry;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_notifications(
            &Entry("8cfec329267".to_string()),
            &ListNotificationsQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `Entry` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>
See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

List of field names accepted:
- `frequency` (in, nin,ne, eq)
- `method` (in, nin, eq, ne)
- `event` (in, nin, eq, ne)
- `target` (ct, nct, eq, ne)
- `status` (eq, ne)

List of comparison accepted - enclosed between parentheses:
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array
- nin => not inside array

List of parameters accepted:
- limitRecord : max number of records for query (default="20", "0" or negative value for all)
- fromRecord : initial record in query

Example: totalAmount(gt)=20  return all records with totalAmount greater than 20.00
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_notifications_org</a>(org_id: i64, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryResponseNotifications, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Return a list of notifications for an organization. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .query
        .list_notifications_org(
            123,
            &ListNotificationsOrgQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>
See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

List of field names accepted:
- `frequency` (in, nin,ne, eq)
- `method` (in, nin, eq, ne)
- `event` (in, nin, eq, ne)
- `target` (ct, nct, eq, ne)
- `status` (eq, ne)

List of comparison accepted - enclosed between parentheses:
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array
- nin => not inside array

List of parameters accepted:
- limitRecord : max number of records for query (default="20", "0" or negative value for all)
- fromRecord : initial record in query

Example: totalAmount(gt)=20  return all records with totalAmount greater than 20.00
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_organizations</a>(org_id: i64, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<ListOrganizationsResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a list of an organization's suborganizations and their full details such as orgId, users, and settings. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_organizations(
            123,
            &ListOrganizationsQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query.
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>
**List of field names accepted:**

- `createdAt` (gt, ge, lt, le, eq, ne)
- `startDate` (gt, ge, lt, le, eq, ne)
- `dbaname`  (ct, nct)
- `legalname`  (ct, nct)
- `ein`  (ct, nct)
- `address`  (ct, nct)
- `city`  (ct, nct)
- `state`  (ct, nct)
- `phone`  (ct, nct)
- `mcc`  (ct, nct)
- `owntype`  (ct, nct)
- `ownerName`  (ct, nct)
- `contactName`  (ct, nct)
- `orgParentname`  (ct, nct)
- `boardingId` (eq, ne) 
- `entryName`  (ct, nct)

**List of comparison accepted - enclosed between parentheses:**

- `eq` or empty => equal
- `gt` => greater than
- `ge` => greater or equal
- `lt` => less than
- `le` => less or equal
- `ne` => not equal
- `ct` => contains
- `nct` => not contains
- `in` => inside array
- `nin` => not inside array

**List of parameters accepted:**

- `limitRecord` : max number of records for query (default="20", "0" or negative value for all)
- `fromRecord` : initial record in query

Example: `dbaname(ct)=hoa` returns all records with a `dbaname` containing "hoa"
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_payout</a>(entry: Entry, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryPayoutTransaction, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a list of money out transactions (payouts) for a paypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{Entry, ExportFormat};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_payout(
            &Entry("8cfec329267".to_string()),
            &ListPayoutQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `Entry` 
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query.
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

List of field names accepted:

  - `status` (in, nin, eq, ne)
  - `transactionDate` (gt, ge, lt, le, eq, ne)
  - `billNumber` (ct, nct)
  - `vendorNumber` (ct, nct, eq, ne)
  - `vendorName` (ct, nct, eq, ne)
  - `paymentMethod` (ct, nct, eq, ne, in, nin)
  - `paymentId` (ct, nct, eq, ne)
  - `parentOrgId` (ne, eq, nin, in)
  - `batchNumber` (ct, nct, eq, ne)
  - `totalAmount` (gt, ge, lt, le, eq, ne)
  - `paypointLegal` (ne, eq, ct, nct)
  - `paypointDba` (ne, eq, ct, nct)
  - `accountId` (ne, eq, ct, nct)
  - `orgName` (ne, eq, ct, nct)
  - `externalPaypointID` (ct, nct, eq, ne)
  - `paypointId` (eq, ne)
  - `vendorId` (eq, ne)
  - `vendorEIN` (ct, nct, eq, ne)
  - `vendorPhone` (ct, nct, eq, ne)
  - `vendorEmail` (ct, nct, eq, ne)
  - `vendorAddress` (ct, nct, eq, ne)
  - `vendorCity` (ct, nct, eq, ne)
  - `vendorState` (ct, nct, eq, ne)
  - `vendorCountry` (ct, nct, eq, ne)
  - `vendorZip` (ct, nct, eq, ne)
  - `vendorMCC` (ct, nct, eq, ne)
  - `vendorLocationCode` (ct, nct, eq, ne)
  - `vendorCustomField1` (ct, nct, eq, ne)
  - `vendorCustomField2` (ct, nct, eq, ne)
  - `comments` (ct, nct)
  - `payaccountCurrency` (ne, eq, in, nin)
  - `remitAddress` (ct, nct)
  - `source` (ct, nct, eq, ne)
  - `updatedOn` (gt, ge, lt, le, eq, ne)
  - `feeAmount` (gt, ge, lt, le, eq, ne)
  - `lotNumber` (ct, nct)
  - `customerVendorAccount` (ct, nct, eq, ne)
  - `batchId` (eq, ne)
  - `AchTraceNumber` (eq, ne)
  - `payoutProgram`(eq, ne) the options are `managed` or `odp`. For example, `payoutProgram(eq)=managed` returns all records with a `payoutProgram` equal to `managed`. 

  List of comparison accepted - enclosed between parentheses:
  - eq or empty => equal
  - gt => greater than
  - ge => greater or equal
  - lt => less than
  - le => less or equal
  - ne => not equal
  - ct => contains
  - nct => not contains
  - in => inside array separated by \"|\"
  - nin => not inside array separated by \"|\"

  List of parameters accepted:

  - limitRecord : max number of records for query (default=\"20\", \"0\" or negative value for all)
  - fromRecord : initial record in query
  - sortBy : indicate field name and direction to sort the results

  Example: `netAmount(gt)=20` returns all records with a `netAmount` greater than 20.00

  Example: `sortBy=desc(netamount)` returns all records sorted by `netAmount` descending
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_payout_org</a>(org_id: i64, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryPayoutTransaction, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a list of money out transactions (payouts) for an organization. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_payout_org(
            123,
            &ListPayoutOrgQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query.
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>
List of field names accepted:
  
  - `status` (in, nin, eq, ne)
  - `transactionDate` (gt, ge, lt, le, eq, ne)
  - `billNumber` (ct, nct)
  - `vendorNumber` (ct, nct, eq, ne)
  - `vendorName` (ct, nct, eq, ne)
  - `parentOrgId` (ne, eq, nin, in)
  - `paymentMethod` (ct, nct, eq, ne, in, nin)
  - `paymentId` (ct, nct, eq, ne)
  - `batchNumber` (ct, nct, eq, ne)
  - `totalAmount` (gt, ge, lt, le, eq, ne)
  - `paypointLegal` (ne, eq, ct, nct)
  - `paypointDba` (ne, eq, ct, nct)
  - `accountId` (ne, eq, ct, nct)
  - `orgName` (ne, eq, ct, nct)
  - `externalPaypointID` (ct, nct, eq, ne)
  - `paypointId` (eq, ne)
  - `vendorId` (eq, ne)
  - `vendorEIN` (ct, nct, eq, ne)
  - `vendorPhone` (ct, nct, eq, ne)
  - `vendorEmail` (ct, nct, eq, ne)
  - `vendorAddress` (ct, nct, eq, ne)
  - `vendorCity` (ct, nct, eq, ne)
  - `vendorState` (ct, nct, eq, ne)
  - `vendorCountry` (ct, nct, eq, ne)
  - `vendorZip` (ct, nct, eq, ne)
  - `vendorMCC` (ct, nct, eq, ne)
  - `vendorLocationCode` (ct, nct, eq, ne)
  - `vendorCustomField1` (ct, nct, eq, ne)
  - `vendorCustomField2` (ct, nct, eq, ne)
  - `comments` (ct, nct)
  - `payaccountCurrency` (ne, eq, in, nin)
  - `remitAddress` (ct, nct)
  - `source` (ct, nct, eq, ne)
  - `updatedOn` (gt, ge, lt, le, eq, ne)
  - `feeAmount` (gt, ge, lt, le, eq, ne)
  - `lotNumber` (ct, nct)
  - `customerVendorAccount` (ct, nct, eq, ne)
  - `batchId` (eq, ne)
  - `AchTraceNumber` (eq, ne)
  - `payoutProgram`(eq, ne) the options are `managed` or `odp`. For example, `payoutProgram(eq)=managed` returns all records with a `payoutProgram` equal to `managed`.

  List of comparison accepted - enclosed between parentheses:
  - eq or empty => equal
  - gt => greater than
  - ge => greater or equal
  - lt => less than
  - le => less or equal
  - ne => not equal
  - ct => contains
  - nct => not contains
  - in => inside array separated by \"|\"
  - nin => not inside array separated by \"|\"

  List of parameters accepted:

  - limitRecord : max number of records for query (default=\"20\", \"0\" or negative value for all)
  - fromRecord : initial record in query
  - sortBy : indicate field name and direction to sort the results

  Example: `netAmount(gt)=20` returns all records with a `netAmount` greater than 20.00

  Example: `sortBy=desc(netamount)` returns all records sorted by `netAmount` descending
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_paypoints</a>(org_id: i64, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryEntrypointResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a list of paypoints in an organization. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_paypoints(
            123,
            &ListPaypointsQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>
**List of field names accepted:**

- `createdAt` (gt, ge, lt, le, eq, ne)
- `lastModified` (gt, ge, lt, le, eq, ne)
- `startDate` (gt, ge, lt, le, eq, ne)
- `dbaname`  (ct, nct)
- `status` (eq, ne)
- `legalname`  (ct, nct)
- `externalPaypointID` (ct, nct)
- `ein`  (ct, nct)
- `address`  (ct, nct)
- `city`  (ct, nct)
- `state`  (ct, nct)
- `phone`  (ct, nct)
- `mcc`  (ct, nct)
- `owntype`  (ct, nct)
- `ownerName`  (ct, nct)
- `contactName`  (ct, nct)
- `paypointId` (eq, ne)
- `orgParentname`  (ct, nct, in, nin)
- `boardingId` (eq, ne) 
- `entryName`  (ct, nct)
- `externalOrgID` (ct, nct)

**List of comparison accepted - enclosed between parentheses:**

- `eq` or empty => equal
- `gt` => greater than
- `ge` => greater or equal
- `lt` => less than
- `le` => less or equal
- `ne` => not equal
- `ct` => contains
- `nct` => not contains
- `in` => inside array
- `nin` => not inside array

**List of parameters accepted:**

- `limitRecord` : max number of records for query (default="20", "0" or negative value for all)
- `fromRecord` : initial record in query

Example: `dbaname(ct)=hoa` returns all records with a `dbaname` containing "hoa"
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_settlements</a>(entry: Entry, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryResponseSettlements, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a list of settled transactions for a paypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{Entry, ExportFormat};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_settlements(
            &Entry("8cfec329267".to_string()),
            &ListSettlementsQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `Entry` 
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 


Collection of field names, conditions, and values used to filter the query. 
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.

**List of field names accepted:**

- `settlementDate` (gt, ge, lt, le, eq, ne)
- `depositDate` (gt, ge, lt, le, eq, ne)
- `transId`  (ne, eq, ct, nct)
- `gatewayTransId`  (ne, eq, ct, nct)
- `method`   (in, nin, eq, ne)
- `settledAmount`  (gt, ge, lt, le, eq, ne)
- `operation`    (in, nin, eq, ne)
- `source`   (in, nin, eq, ne)
- `batchNumber`  (ct, nct, eq, ne)
- `payaccountLastfour`   (nct, ct)
- `payaccountType`   (ne, eq, in, nin)
- `customerFirstname`   (ct, nct, eq, ne)
- `customerLastname`    (ct, nct, eq, ne)
- `customerName`   (ct, nct)
- `customerId`  (eq, ne)
- `customerNumber`  (ct, nct, eq, ne)
- `customerCompanyname`    (ct, nct, eq, ne)
- `customerAddress` (ct, nct, eq, ne)
- `customerCity`    (ct, nct, eq, ne)
- `customerZip` (ct, nct, eq, ne)
- `customerState` (ct, nct, eq, ne)
- `customerCountry` (ct, nct, eq, ne)
- `customerPhone` (ct, nct, eq, ne)
- `customerEmail` (ct, nct, eq, ne)
- `customerShippingAddress` (ct, nct, eq, ne)
- `customerShippingCity`    (ct, nct, eq, ne)
- `customerShippingZip` (ct, nct, eq, ne)
- `customerShippingState` (ct, nct, eq, ne)
- `customerShippingCountry` (ct, nct, eq, ne)
- `orgId`  (eq) *mandatory when entry=org*
- `isHold` (eq, ne)
- `paypointId`  (ne, eq)
- `paypointLegal`  (ne, eq, ct, nct)
- `paypointDba`  (ne, eq, ct, nct)
- `orgName`  (ne, eq, ct, nct)
- `batchId` (ct, nct, eq, neq)
- `additional-xxx`  (ne, eq, ct, nct) where xxx is the additional field name

**List of comparison accepted:**
- `eq` or empty => equal
- `gt` => greater than
- `ge` => greater or equal
- `lt` => less than
- `le` => less or equal
- `ne` => not equal
- `ct` => contains
- `nct` => not contains
- `in` => inside array separated by "|"
- `nin` => not inside array separated by "|"

**List of parameters accepted:**

- `limitRecord`: max number of records for query (default="20", "0" or negative value for all)
- `fromRecord`: initial record in query

Example: `settledAmount(gt)=20` returns all records with a `settledAmount` greater than 20.00.
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_settlements_org</a>(org_id: i64, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryResponseSettlements, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a list of settled transactions for an organization. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_settlements_org(
            123,
            &ListSettlementsOrgQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 


Collection of field names, conditions, and values used to filter the query. 
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.

**List of field names accepted:**

- `settlementDate` (gt, ge, lt, le, eq, ne)
- `depositDate` (gt, ge, lt, le, eq, ne)
- `transId`  (ne, eq, ct, nct)
- `gatewayTransId`  (ne, eq, ct, nct)
- `method`   (in, nin, eq, ne)
- `settledAmount`  (gt, ge, lt, le, eq, ne)
- `operation`    (in, nin, eq, ne)
- `source`   (in, nin, eq, ne)
- `batchNumber`  (ct, nct, eq, ne)
- `payaccountLastfour`   (nct, ct)
- `payaccountType`   (ne, eq, in, nin)
- `customerFirstname`   (ct, nct, eq, ne)
- `customerLastname`    (ct, nct, eq, ne)
- `customerName`   (ct, nct)
- `customerId`  (eq, ne)
- `customerNumber`  (ct, nct, eq, ne)
- `customerCompanyname`    (ct, nct, eq, ne)
- `customerAddress` (ct, nct, eq, ne)
- `customerCity`    (ct, nct, eq, ne)
- `customerZip` (ct, nct, eq, ne)
- `customerState` (ct, nct, eq, ne)
- `customerCountry` (ct, nct, eq, ne)
- `customerPhone` (ct, nct, eq, ne)
- `customerEmail` (ct, nct, eq, ne)
- `customerShippingAddress` (ct, nct, eq, ne)
- `customerShippingCity`    (ct, nct, eq, ne)
- `customerShippingZip` (ct, nct, eq, ne)
- `customerShippingState` (ct, nct, eq, ne)
- `customerShippingCountry` (ct, nct, eq, ne)
- `orgId`  (eq) *mandatory when entry=org*
- `isHold` (eq, ne)
- `paypointId`  (ne, eq)
- `paypointLegal`  (ne, eq, ct, nct)
- `paypointDba`  (ne, eq, ct, nct)
- `orgName`  (ne, eq, ct, nct)
- `batchId` (ct, nct, eq, neq)
- `additional-xxx`  (ne, eq, ct, nct) where xxx is the additional field name

**List of comparison accepted:**
- `eq` or empty => equal
- `gt` => greater than
- `ge` => greater or equal
- `lt` => less than
- `le` => less or equal
- `ne` => not equal
- `ct` => contains
- `nct` => not contains
- `in` => inside array separated by "|"
- `nin` => not inside array separated by "|"

**List of parameters accepted:**

- `limitRecord`: max number of records for query (default="20", "0" or negative value for all)
- `fromRecord`: initial record in query

Example: `settledAmount(gt)=20` returns all records with a `settledAmount` greater than 20.00.
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_subscriptions</a>(entry: Entry, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QuerySubscriptionResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a list of subscriptions for a single paypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{Entry, ExportFormat};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_subscriptions(
            &Entry("8cfec329267".to_string()),
            &ListSubscriptionsQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `Entry` 
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 


Collection of field names, conditions, and values used to filter the query. 
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>
See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.
      
**List of field names accepted:**

- `startDate` (gt, ge, lt, le, eq, ne)
- `endDate` (gt, ge, lt, le, eq, ne)
- `nextDate` (gt, ge, lt, le, eq, ne)
- `frequency` (in, nin, ne, eq)
- `method` (in, nin, eq, ne)
- `totalAmount` (gt, ge, lt, le, eq, ne)
- `netAmount` (gt, ge, lt, le, eq, ne)
- `feeAmount` (gt, ge, lt, le, eq, ne)
- `status` (in, nin, eq, ne)
- `untilcancelled` (eq, ne)
- `payaccountLastfour` (nct, ct)
- `payaccountType` (ne, eq, in, nin)
- `payaccountCurrency` (ne, eq, in, nin)
- `customerFirstname` (ct, nct, eq, ne)
- `customerLastname` (ct, nct, eq, ne)
- `customerName` (ct, nct)
- `customerId` (eq, ne)
- `customerNumber` (ct, nct, eq, ne)
- `customerCompanyname` (ct, nct, eq, ne)
- `customerAddress` (ct, nct, eq, ne)
- `customerCity` (ct, nct, eq, ne)
- `customerZip` (ct, nct, eq, ne)
- `customerState` (ct, nct, eq, ne)
- `customerCountry` (ct, nct, eq, ne)
- `customerPhone` (ct, nct, eq, ne)
- `customerEmail` (ct, nct, eq, ne)
- `customerShippingAddress` (ct, nct, eq, ne)
- `customerShippingCity` (ct, nct, eq, ne)
- `customerShippingZip` (ct, nct, eq, ne)
- `customerShippingState` (ct, nct, eq, ne)
- `customerShippingCountry` (ct, nct, eq, ne)
- `orgId` (eq)
- `paypointId` (ne, eq)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)
- `externalPaypointId` (ct, nct, ne, eq)
- `subId` (eq, ne)
- `orderDescription` (ct, nct)
- `cycles` (eq, ne, gt, ge, lt, le)
- `leftcycles` (eq, ne, gt, ge, lt, le)
- `createdAt` (eq, ne, gt, ge, lt, le)
- `updatedOn` (eq, ne, gt, ge, lt, le)
- `invoiceNumber` (ct, nct)
- `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name  

**List of comparison operators accepted:**
- `eq` or empty => equal
- `gt` => greater than
- `ge` => greater or equal
- `lt` => less than
- `le` => less or equal
- `ne` => not equal
- `ct` => contains
- `nct` => not contains
- `in` => inside array
- `nin` => not inside array
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_subscriptions_org</a>(org_id: i64, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QuerySubscriptionResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Returns a list of subscriptions for a single org. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_subscriptions_org(
            123,
            &ListSubscriptionsOrgQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 


Collection of field names, conditions, and values used to filter the query. 
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>
See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.
      
**List of field names accepted:**

- `startDate` (gt, ge, lt, le, eq, ne)
- `endDate` (gt, ge, lt, le, eq, ne)
- `nextDate` (gt, ge, lt, le, eq, ne)
- `frequency` (in, nin, ne, eq)
- `method` (in, nin, eq, ne)
- `totalAmount` (gt, ge, lt, le, eq, ne)
- `netAmount` (gt, ge, lt, le, eq, ne)
- `feeAmount` (gt, ge, lt, le, eq, ne)
- `status` (in, nin, eq, ne)
- `untilcancelled` (eq, ne)
- `payaccountLastfour` (nct, ct)
- `payaccountType` (ne, eq, in, nin)
- `payaccountCurrency` (ne, eq, in, nin)
- `customerFirstname` (ct, nct, eq, ne)
- `customerLastname` (ct, nct, eq, ne)
- `customerName` (ct, nct)
- `customerId` (eq, ne)
- `customerNumber` (ct, nct, eq, ne)
- `customerCompanyname` (ct, nct, eq, ne)
- `customerAddress` (ct, nct, eq, ne)
- `customerCity` (ct, nct, eq, ne)
- `customerZip` (ct, nct, eq, ne)
- `customerState` (ct, nct, eq, ne)
- `customerCountry` (ct, nct, eq, ne)
- `customerPhone` (ct, nct, eq, ne)
- `customerEmail` (ct, nct, eq, ne)
- `customerShippingAddress` (ct, nct, eq, ne)
- `customerShippingCity` (ct, nct, eq, ne)
- `customerShippingZip` (ct, nct, eq, ne)
- `customerShippingState` (ct, nct, eq, ne)
- `customerShippingCountry` (ct, nct, eq, ne)
- `orgId` (eq)
- `paypointId` (ne, eq)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)
- `externalPaypointId` (ct, nct, ne, eq)
- `subId` (eq, ne)
- `orderDescription` (ct, nct)
- `cycles` (eq, ne, gt, ge, lt, le)
- `leftcycles` (eq, ne, gt, ge, lt, le)
- `createdAt` (eq, ne, gt, ge, lt, le)
- `updatedOn` (eq, ne, gt, ge, lt, le)
- `invoiceNumber` (ct, nct)
- `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name  

**List of comparison operators accepted:**
- `eq` or empty => equal
- `gt` => greater than
- `ge` => greater or equal
- `lt` => less than
- `le` => less or equal
- `ne` => not equal
- `ct` => contains
- `nct` => not contains
- `in` => inside array
- `nin` => not inside array      
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_transactions</a>(entry: Entry, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryResponseTransactions, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a list of transactions for a paypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
By default, this endpoint returns only transactions from the last 60 days. To query transactions outside of this period, include `transactionDate` filters.
For example, this request parameters filter for transactions between April 01, 2024 and April 09, 2024. 
``` curl --request GET \
  --url https://sandbox.payabli.com/api/Query/transactions/org/1?limitRecord=20&fromRecord=0&transactionDate(ge)=2024-04-01T00:00:00&transactionDate(le)=2024-04-09T23:59:59\
  --header 'requestToken: <api-key>'

  ```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{Entry, ExportFormat};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_transactions(
            &Entry("8cfec329267".to_string()),
            &ListTransactionsQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `Entry` 
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 


Collection of field names, conditions, and values used to filter the query. 
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>
See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.

**List of field names accepted:**

- `transactionDate` (gt, ge, lt, le, eq, ne)
- `transId` (ne, eq, ct, nct, in, nin)
- `gatewayTransId` (ne, eq, ct, nct)
- `orderId` (ne, eq)
- `scheduleId` (ne, eq)
- `returnId` (ne, eq)
- `refundId` (ne, eq)
- `idTrans` (ne, eq)
- `orgId` (ne, eq)
- `paypointId` (ne, eq)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)
- `externalPaypointId` (ct, nct, eq, ne)
- `method` (in, nin, eq, ne)
- `totalAmount` (gt, ge, lt, le, eq, ne)
- `netAmount` (gt, ge, lt, le, eq, ne)
- `feeAmount` (gt, ge, lt, le, eq, ne)
- `operation` (in, nin, eq, ne)
- `source` (in, nin, eq, ne, ct, nct)
- `status` (in, nin, eq, ne)
- `settlementStatus` (in, nin, eq, ne)
- `batchNumber` (nct, ct)
- `invoiceNumber` (ct, nct)
- `ipAddress` (eq, ne)
- `authCode` (ct, nct)
- `orderDescription` (ct, nct)
- `payaccountLastfour` (nct, ct)
- `payaccountType` (ne, eq, in, nin)
- `payaccountCurrency` (ne, eq, in, nin)
- `customerFirstname` (ct, nct, eq, ne)
- `customerLastname` (ct, nct, eq, ne)
- `customerName` (ct, nct)
- `customerId` (eq, ne)
- `customerNumber` (ct, nct, eq, ne)
- `customerCompanyname` (ct, nct, eq, ne)
- `customerAddress` (ct, nct, eq, ne)
- `customerCity` (ct, nct, eq, ne)
- `customerZip` (ct, nct, eq, ne)
- `customerState` (ct, nct, eq, ne)
- `customerCountry` (ct, nct, eq, ne)
- `customerPhone` (ct, nct, eq, ne)
- `customerEmail` (ct, nct, eq, ne)
- `customerShippingAddress` (ct, nct, eq, ne)
- `customerShippingCity` (ct, nct, eq, ne)
- `customerShippingZip` (ct, nct, eq, ne)
- `customerShippingState` (ct, nct, eq, ne)
- `customerShippingCountry` (ct, nct, eq, ne)
- `deviceId` (ct, nct, in, nin, eq, ne)
- `AchSecCode` ( ct, nct, in, nin, eq, ne)
- `AchHolderType` (ct, nct, in, nin, eq, and ne)
- `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name related to customer data
- 'invoiceAdditional-xxx' (ne, eq, ct, nct) where xxx is the additional field name related to invoice data

**List of comparison operators accepted:**
- `eq` or empty => equal
- `gt` => greater than
- `ge` => greater or equal
- `lt` => less than
- `le` => less or equal
- `ne` => not equal
- `ct` => contains
- `nct` => not contains
- `in` => inside array
- `nin` => not inside array      
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_transactions_org</a>(org_id: i64, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryResponseTransactions, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>


Retrieve a list of transactions for an organization. Use filters to
limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.


By default, this endpoint returns only transactions from the last 60 days. To query transactions outside of this period, include `transactionDate` filters.

For example, this request parameters filter for transactions between April 01, 2024 and April 09, 2024. 

```
curl --request GET \
  --url https://sandbox.payabli.com/api/Query/transactions/org/1?limitRecord=20&fromRecord=0&transactionDate(ge)=2024-04-01T00:00:00&transactionDate(le)=2024-04-09T23:59:59\
  --header 'requestToken: <api-key>'

  ```
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_transactions_org(
            123,
            &ListTransactionsOrgQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 


Collection of field names, conditions, and values used to filter the query. 
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>
See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.

**List of field names accepted:**

- `transactionDate` (gt, ge, lt, le, eq, ne)
- `transId` (ne, eq, ct, nct, in, nin)
- `gatewayTransId` (ne, eq, ct, nct)
- `orderId` (ne, eq)
- `scheduleId` (ne, eq)
- `returnId` (ne, eq)
- `refundId` (ne, eq)
- `idTrans` (ne, eq)
- `orgId` (ne, eq)
- `paypointId` (ne, eq)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)
- `externalPaypointId` (ct, nct, eq, ne)
- `method` (in, nin, eq, ne)
- `totalAmount` (gt, ge, lt, le, eq, ne)
- `netAmount` (gt, ge, lt, le, eq, ne)
- `feeAmount` (gt, ge, lt, le, eq, ne)
- `operation` (in, nin, eq, ne)
- `source` (in, nin, eq, ne, ct, nct)
- `status` (in, nin, eq, ne)
- `settlementStatus` (in, nin, eq, ne)
- `batchNumber` (nct, ct)
- `invoiceNumber` (ct, nct)
- `authCode` (ct, nct)
- `orderDescription` (ct, nct)
- `payaccountLastfour` (nct, ct)
- `payaccountType` (ne, eq, in, nin)
- `payaccountCurrency` (ne, eq, in, nin)
- `customerFirstname` (ct, nct, eq, ne)
- `customerLastname` (ct, nct, eq, ne)
- `customerName` (ct, nct)
- `customerId` (eq, ne)
- `customerNumber` (ct, nct, eq, ne)
- `customerCompanyname` (ct, nct, eq, ne)
- `customerAddress` (ct, nct, eq, ne)
- `customerCity` (ct, nct, eq, ne)
- `customerZip` (ct, nct, eq, ne)
- `customerState` (ct, nct, eq, ne)
- `customerCountry` (ct, nct, eq, ne)
- `customerPhone` (ct, nct, eq, ne)
- `customerEmail` (ct, nct, eq, ne)
- `customerShippingAddress` (ct, nct, eq, ne)
- `customerShippingCity` (ct, nct, eq, ne)
- `customerShippingZip` (ct, nct, eq, ne)
- `customerShippingState` (ct, nct, eq, ne)
- `customerShippingCountry` (ct, nct, eq, ne)
- `deviceId` (ct, nct, in, nin, eq, ne)
- `AchSecCode` ( ct, nct, in, nin, eq, ne)
- `AchHolderType`` (ct, nct, in, nin, eq, and ne)
- `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name related to customer data
- 'invoiceAdditional-xxx' (ne, eq, ct, nct) where xxx is the additional field name related to invoice data

**List of comparison operators accepted:**
- `eq` or empty => equal
- `gt` => greater than
- `ge` => greater or equal
- `lt` => less than
- `le` => less or equal
- `ne` => not equal
- `ct` => contains
- `nct` => not contains
- `in` => inside array
- `nin` => not inside array
  
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_transfer_details</a>(entry: Entry, transfer_id: i64, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<LimitRecord>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryTransferDetailResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a list of transfer details records for a paypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{Entry, ExportFormat, LimitRecord};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_transfer_details(
            &Entry("47862acd".to_string()),
            123456,
            &ListTransferDetailsQueryRequest {
                export_format: None,
                from_record: None,
                limit_record: None,
                parameters: None,
                sort_by: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `Entry` 
    
</dd>
</dl>

<dl>
<dd>

**transfer_id:** `i64` — The numeric identifier for the transfer, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<LimitRecord>` 
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 


Collection of field names, conditions, and values used to filter
the query. 

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>

See [Filters and Conditions
Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference)
for more information.


**List of field names accepted:**

  - `grossAmount` (gt, ge, lt, le, eq, ne)
  - `chargeBackAmount` (gt, ge, lt, le, eq, ne)
  - `returnedAmount` (gt, ge, lt, le, eq, ne)
  - `billingFeeAmount` (gt, ge, lt, le, eq, ne)
  - `thirdPartyPaidAmount` (gt, ge, lt, le, eq, ne)
  - `netFundedAmount` (gt, ge, lt, le, eq, ne)
  - `adjustmentAmount` (gt, ge, lt, le, eq, ne)
  - `splitFundingAmount` (gt, ge, lt, le, eq, ne)
  - `operation` (in, nin, eq, ne)
  - `transactionId` (eq, ne, in, nin)
  - `category` (eq, ne, ct, nct)
  - `type` (eq, ne, in, nin)
  - `method` (eq, ne, in, nin)
  
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_transfers</a>(entry: Entry, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<TransferQueryResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a list of transfers for a paypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{Entry, ExportFormat};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_transfers(
            &Entry("47862acd".to_string()),
            &ListTransfersQueryRequest {
                from_record: Some(0),
                limit_record: Some(20),
                export_format: None,
                parameters: None,
                sort_by: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `Entry` 
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query. See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>
List of field names accepted:

  - `transferDate` (gt, ge, lt, le, eq, ne)
  - `grossAmount` (gt, ge, lt, le, eq, ne)
  - `chargeBackAmount` (gt, ge, lt, le, eq, ne)
  - `returnedAmount` (gt, ge, lt, le, eq, ne)
  - `billingFeeAmount` (gt, ge, lt, le, eq, ne)
  - `thirdPartyPaidAmount` (gt, ge, lt, le, eq, ne)
  - `netFundedAmount` (gt, ge, lt, le, eq, ne)
  - `adjustmentAmount` (gt, ge, lt, le, eq, ne)
  - `processor` (ne, eq, ct, nct)
  - `transferStatus` (ne, eq, in, nin)
  - `batchNumber` (ne, eq, ct, nct)
  - `batchId` (ne, eq, in, nin)
  - `transferId` (in, nin, eq, ne)
  - `bankAccountNumber` (ct, nct, ne, eq)
  - `bankRoutingNumber` (ct, nct, ne, eq)
  - `batchCurrency` (in, nin, ne, eq)
  - `parentOrgName` (ct, nct, ne, eq)
  - `parentOrgId` (ct, nct, ne, eq)
  - `externalPaypointID` (ct, nct)
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_transfers_org</a>(org_id: Orgid, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<TransferQueryResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a list of transfers for an org. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{ExportFormat, Orgid};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_transfers_org(
            &Orgid(123),
            &ListTransfersOrgQueryRequest {
                from_record: Some(0),
                limit_record: Some(20),
                export_format: None,
                parameters: None,
                sort_by: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**org_id:** `Orgid` 
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query. See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>
List of field names accepted:

  - `transferDate` (gt, ge, lt, le, eq, ne)
  - `grossAmount` (gt, ge, lt, le, eq, ne)
  - `chargeBackAmount` (gt, ge, lt, le, eq, ne)
  - `returnedAmount` (gt, ge, lt, le, eq, ne)
  - `billingFeeAmount` (gt, ge, lt, le, eq, ne)
  - `thirdPartyPaidAmount` (gt, ge, lt, le, eq, ne)
  - `netFundedAmount` (gt, ge, lt, le, eq, ne)
  - `adjustmentAmount` (gt, ge, lt, le, eq, ne)
  - `processor` (ne, eq, ct, nct)
  - `transferStatus` (ne, eq, in, nin)
  - `batchNumber` (ne, eq, ct, nct)
  - `batchId` (ne, eq, in, nin)
  - `transferId` (in, nin, eq, ne)
  - `bankAccountNumber` (ct, nct, ne, eq)
  - `bankRoutingNumber` (ct, nct, ne, eq)
  - `batchCurrency` (in, nin, ne, eq)
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_users_org</a>(org_id: i64, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryUserResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get list of users for an org. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .query
        .list_users_org(
            123,
            &ListUsersOrgQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query.
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>
See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

**List of field names accepted:**

- `createdDate` (gt, ge, lt, le, eq, ne)
- `name`  (ne, eq, ct, nct)
- `email`  (ne, eq, ct, nct)
- `status`   (in, nin, eq, ne)
- `role.xxx`  (ne, eq, ct, nct) where xxx is the role field: `roleLabel` or `roleValue`

**List of comparison accepted - enclosed between parentheses:**

- `eq` or empty => equal
- `gt` => greater than
- `ge` => greater or equal
- `lt` => less than
- `le` => less or equal
- `ne` => not equal
- `ct` => contains
- `nct` => not contains
- `in` => inside array separated by "|"
- `nin` => not inside array separated by "|"

**List of parameters accepted:**
- `limitRecord`: max number of records for query (default="20", "0" or negative value for all)
- `fromRecord`: initial record in query

Example: `name(ct)=john`  return all records with name containing 'john'.
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_users_paypoint</a>(entry: String, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryUserResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Get list of users for a paypoint. Use filters to limit results.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .query
        .list_users_paypoint(
            &"8cfec329267".to_string(),
            &ListUsersPaypointQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query.
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>
See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

**List of field names accepted:**

- `createdDate` (gt, ge, lt, le, eq, ne)
- `name`  (ne, eq, ct, nct)
- `email`  (ne, eq, ct, nct)
- `status`   (in, nin, eq, ne)
- `role.xxx`  (ne, eq, ct, nct) where xxx is the role field: `roleLabel` or `roleValue`

**List of comparison accepted - enclosed between parentheses:**

- `eq` or empty => equal
- `gt` => greater than
- `ge` => greater or equal
- `lt` => less than
- `le` => less or equal
- `ne` => not equal
- `ct` => contains
- `nct` => not contains
- `in` => inside array separated by "|"
- `nin` => not inside array separated by "|"

**List of parameters accepted:**
- `limitRecord`: max number of records for query (default="20", "0" or negative value for all)
- `fromRecord`: initial record in query

Example: `name(ct)=john`  return all records with name containing 'john'
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_vendors</a>(entry: String, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryResponseVendors, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a list of vendors for an entrypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_vendors(
            &"8cfec329267".to_string(),
            &ListVendorsQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `String` — The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>
See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

List of field names accepted:
- `method` (in, nin, eq, ne)
- `enrollmentStatus` (in,nin, eq, ne)
- `status` (in, nin, eq, ne)
- `vendorNumber` (ct, nct, eq, ne)
- `name` (ct, nct, eq, ne)
- `ein` (ct, nct, eq, ne)
- `phone` (ct, nct, eq, ne)
- `email` (ct, nct, eq, ne)
- `address` (ct, nct, eq, ne)
- `city` (ct, nct, eq, ne)
- `state` (ct, nct, eq, ne)
- `country` (ct, nct, eq, ne)
- `zip` (ct, nct, eq, ne)
- `mcc` (ct, nct, eq, ne)
- `locationCode` (ct, nct, eq, ne)
- `paypointLegal` (ne, eq, ct, nct)
- `parentOrgId` (ne, eq, nin, in)
- `paypointDba` (ne, eq, ct, nct)
- `orgName` (ne, eq, ct, nct)

List of comparison accepted - enclosed between parentheses:
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array separated by "|"
- nin => not inside array separated by "|"

List of parameters accepted:
- limitRecord : max number of records for query (default="20", "0" or negative value for all)
- fromRecord : initial record in query

Example: `netAmount(gt)=20` returns all records with a `netAmount` greater than 20.00
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_vendors_org</a>(org_id: i64, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<QueryResponseVendors, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a list of vendors for an organization. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_vendors_org(
            123,
            &ListVendorsOrgQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>
See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

List of field names accepted:
- `method` (in, nin, eq, ne)
- `enrollmentStatus` (in,nin, eq, ne)
- `status` (in, nin, eq, ne)
- `vendorNumber` (ct, nct, eq, ne)
- `name` (ct, nct, eq, ne)
- `ein` (ct, nct, eq, ne)
- `phone` (ct, nct, eq, ne)
- `email` (ct, nct, eq, ne)
- `address` (ct, nct, eq, ne)
- `city` (ct, nct, eq, ne)
- `state` (ct, nct, eq, ne)
- `country` (ct, nct, eq, ne)
- `zip` (ct, nct, eq, ne)
- `mcc` (ct, nct, eq, ne)
- `locationCode` (ct, nct, eq, ne)
- `paypointLegal` (ne, eq, ct, nct)
- `paypointDba` (ne, eq, ct, nct)
- `parentOrgId` (ne, eq, nin, in)
- `orgName` (ne, eq, ct, nct)

List of comparison accepted - enclosed between parentheses:
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array separated by "|"
- nin => not inside array separated by "|"

List of parameters accepted:
- limitRecord : max number of records for query (default="20", "0" or negative value for all)
- fromRecord : initial record in query

Example: `netAmount(gt)=20` returns all records with a `netAmount` greater than 20.00
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_vcards</a>(entry: Entry, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<VCardQueryResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a list of vcards (virtual credit cards) issued for an entrypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{Entry, ExportFormat};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_vcards(
            &Entry("8cfec329267".to_string()),
            &ListVcardsQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `Entry` 
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query. 
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>
List of field names accepted:  

  - `status` (in, nin, eq, ne)  
  - `createdAt` (gt, ge, lt, le, eq, ne)  
  - `cardToken` (ct, nct, eq, ne)  
  - `lastFour` (ct, nct, eq, ne)  
  - `expirationDate` (ct, nct, eq, ne)  
  - `payoutId` (ct, nct, eq, ne, in, nin)  
  - `vendorId` (ct, nct, eq, ne, in, nin)  
  - `miscData1` (ct, nct, eq, ne)  
  - `miscData2` (ct, nct, eq, ne)  
  - `currentUses` (gt, ge, lt, le, eq, ne)  
  - `amount` (gt, ge, lt, le, eq, ne)  
  - `balance` (gt, ge, lt, le, eq, ne)  
  - `paypointLegal` (ne, eq, ct, nct)  
  - `paypointDba` (ne, eq, ct, nct)  
  - `orgName` (ne, eq, ct, nct)  
  - `externalPaypointId` (ct, nct, eq, ne)  
  - `paypointId` (in, nin, eq, ne)  

List of comparison accepted - enclosed between parentheses:  

  - eq or empty => equal  
  - gt => greater than  
  - ge => greater or equal  
  - lt => less than  
  - le => less or equal  
  - ne => not equal  
  - ct => contains  
  - nct => not contains  
  - in => inside array separated by "|"  
  - nin => not inside array separated by "|"
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.query.<a href="/src/api/resources/query/client.rs">list_vcards_org</a>(org_id: i64, export_format: Option<Option<ExportFormat>>, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<VCardQueryResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve a list of vcards (virtual credit cards) issued for an organization. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::ExportFormat;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .query
        .list_vcards_org(
            123,
            &ListVcardsOrgQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                export_format: None,
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**export_format:** `Option<ExportFormat>` 
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 

Collection of field names, conditions, and values used to filter the query. 
<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>
List of field names accepted:  

  - `status` (in, nin, eq, ne)  
  - `createdAt` (gt, ge, lt, le, eq, ne)  
  - `cardToken` (ct, nct, eq, ne)  
  - `lastFour` (ct, nct, eq, ne)  
  - `expirationDate` (ct, nct, eq, ne)  
  - `payoutId` (ct, nct, eq, ne, in, nin)  
  - `vendorId` (ct, nct, eq, ne, in, nin)  
  - `miscData1` (ct, nct, eq, ne)  
  - `miscData2` (ct, nct, eq, ne)  
  - `currentUses` (gt, ge, lt, le, eq, ne)  
  - `amount` (gt, ge, lt, le, eq, ne)  
  - `balance` (gt, ge, lt, le, eq, ne)  
  - `paypointLegal` (ne, eq, ct, nct)  
  - `paypointDba` (ne, eq, ct, nct)  
  - `orgName` (ne, eq, ct, nct)  
  - `externalPaypointId` (ct, nct, eq, ne)  
  - `paypointId` (in, nin, eq, ne)  

List of comparison accepted - enclosed between parentheses:  

  - eq or empty => equal  
  - gt => greater than  
  - ge => greater or equal  
  - lt => less than  
  - le => less or equal  
  - ne => not equal  
  - ct => contains  
  - nct => not contains  
  - in => inside array separated by "|"  
  - nin => not inside array separated by "|"
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Statistic
<details><summary><code>client.statistic.<a href="/src/api/resources/statistic/client.rs">basic_stats</a>(mode: String, freq: String, level: i64, entry_id: String, end_date: Option<Option<String>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, start_date: Option<Option<String>>) -> Result<Vec<StatBasicExtendedQueryRecord>, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the basic statistics for an organization or a paypoint, for a given time period, grouped by a particular frequency. 
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .statistic
        .basic_stats(
            &"ytd".to_string(),
            &"m".to_string(),
            1,
            1000000,
            &BasicStatsQueryRequest {
                end_date: Some("2025-11-01".to_string()),
                start_date: Some("2025-11-30".to_string()),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**mode:** `String` 

Mode for the request. Allowed values:

- `custom` - Allows you to set a custom date range
- `ytd` - Year To Date
- `mtd` - Month To Date
- `wtd` - Week To Date
- `today` - All current day
- `m12` - Last 12 months
- `d30` - Last 30 days
- `h24` - Last 24 hours
- `lasty` - Last Year
- `lastm` - Last Month
- `lastw` - Last Week
- `yesterday` - Last Day
  
    
</dd>
</dl>

<dl>
<dd>

**freq:** `String` 

Frequency to group series. Allowed values:

- `m` - monthly
- `w` - weekly
- `d` - daily
- `h` - hourly

For example, `w` groups the results by week.
    
</dd>
</dl>

<dl>
<dd>

**level:** `i64` 

The entry level for the request: 
  - 0 for Organization
  - 2 for Paypoint
    
</dd>
</dl>

<dl>
<dd>

**entry_id:** `String` — Identifier in Payabli for the entity.
    
</dd>
</dl>

<dl>
<dd>

**end_date:** `Option<String>` 

Used with `custom` mode. The end date for the range. 
Valid formats:
  - YYYY-mm-dd
  - YYYY/mm/dd
  - mm-dd-YYYY
  - mm/dd/YYYY
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` — List of parameters.
    
</dd>
</dl>

<dl>
<dd>

**start_date:** `Option<String>` 

Used with `custom` mode. The start date for the range. 
Valid formats:
   - YYYY-mm-dd
   - YYYY/mm/dd
   -  mm-dd-YYYY
   - mm/dd/YYYY
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.statistic.<a href="/src/api/resources/statistic/client.rs">customer_basic_stats</a>(mode: String, freq: String, customer_id: i64, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<Vec<SubscriptionStatsQueryRecord>, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the basic statistics for a customer for a specific time period, grouped by a selected frequency. 
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .statistic
        .customer_basic_stats(
            &"ytd".to_string(),
            &"m".to_string(),
            998,
            &CustomerBasicStatsQueryRequest { parameters: None },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**mode:** `String` 

Mode for request. Allowed values:

- `ytd` - Year To Date
- `mtd` - Month To Date
- `wtd` - Week To Date
- `today` - All current day
- `m12` - Last 12 months
- `d30` - Last 30 days
- `h24` - Last 24 hours
- `lasty` - Last Year
- `lastm` - Last Month
- `lastw` - Last Week
- `yesterday` - Last Day
    
</dd>
</dl>

<dl>
<dd>

**freq:** `String` 

Frequency to group series. Allowed values:

- `m` - monthly
- `w` - weekly
- `d` - daily
- `h` - hourly

For example, `w` groups the results by week.
    
</dd>
</dl>

<dl>
<dd>

**customer_id:** `i64` — Payabli-generated customer ID. Maps to "Customer ID" column in PartnerHub. 
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` — List of parameters.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.statistic.<a href="/src/api/resources/statistic/client.rs">sub_stats</a>(interval: String, level: i64, entry_id: String, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<Vec<StatBasicQueryRecord>, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves the subscription statistics for a given interval for a paypoint or organization.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .statistic
        .sub_stats(
            &"30".to_string(),
            1,
            1000000,
            &SubStatsQueryRequest { parameters: None },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**interval:** `String` 

Interval to get the data. Allowed values:

- `all` - all intervals
- `30` - 1-30 days
- `60` - 31-60 days
- `90` - 61-90 days
- `plus` - +90 days
    
</dd>
</dl>

<dl>
<dd>

**level:** `i64` 

The entry level for the request: 
  - 0 for Organization
  - 2 for Paypoint
    
</dd>
</dl>

<dl>
<dd>

**entry_id:** `String` — Identifier in Payabli for the entity.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` — List of parameters
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.statistic.<a href="/src/api/resources/statistic/client.rs">vendor_basic_stats</a>(mode: String, freq: String, id_vendor: i64, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>) -> Result<Vec<StatisticsVendorQueryRecord>, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieve the basic statistics about a vendor for a given time period, grouped by frequency. 
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .statistic
        .vendor_basic_stats(
            &"ytd".to_string(),
            &"m".to_string(),
            1,
            &VendorBasicStatsQueryRequest { parameters: None },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**mode:** `String` 

Mode for request. Allowed values:

- `ytd` - Year To Date
- `mtd` - Month To Date
- `wtd` - Week To Date
- `today` - All current day
- `m12` - Last 12 months
- `d30` - Last 30 days
- `h24` - Last 24 hours
- `lasty` - Last Year
- `lastm` - Last Month
- `lastw` - Last Week
- `yesterday` - Last Day
    
</dd>
</dl>

<dl>
<dd>

**freq:** `String` 

Frequency to group series. Allowed values:

- `m` - monthly
- `w` - weekly
- `d` - daily
- `h` - hourly

For example, `w` groups the results by week.
    
</dd>
</dl>

<dl>
<dd>

**id_vendor:** `i64` — Vendor ID.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` — List of parameters
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Subscription
<details><summary><code>client.subscription.<a href="/src/api/resources/subscription/client.rs">get_subscription</a>(sub_id: i64) -> Result<SubscriptionQueryRecords, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a single subscription's details.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.subscription.get_subscription(263, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**sub_id:** `i64` — The subscription ID. 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.subscription.<a href="/src/api/resources/subscription/client.rs">new_subscription</a>(request: SubscriptionRequestBody, force_customer_creation: Option<Option<ForceCustomerCreation>>) -> Result<AddSubscriptionResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a subscription or scheduled payment to run at a specified time and frequency. 
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    AchHolder, AchHolderType, AchSecCode, Achaccount, Achaccounttype, Achrouting, AdditionalData,
    AdditionalDataString, Attachments, BillData, BillDataPaymentTerms, BillItem,
    BillingAddressAddtlNullable, BillingAddressNullable, BillingCityNullable,
    BillingCountryNullable, BillingStateNullable, BillingZip, Cardcvv, Cardexp, Cardholder,
    Cardnumber, Cardzip, CustomerId, CustomerNumberNullable, Datenullable, Device, Discount,
    DutyAmount, Email, Entrypointfield, FileContent, FileContentFtype, ForceCustomerCreation,
    FreightAmount, Frequency, IdempotencyKey, Identifierfields, Initiator, InvoiceAmount,
    InvoiceNumber, InvoiceType, Invoicestatus, ItemCommodityCode, ItemDescription, ItemProductCode,
    ItemProductName, ItemUnitofMeasure, PayMethodAch, PayMethodCredit, PaymentCategories,
    PaymentDetail, PayorDataRequest, PhoneNumber, PurchaseOrder, RequestSchedulePaymentMethod,
    RequestSchedulePaymentMethodInitiator, SaveIfSuccess, ScheduleDetail, SetPause,
    ShippingFromZip, Shippingaddress, Shippingaddressadditional, Shippingcity, Shippingcountry,
    Shippingstate, Shippingzip, Source, SplitFunding, SplitFundingContent, StoredMethodUsageType,
    Storedmethodid, Subdomain, SubscriptionRequestBody, SummaryCommodityCode, Tax, TermsConditions,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .subscription
        .new_subscription(
            &NewSubscriptionRequest {
                body: SubscriptionRequestBody {
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
                    payment_details: Some(PaymentDetail {
                        categories: None,
                        check_image: None,
                        check_number: None,
                        currency: None,
                        service_fee: Some(0.0),
                        split_funding: None,
                        total_amount: 100.0,
                    }),
                    payment_method: Some(RequestSchedulePaymentMethod::PayMethodCredit(
                        PayMethodCredit {
                            cardcvv: Some(Cardcvv("123".to_string())),
                            cardexp: Cardexp("02/25".to_string()),
                            card_holder: Some(Cardholder("John Cassian".to_string())),
                            cardnumber: Cardnumber("4111111111111111".to_string()),
                            cardzip: Some(Cardzip("37615".to_string())),
                            initiator: Some(Initiator("payor".to_string())),
                            method: "card".to_string(),
                            save_if_success: None,
                        },
                    )),
                    schedule_details: Some(ScheduleDetail {
                        end_date: Some("03-20-2025".to_string()),
                        frequency: Some(Frequency::Weekly),
                        plan_id: Some(1),
                        start_date: Some("09-20-2024".to_string()),
                    }),
                    set_pause: None,
                    source: None,
                    subdomain: None,
                },
                force_customer_creation: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**force_customer_creation:** `Option<ForceCustomerCreation>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.subscription.<a href="/src/api/resources/subscription/client.rs">remove_subscription</a>(sub_id: i64) -> Result<RemoveSubscriptionResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes a subscription, autopay, or recurring payment and prevents future charges.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.subscription.remove_subscription(396, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**sub_id:** `i64` — The subscription ID. 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.subscription.<a href="/src/api/resources/subscription/client.rs">update_subscription</a>(sub_id: i64, request: RequestUpdateSchedule) -> Result<UpdateSubscriptionResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates a subscription's details.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    Frequency, PaymentCategories, PaymentDetail, ScheduleDetail, SetPause, SplitFunding,
    SplitFundingContent,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .subscription
        .update_subscription(
            231,
            &RequestUpdateSchedule {
                set_pause: Some(SetPause(true)),
                payment_details: None,
                schedule_details: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**sub_id:** `i64` — The subscription ID. 
    
</dd>
</dl>

<dl>
<dd>

**payment_details:** `Option<PaymentDetail>` — Object describing details of the payment. To skip the payment, set the `totalAmount` to 0. Payments will be paused until the amount is updated to a non-zero value. When `totalAmount` is set to 0, the `serviceFee` must also be set to 0.
    
</dd>
</dl>

<dl>
<dd>

**schedule_details:** `Option<ScheduleDetail>` — Object describing the schedule for subscription
    
</dd>
</dl>

<dl>
<dd>

**set_pause:** `Option<SetPause>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Templates
<details><summary><code>client.templates.<a href="/src/api/resources/templates/client.rs">delete_template</a>(template_id: f64) -> Result<PayabliApiResponseTemplateId, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes a template by ID. 
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.templates.delete_template(80.0, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**template_id:** `f64` — The boarding template ID. Can be found at the end of the boarding template URL in PartnerHub. Example: `https://partner-sandbox.payabli.com/myorganization/boarding/edittemplate/80`. Here, the template ID is `80`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.templates.<a href="/src/api/resources/templates/client.rs">getlink_template</a>(template_id: f64, ignore_empty: bool) -> Result<BoardingLinkApiResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Generates a boarding link from a boarding template.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.templates.getlink_template(80.0, true, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**template_id:** `f64` — The boarding template ID. Can be found at the end of the boarding template URL in PartnerHub. Example: `https://partner-sandbox.payabli.com/myorganization/boarding/edittemplate/80`. Here, the template ID is `80`.
    
</dd>
</dl>

<dl>
<dd>

**ignore_empty:** `bool` — Ignore read-only and empty fields Default is `false`. If `ignoreEmpty` = `false` and any field is empty, then the request returns a failure response. If `ignoreEmpty` = `true`, the request returns the boarding link name regardless of whether fields are empty.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.templates.<a href="/src/api/resources/templates/client.rs">get_template</a>(template_id: f64) -> Result<TemplateQueryRecord, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a boarding template's details by ID.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.templates.get_template(80.0, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**template_id:** `f64` — The boarding template ID. Can be found at the end of the boarding template URL in PartnerHub. Example: `https://partner-sandbox.payabli.com/myorganization/boarding/edittemplate/80`. Here, the template ID is `80`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.templates.<a href="/src/api/resources/templates/client.rs">list_templates</a>(org_id: i64, from_record: Option<Option<i64>>, limit_record: Option<Option<i64>>, parameters: Option<Option<std::collections::HashMap<String, Option<String>>>>, sort_by: Option<Option<String>>) -> Result<TemplateQueryResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a list of boarding templates for an organization. Use filters to limit results. You can't make a request that includes filters from the API console in the documentation. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .templates
        .list_templates(
            123,
            &ListTemplatesQueryRequest {
                from_record: Some(251),
                limit_record: Some(0),
                sort_by: Some("desc(field_name)".to_string()),
                parameters: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**org_id:** `i64` — The numeric identifier for organization, assigned by Payabli.
    
</dd>
</dl>

<dl>
<dd>

**from_record:** `Option<i64>` — The number of records to skip before starting to collect the result set.
    
</dd>
</dl>

<dl>
<dd>

**limit_record:** `Option<i64>` — Max number of records to return for the query. Use `0` or negative value to return all records.
    
</dd>
</dl>

<dl>
<dd>

**parameters:** `Option<std::collections::HashMap<String, Option<String>>>` 


Collection of field names, conditions, and values used to filter the query.

<Info>
  **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**

  Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.

  For example:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20

  should become:

  --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
</Info>


See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.

List of field names accepted:
- `createdAt` (gt, ge, lt, le, eq, ne)
- `title` (ct, nct)
- `description` (ct, nct)
- `code` (ct, nct)
- `orgParentname` (ct, nct)

List of comparison accepted - enclosed between parentheses:
- eq or empty => equal
- gt => greater than
- ge => greater or equal
- lt => less than
- le => less or equal
- ne => not equal
- ct => contains
- nct => not contains
- in => inside array
- nin => not inside array

List of parameters accepted:
- limitRecord : max number of records for query (default="20", "0" or negative value for all)
- fromRecord : initial record in query

Example: title(ct)=hoa return all records with title containing "hoa"
    
</dd>
</dl>

<dl>
<dd>

**sort_by:** `Option<String>` — The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## TokenStorage
<details><summary><code>client.token_storage.<a href="/src/api/resources/token_storage/client.rs">add_method</a>(request: RequestTokenStorage, ach_validation: Option<Option<AchValidation>>, create_anonymous: Option<CreateAnonymous>, force_customer_creation: Option<Option<ForceCustomerCreation>>, temporary: Option<Temporary>) -> Result<AddMethodResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Saves a payment method for reuse. This call exchanges sensitive payment information for a token that can be used to process future transactions. The `ReferenceId` value in the response is the `storedMethodId` to use with transactions.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    AchHolder, AchHolderType, AchSecCode, AchValidation, Achaccount, Achaccounttype, Achrouting,
    AdditionalData, BillingAddressAddtlNullable, BillingAddressNullable, BillingCityNullable,
    BillingCountryNullable, BillingStateNullable, BillingZip, Cardcvv, Cardexp, Cardholder,
    Cardnumber, Cardzip, ConvertToken, CreateAnonymous, CustomerId, CustomerNumberNullable, Device,
    Email, Entrypointfield, ForceCustomerCreation, IdempotencyKey, Identifierfields,
    PayorDataRequest, PhoneNumber, RequestTokenStorage, RequestTokenStoragePaymentMethod,
    Shippingaddress, Shippingaddressadditional, Shippingcity, Shippingcountry, Shippingstate,
    Shippingzip, Source, Subdomain, Temporary, TokenizeAch, TokenizeCard, VendorDataRequest,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .token_storage
        .add_method(
            &AddMethodRequest {
                body: RequestTokenStorage {
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
                    fallback_auth: Some(true),
                    fallback_auth_amount: None,
                    method_description: None,
                    payment_method: Some(RequestTokenStoragePaymentMethod::TokenizeCard(
                        TokenizeCard {
                            method: "card".to_string(),
                            cardcvv: Some(Cardcvv("123".to_string())),
                            cardexp: Cardexp("02/25".to_string()),
                            card_holder: Cardholder("John Doe".to_string()),
                            cardnumber: Cardnumber("4111111111111111".to_string()),
                            cardzip: Some(Cardzip("12345".to_string())),
                        },
                    )),
                    vendor_data: None,
                    source: None,
                    subdomain: None,
                },
                ach_validation: None,
                create_anonymous: CreateAnonymous(Default::default()),
                force_customer_creation: None,
                temporary: Temporary(Default::default()),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**ach_validation:** `Option<AchValidation>` 
    
</dd>
</dl>

<dl>
<dd>

**create_anonymous:** `CreateAnonymous` 
    
</dd>
</dl>

<dl>
<dd>

**force_customer_creation:** `Option<ForceCustomerCreation>` 
    
</dd>
</dl>

<dl>
<dd>

**temporary:** `Temporary` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.token_storage.<a href="/src/api/resources/token_storage/client.rs">get_method</a>(method_id: String, card_expiration_format: Option<Option<i64>>, include_temporary: Option<Option<bool>>) -> Result<GetMethodResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves details for a saved payment method.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .token_storage
        .get_method(
            &"32-8877drt00045632-678".to_string(),
            &GetMethodQueryRequest {
                card_expiration_format: Some(1),
                include_temporary: Some(false),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**method_id:** `String` — The saved payment method ID.
    
</dd>
</dl>

<dl>
<dd>

**card_expiration_format:** `Option<i64>` 

Format for card expiration dates in the response. 

Accepted values:
  
- 0: default, no formatting. Expiration dates are returned in the format they're saved in.

- 1: MMYY
 
- 2: MM/YY
    
</dd>
</dl>

<dl>
<dd>

**include_temporary:** `Option<bool>` — When `true`, the request will include temporary tokens in the search and return details for a matching temporary token. The default behavior searches only for permanent tokens.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.token_storage.<a href="/src/api/resources/token_storage/client.rs">remove_method</a>(method_id: String) -> Result<PayabliApiResponsePaymethodDelete, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Deletes a saved payment method.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .token_storage
        .remove_method(&"32-8877drt00045632-678".to_string(), None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**method_id:** `String` — The saved payment method ID.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.token_storage.<a href="/src/api/resources/token_storage/client.rs">update_method</a>(method_id: String, request: RequestTokenStorage, ach_validation: Option<Option<AchValidation>>) -> Result<PayabliApiResponsePaymethodDelete, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates a saved payment method.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    AchHolder, AchHolderType, AchSecCode, AchValidation, Achaccount, Achaccounttype, Achrouting,
    AdditionalData, BillingAddressAddtlNullable, BillingAddressNullable, BillingCityNullable,
    BillingCountryNullable, BillingStateNullable, BillingZip, Cardcvv, Cardexp, Cardholder,
    Cardnumber, Cardzip, ConvertToken, CustomerId, CustomerNumberNullable, Device, Email,
    Entrypointfield, Identifierfields, PayorDataRequest, PhoneNumber, RequestTokenStorage,
    RequestTokenStoragePaymentMethod, Shippingaddress, Shippingaddressadditional, Shippingcity,
    Shippingcountry, Shippingstate, Shippingzip, Source, Subdomain, TokenizeAch, TokenizeCard,
    VendorDataRequest,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .token_storage
        .update_method(
            &"32-8877drt00045632-678".to_string(),
            &UpdateMethodRequest {
                body: RequestTokenStorage {
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
                    fallback_auth: Some(true),
                    fallback_auth_amount: None,
                    method_description: None,
                    payment_method: Some(RequestTokenStoragePaymentMethod::TokenizeCard(
                        TokenizeCard {
                            method: "card".to_string(),
                            cardcvv: Some(Cardcvv("123".to_string())),
                            cardexp: Cardexp("02/25".to_string()),
                            card_holder: Cardholder("John Doe".to_string()),
                            cardnumber: Cardnumber("4111111111111111".to_string()),
                            cardzip: Some(Cardzip("12345".to_string())),
                        },
                    )),
                    vendor_data: None,
                    source: None,
                    subdomain: None,
                },
                ach_validation: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**method_id:** `String` — The saved payment method ID.
    
</dd>
</dl>

<dl>
<dd>

**ach_validation:** `Option<AchValidation>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## User
<details><summary><code>client.user.<a href="/src/api/resources/user/client.rs">add_user</a>(request: UserData) -> Result<AddUserResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Use this endpoint to add a new user to an organization.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    AdditionalData, Email, Language, MfaData, MfaMode, NameUser, OrgScope, Orgid, Orgtype,
    PhoneNumber, Timezone, UserData, UsrAccess, UsrStatus,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .user
        .add_user(
            &UserData {
                access: None,
                additional_data: None,
                email: None,
                language: None,
                mfa_data: None,
                name: None,
                phone: None,
                pwd: None,
                scope: None,
                time_zone: None,
                usr_status: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.user.<a href="/src/api/resources/user/client.rs">auth_refresh_user</a>() -> Result<PayabliApiResponseUserMfa, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Use this endpoint to refresh the authentication token for a user within an organization.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.user.auth_refresh_user(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.user.<a href="/src/api/resources/user/client.rs">auth_reset_user</a>(request: UserAuthResetRequest) -> Result<AuthResetUserResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Use this endpoint to initiate a password reset for a user within an organization.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::Email;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .user
        .auth_reset_user(
            &UserAuthResetRequest {
                email: None,
                entry: None,
                entry_type: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**email:** `Option<Email>` — The user's email address.
    
</dd>
</dl>

<dl>
<dd>

**entry:** `Option<String>` — Identifier for entrypoint originating the request (used by front-end apps)
    
</dd>
</dl>

<dl>
<dd>

**entry_type:** `Option<i64>` — Type of entry identifier: 0 - partner, 2 - paypoint. This is used by front-end apps, required if an Entry is indicated.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.user.<a href="/src/api/resources/user/client.rs">auth_user</a>(provider: String, request: UserAuthRequest) -> Result<PayabliApiResponseMfaBasic, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

This endpoint requires an application API token.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::Email;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .user
        .auth_user(
            &"provider".to_string(),
            &UserAuthRequest {
                email: None,
                entry: None,
                entry_type: None,
                psw: None,
                user_id: None,
                user_token_id: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**provider:** `String` — Auth provider. This fields is optional and defaults to null for the built-in provider.
    
</dd>
</dl>

<dl>
<dd>

**email:** `Option<Email>` 
    
</dd>
</dl>

<dl>
<dd>

**entry:** `Option<String>` — Identifier for entry point originating the request (used by front-end apps)
    
</dd>
</dl>

<dl>
<dd>

**entry_type:** `Option<i64>` — Type of entry identifier: 0 - partner, 2 - paypoint. This is used by front-end apps, required if an Entry is indicated.
    
</dd>
</dl>

<dl>
<dd>

**psw:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**user_id:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**user_token_id:** `Option<String>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.user.<a href="/src/api/resources/user/client.rs">change_psw_user</a>(request: UserAuthPswResetRequest) -> Result<ChangePswUserResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Use this endpoint to change the password for a user within an organization.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .user
        .change_psw_user(&UserAuthPswResetRequest { psw: None }, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**psw:** `Option<String>` — New User password
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.user.<a href="/src/api/resources/user/client.rs">delete_user</a>(user_id: String) -> Result<DeleteUserResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Use this endpoint to delete a specific user within an organization.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.user.delete_user(1000000, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**user_id:** `String` — The Payabli-generated `userId` value.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.user.<a href="/src/api/resources/user/client.rs">edit_mfa_user</a>(user_id: String, request: MfaData) -> Result<EditMfaUserResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Use this endpoint to enable or disable multi-factor authentication (MFA) for a user within an organization.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{MfaData, MfaMode};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .user
        .edit_mfa_user(
            1000000,
            &MfaData {
                mfa: None,
                mfa_mode: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**user_id:** `String` — User Identifier
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.user.<a href="/src/api/resources/user/client.rs">edit_user</a>(user_id: String, request: UserData) -> Result<PayabliApiResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Use this endpoint to modify the details of a specific user within an organization.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    AdditionalData, Email, Language, MfaData, MfaMode, NameUser, OrgScope, Orgid, Orgtype,
    PhoneNumber, Timezone, UserData, UsrAccess, UsrStatus,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .user
        .edit_user(
            1000000,
            &UserData {
                access: None,
                additional_data: None,
                email: None,
                language: None,
                mfa_data: None,
                name: None,
                phone: None,
                pwd: None,
                scope: None,
                time_zone: None,
                usr_status: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**user_id:** `String` — User Identifier
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.user.<a href="/src/api/resources/user/client.rs">get_user</a>(user_id: String, entry: Option<Option<String>>, level: Option<Option<i64>>) -> Result<UserQueryRecord, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Use this endpoint to retrieve information about a specific user within an organization.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .user
        .get_user(
            1000000,
            &GetUserQueryRequest {
                entry: Some("478ae1234".to_string()),
                level: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**user_id:** `String` — The Payabli-generated `userId` value.
    
</dd>
</dl>

<dl>
<dd>

**entry:** `Option<String>` — The entrypoint identifier.
    
</dd>
</dl>

<dl>
<dd>

**level:** `Option<i64>` — Entry level: 0 - partner, 2 - paypoint
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.user.<a href="/src/api/resources/user/client.rs">logout_user</a>() -> Result<LogoutUserResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Use this endpoint to log a user out from the system.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.user.logout_user(None).await;
}
```
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.user.<a href="/src/api/resources/user/client.rs">resend_mfa_code</a>(usrname: String, entry: String, entry_type: i64) -> Result<PayabliApiResponseMfaBasic, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Resends the MFA code to the user via the selected MFA mode (email or SMS).
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

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
        .user
        .resend_mfa_code(&"usrname".to_string(), &"Entry".to_string(), 1, None)
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**usrname:** `String` —  
    
</dd>
</dl>

<dl>
<dd>

**entry:** `String` —  
    
</dd>
</dl>

<dl>
<dd>

**entry_type:** `i64` —  
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.user.<a href="/src/api/resources/user/client.rs">validate_mfa_user</a>(request: MfaValidationData) -> Result<PayabliApiResponseUserMfa, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Use this endpoint to validate the multi-factor authentication (MFA) code for a user within an organization.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::MfaValidationCode;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .user
        .validate_mfa_user(
            &MfaValidationData {
                mfa_code: None,
                mfa_validation_code: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**mfa_code:** `Option<String>` 
    
</dd>
</dl>

<dl>
<dd>

**mfa_validation_code:** `Option<MfaValidationCode>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Vendor
<details><summary><code>client.vendor.<a href="/src/api/resources/vendor/client.rs">add_vendor</a>(entry: String, request: VendorData) -> Result<PayabliApiResponseVendors, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Creates a vendor in an entrypoint.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    AdditionalData, AdditionalDataString, AddressAddtlNullable, AddressNullable,
    BankAccountHolderName, BankAccountHolderType, BankName, BillingData, Contacts, ContactsField,
    Email, LocationCode, Mcc, PayeeName, RemitEmail, Remitaddress1, Remitaddress2, Remitcity,
    Remitcountry, Remitstate, Remitzip, RoutingAccount, TypeAccount, VendorData, VendorEin,
    VendorName1, VendorName2, VendorNumber, VendorPaymentMethodString, VendorPhone, Vendorstatus,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .vendor
        .add_vendor(
            &"8cfec329267".to_string(),
            &VendorData {
                vendor_number: Some(VendorNumber("1234".to_string())),
                additional_data: None,
                address_1: Some(AddressNullable("123 Ocean Drive".to_string())),
                address_2: Some(AddressAddtlNullable("Suite 400".to_string())),
                billing_data: Some(BillingData {
                    account_number: Some("123123123".to_string()),
                    bank_account_function: Some(0),
                    bank_account_holder_name: Some(BankAccountHolderName(
                        "Gruzya Adventure Outfitters LLC".to_string(),
                    )),
                    bank_account_holder_type: Some(BankAccountHolderType::Business),
                    bank_name: Some(BankName("Country Bank".to_string())),
                    id: Some(123),
                    routing_account: Some(RoutingAccount("123123123".to_string())),
                    type_account: Some(TypeAccount::Checking),
                }),
                city: Some("Miami".to_string()),
                contacts: Some(ContactsField(Some(vec![Contacts {
                    contact_email: Some(Email("example@email.com".to_string())),
                    contact_name: Some("Herman Martinez".to_string()),
                    contact_phone: Some("3055550000".to_string()),
                    contact_title: Some("Owner".to_string()),
                    additional_data: None,
                }]))),
                country: Some("US".to_string()),
                custom_field_1: None,
                custom_field_2: None,
                customer_vendor_account: Some("A-37622".to_string()),
                ein: Some(VendorEin("12-3456789".to_string())),
                email: Some(Email("example@email.com".to_string())),
                internal_reference_id: Some(123),
                location_code: Some(LocationCode("MIA123".to_string())),
                mcc: Some(Mcc("7777".to_string())),
                name_1: Some(VendorName1("Herman's Coatings and Masonry".to_string())),
                name_2: Some(VendorName2("<string>".to_string())),
                payee_name_1: Some(PayeeName("<string>".to_string())),
                payee_name_2: Some(PayeeName("<string>".to_string())),
                payment_method: Some(VendorPaymentMethodString("managed".to_string())),
                phone: Some(VendorPhone(Some("5555555555".to_string()))),
                remit_address_1: Some(Remitaddress1("123 Walnut Street".to_string())),
                remit_address_2: Some(Remitaddress2("Suite 900".to_string())),
                remit_city: Some(Remitcity("Miami".to_string())),
                remit_country: Some(Remitcountry("US".to_string())),
                remit_email: None,
                remit_state: Some(Remitstate("FL".to_string())),
                remit_zip: Some(Remitzip("31113".to_string())),
                state: Some("FL".to_string()),
                vendor_status: Some(Vendorstatus(1)),
                zip: Some("33139".to_string()),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `String` — Entrypoint identifier.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.vendor.<a href="/src/api/resources/vendor/client.rs">delete_vendor</a>(id_vendor: i64) -> Result<PayabliApiResponseVendors, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Delete a vendor. 
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.vendor.delete_vendor(1, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id_vendor:** `i64` — Vendor ID.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.vendor.<a href="/src/api/resources/vendor/client.rs">edit_vendor</a>(id_vendor: i64, request: VendorData) -> Result<PayabliApiResponseVendors, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Updates a vendor's information. Send only the fields you need to update.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{
    AdditionalData, AdditionalDataString, AddressAddtlNullable, AddressNullable,
    BankAccountHolderName, BankAccountHolderType, BankName, BillingData, Contacts, ContactsField,
    Email, LocationCode, Mcc, PayeeName, RemitEmail, Remitaddress1, Remitaddress2, Remitcity,
    Remitcountry, Remitstate, Remitzip, RoutingAccount, TypeAccount, VendorData, VendorEin,
    VendorName1, VendorName2, VendorNumber, VendorPaymentMethodString, VendorPhone, Vendorstatus,
};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .vendor
        .edit_vendor(
            1,
            &VendorData {
                vendor_number: None,
                additional_data: None,
                address_1: None,
                address_2: None,
                billing_data: None,
                city: None,
                contacts: None,
                country: None,
                custom_field_1: None,
                custom_field_2: None,
                customer_vendor_account: None,
                ein: None,
                email: None,
                internal_reference_id: None,
                location_code: None,
                mcc: None,
                name_1: Some(VendorName1("Theodore's Janitorial".to_string())),
                name_2: None,
                payee_name_1: None,
                payee_name_2: None,
                payment_method: None,
                phone: None,
                remit_address_1: None,
                remit_address_2: None,
                remit_city: None,
                remit_country: None,
                remit_email: None,
                remit_state: None,
                remit_zip: None,
                state: None,
                vendor_status: None,
                zip: None,
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id_vendor:** `i64` — Vendor ID.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.vendor.<a href="/src/api/resources/vendor/client.rs">get_vendor</a>(id_vendor: i64) -> Result<VendorQueryRecord, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Retrieves a vendor's details.
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client.vendor.get_vendor(1, None).await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**id_vendor:** `i64` — Vendor ID.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

## Wallet
<details><summary><code>client.wallet.<a href="/src/api/resources/wallet/client.rs">configure_apple_pay_organization</a>(request: ConfigureOrganizationRequestApplePay) -> Result<ConfigureApplePayOrganizationApiResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Configure and activate Apple Pay for a Payabli organization
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{Cascade, IsEnabled, OrganizationId};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .wallet
        .configure_apple_pay_organization(
            &ConfigureOrganizationRequestApplePay {
                cascade: Some(Cascade(true)),
                is_enabled: Some(IsEnabled(true)),
                org_id: Some(OrganizationId(901)),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**cascade:** `Option<Cascade>` 
    
</dd>
</dl>

<dl>
<dd>

**is_enabled:** `Option<IsEnabled>` 
    
</dd>
</dl>

<dl>
<dd>

**org_id:** `Option<OrganizationId>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.wallet.<a href="/src/api/resources/wallet/client.rs">configure_apple_pay_paypoint</a>(request: ConfigurePaypointRequestApplePay) -> Result<ConfigureApplePaypointApiResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Configure and activate Apple Pay for a Payabli paypoint
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{Entry, IsEnabled};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .wallet
        .configure_apple_pay_paypoint(
            &ConfigurePaypointRequestApplePay {
                entry: Some(Entry("8cfec329267".to_string())),
                is_enabled: Some(IsEnabled(true)),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `Option<Entry>` 
    
</dd>
</dl>

<dl>
<dd>

**is_enabled:** `Option<IsEnabled>` — When `true`, Apple Pay is enabled.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.wallet.<a href="/src/api/resources/wallet/client.rs">configure_google_pay_organization</a>(request: ConfigureOrganizationRequestGooglePay) -> Result<ConfigureApplePayOrganizationApiResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Configure and activate Google Pay for a Payabli organization
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{Cascade, IsEnabled, OrganizationId};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .wallet
        .configure_google_pay_organization(
            &ConfigureOrganizationRequestGooglePay {
                cascade: Some(Cascade(true)),
                is_enabled: Some(IsEnabled(true)),
                org_id: Some(OrganizationId(901)),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**cascade:** `Option<Cascade>` 
    
</dd>
</dl>

<dl>
<dd>

**is_enabled:** `Option<IsEnabled>` 
    
</dd>
</dl>

<dl>
<dd>

**org_id:** `Option<OrganizationId>` 
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>

<details><summary><code>client.wallet.<a href="/src/api/resources/wallet/client.rs">configure_google_pay_paypoint</a>(request: ConfigurePaypointRequestGooglePay) -> Result<ConfigureGooglePaypointApiResponse, ApiError></code></summary>
<dl>
<dd>

#### 📝 Description

<dl>
<dd>

<dl>
<dd>

Configure and activate Google Pay for a Payabli paypoint
</dd>
</dl>
</dd>
</dl>

#### 🔌 Usage

<dl>
<dd>

<dl>
<dd>

```rust
use payabli_api::prelude::*;
use payabli_api::{Entry, IsEnabled};

#[tokio::main]
async fn main() {
    let config = ClientConfig {
        api_key: Some("<value>".to_string()),
        ..Default::default()
    };
    let client = ApiClient::new(config).expect("Failed to build client");
    client
        .wallet
        .configure_google_pay_paypoint(
            &ConfigurePaypointRequestGooglePay {
                entry: Some(Entry("8cfec329267".to_string())),
                is_enabled: Some(IsEnabled(true)),
            },
            None,
        )
        .await;
}
```
</dd>
</dl>
</dd>
</dl>

#### ⚙️ Parameters

<dl>
<dd>

<dl>
<dd>

**entry:** `Option<Entry>` 
    
</dd>
</dl>

<dl>
<dd>

**is_enabled:** `Option<IsEnabled>` — When `true`, Google Pay is enabled.
    
</dd>
</dl>
</dd>
</dl>


</dd>
</dl>
</details>
