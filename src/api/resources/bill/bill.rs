use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct BillClient {
    pub http_client: HttpClient,
}

impl BillClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Creates a bill in an entrypoint.
    ///
    /// # Arguments
    ///
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use payabli_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .bill
    ///         .add_bill(
    ///             &"8cfec329267".to_string(),
    ///             &BillOutData {
    ///                 accounting_field_1: Some(AccountingField("MyInternalId".to_string())),
    ///                 attachments: Some(Attachments(vec![FileContent {
    ///                     filename: Some("my-doc.pdf".to_string()),
    ///                     ftype: Some(FileContentFtype::Pdf),
    ///                     furl: Some("https://mysite.com/my-doc.pdf".to_string()),
    ///                     ..Default::default()
    ///                 }])),
    ///                 bill_date: Some(NaiveDate::parse_from_str("2024-07-01", "%Y-%m-%d").unwrap()),
    ///                 bill_items: Some(Billitems(vec![BillItem {
    ///                     item_categories: Some(vec!["deposits".to_string()]),
    ///                     item_commodity_code: Some(ItemCommodityCode("010".to_string())),
    ///                     item_cost: Some(5.0),
    ///                     item_description: Some(ItemDescription("Deposit for materials".to_string())),
    ///                     item_mode: Some(0),
    ///                     item_product_code: Some(ItemProductCode("M-DEPOSIT".to_string())),
    ///                     item_product_name: Some(ItemProductName("Materials deposit".to_string())),
    ///                     item_qty: Some(1),
    ///                     item_tax_amount: Some(7.0),
    ///                     item_tax_rate: Some(0.075),
    ///                     item_total_amount: Some(123.0),
    ///                     item_unit_of_measure: Some(ItemUnitofMeasure("SqFt".to_string())),
    ///                     ..Default::default()
    ///                 }])),
    ///                 bill_number: Some("ABC-123".to_string()),
    ///                 comments: Some(Comments("Deposit for materials".to_string())),
    ///                 due_date: Some(NaiveDate::parse_from_str("2024-07-01", "%Y-%m-%d").unwrap()),
    ///                 end_date: Some(NaiveDate::parse_from_str("2024-07-01", "%Y-%m-%d").unwrap()),
    ///                 frequency: Some(Frequency::Monthly),
    ///                 mode: Some(0),
    ///                 net_amount: Some(3762.87),
    ///                 status: Some(Billstatus(1)),
    ///                 terms: Some(Terms::Net30),
    ///                 vendor: Some(BillOutDataVendor {
    ///                     vendor_number: Some(VendorNumber("VEN-123".to_string())),
    ///                     ..Default::default()
    ///                 }),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn add_bill(
        &self,
        entry: &str,
        request: &BillOutData,
        options: Option<RequestOptions>,
    ) -> Result<BillResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("Bill/single/{}", entry),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Retrieves a bill by ID from an entrypoint.
    ///
    /// # Arguments
    ///
    /// * `id_bill` - Payabli ID for the bill. Get this ID by querying `/api/Query/bills/` for the entrypoint or the organization.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use payabli_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client.bill.get_bill(285, None).await;
    /// }
    /// ```
    pub async fn get_bill(
        &self,
        id_bill: i64,
        options: Option<RequestOptions>,
    ) -> Result<GetBillResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Bill/{}", id_bill),
                None,
                None,
                options,
            )
            .await
    }

    /// Updates a bill by ID.
    ///
    /// # Arguments
    ///
    /// * `id_bill` - Payabli ID for the bill. Get this ID by querying `/api/Query/bills/` for the entrypoint or the organization.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use payabli_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .bill
    ///         .edit_bill(
    ///             285,
    ///             &BillOutData {
    ///                 bill_date: Some(NaiveDate::parse_from_str("2025-07-01", "%Y-%m-%d").unwrap()),
    ///                 net_amount: Some(3762.87),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn edit_bill(
        &self,
        id_bill: i64,
        request: &BillOutData,
        options: Option<RequestOptions>,
    ) -> Result<EditBillResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("Bill/{}", id_bill),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Deletes a bill by ID.
    ///
    /// # Arguments
    ///
    /// * `id_bill` - Payabli ID for the bill. Get this ID by querying `/api/Query/bills/` for the entrypoint or the organization.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use payabli_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client.bill.delete_bill(285, None).await;
    /// }
    /// ```
    pub async fn delete_bill(
        &self,
        id_bill: i64,
        options: Option<RequestOptions>,
    ) -> Result<BillResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("Bill/{}", id_bill),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieves a file attached to a bill, either as a binary file or as a Base64-encoded string.
    ///
    /// # Arguments
    ///
    /// * `id_bill` - Payabli ID for the bill. Get this ID by querying `/api/Query/bills/` for the entrypoint or the organization.
    /// * `filename` - The filename in Payabli. Get this from the `zipName` field
    /// in the `DocumentsRef.filelist` array returned by
    /// `/api/Bill/{idBill}`. Example: `0_Bill.pdf`.
    /// * `return_object` - When `true`, the request returns the file content as a Base64-encoded string.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use payabli_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .bill
    ///         .get_attached_from_bill(
    ///             285,
    ///             &"0_Bill.pdf".to_string(),
    ///             &GetAttachedFromBillQueryRequest {
    ///                 return_object: Some(true),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn get_attached_from_bill(
        &self,
        id_bill: i64,
        filename: &str,
        request: &GetAttachedFromBillQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<FileContent, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Bill/attachedFileFromBill/{}/{}", id_bill, filename),
                None,
                QueryBuilder::new()
                    .bool("returnObject", request.return_object.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Delete a file attached to a bill.
    ///
    /// # Arguments
    ///
    /// * `id_bill` - Payabli ID for the bill. Get this ID by querying `/api/Query/bills/` for the entrypoint or the organization.
    /// * `filename` - The filename in Payabli. Get this from the `zipName` field
    /// in the `DocumentsRef.filelist` array returned by
    /// `/api/Bill/{idBill}`. Example: `0_Bill.pdf`.
    /// * `return_object` - When `true`, the response includes the full bill object.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use payabli_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .bill
    ///         .delete_attached_from_bill(
    ///             285,
    ///             &"0_Bill.pdf".to_string(),
    ///             &DeleteAttachedFromBillQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn delete_attached_from_bill(
        &self,
        id_bill: i64,
        filename: &str,
        request: &DeleteAttachedFromBillQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<BillResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("Bill/attachedFileFromBill/{}/{}", id_bill, filename),
                None,
                QueryBuilder::new()
                    .bool("returnObject", request.return_object.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Send a bill to a user or list of users to approve.
    ///
    /// # Arguments
    ///
    /// * `id_bill` - Payabli ID for the bill. Get this ID by querying `/api/Query/bills/` for the entrypoint or the organization.
    /// * `autocreate_user` - Automatically create the target user for approval if they don't exist.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use payabli_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .bill
    ///         .send_to_approval_bill(
    ///             285,
    ///             &SendToApprovalBillRequest {
    ///                 body: vec!["approver@example.com".to_string()],
    ///                 autocreate_user: None,
    ///             },
    ///             Some(
    ///                 RequestOptions::new()
    ///                     .additional_header("idempotencyKey", "6B29FC40-CA47-1067-B31D-00DD010662DA"),
    ///             ),
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn send_to_approval_bill(
        &self,
        id_bill: i64,
        request: &SendToApprovalBillRequest,
        options: Option<RequestOptions>,
    ) -> Result<BillResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::POST,
                &format!("Bill/approval/{}", id_bill),
                Some(serde_json::to_value(&request.body).map_err(ApiError::Serialization)?),
                QueryBuilder::new()
                    .bool("autocreateUser", request.autocreate_user.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Modify the list of users the bill is sent to for approval.
    ///
    /// # Arguments
    ///
    /// * `id_bill` - Payabli ID for the bill. Get this ID by querying `/api/Query/bills/` for the entrypoint or the organization.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use payabli_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .bill
    ///         .modify_approval_bill(
    ///             285,
    ///             &vec![
    ///                 "approver1@example.com".to_string(),
    ///                 "approver2@example.com".to_string(),
    ///             ],
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn modify_approval_bill(
        &self,
        id_bill: i64,
        request: &Vec<String>,
        options: Option<RequestOptions>,
    ) -> Result<ModifyApprovalBillResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("Bill/approval/{}", id_bill),
                Some(serde_json::to_value(request).map_err(ApiError::Serialization)?),
                None,
                options,
            )
            .await
    }

    /// Approve or disapprove a bill by ID.
    ///
    /// # Arguments
    ///
    /// * `id_bill` - Payabli ID for the bill. Get this ID by querying `/api/Query/bills/` for the entrypoint or the organization.
    /// * `approved` - String representing the approved status. Accepted values: 'true' or 'false'.
    /// * `email` - Email or username of user modifying approval status.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use payabli_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .bill
    ///         .set_approved_bill(
    ///             285,
    ///             &"true".to_string(),
    ///             &SetApprovedBillQueryRequest {
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn set_approved_bill(
        &self,
        id_bill: i64,
        approved: &str,
        request: &SetApprovedBillQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SetApprovedBillResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Bill/approval/{}/{}", id_bill, approved),
                None,
                QueryBuilder::new()
                    .string("email", request.email.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve a list of bills for an entrypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
    ///
    /// # Arguments
    ///
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
    /// * `export_format` - Export format for file downloads. When specified, returns data as a file instead of JSON.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response isn't filtered. Instead, copy the request, remove `parameters=` and run the request in a different client, for example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// **Accepted field names:**
    /// - `frequency` (`in`, `nin`, `ne`, `eq`)
    /// - `method` (`in`, `nin`, `eq`, `ne`)
    /// - `event` (`in`, `nin`, `eq`, `ne`)
    /// - `target` (`ct`, `nct`, `eq`, `ne`)
    /// - `status` (`eq`, `ne`)
    /// - `approvalUserId` (`eq`, `ne`)
    /// - `parentOrgId` (`ne`, `eq`, `nin`, `in`)
    /// - `approvalUserEmail` (`eq`, `ne`)
    /// - `scheduleId` (`ne`, `eq`)
    ///
    /// Accepted comparison operators - enclosed between parentheses:
    /// - `eq` or empty => equal
    /// - `gt` => greater than
    /// - `ge` => greater or equal
    /// - `lt` => less than
    /// - `le` => less or equal
    /// - `ne` => not equal
    /// - `ct` => contains
    /// - `nct` => not contains
    /// - `in` => inside array
    /// - `nin` => not inside array
    ///
    /// Accepted parameters:
    /// - `limitRecord` : max number of records for query (default="20", "0" or negative value for all)
    /// - `fromRecord` : initial record in query
    /// Example: `totalAmount(gt)=20` returns all records with a `totalAmount` that's greater than 20.00
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use payabli_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .bill
    ///         .list_bills(
    ///             &"8cfec329267".to_string(),
    ///             &ListBillsQueryRequest {
    ///                 from_record: Some(251),
    ///                 limit_record: Some(0),
    ///                 sort_by: Some("desc(field_name)".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_bills(
        &self,
        entry: &str,
        request: &ListBillsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<BillQueryResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/bills/{}", entry),
                None,
                QueryBuilder::new()
                    .serialize("exportFormat", request.export_format.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .string("sortBy", request.sort_by.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve a list of bills for an organization. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
    ///
    /// # Arguments
    ///
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `export_format` - Export format for file downloads. When specified, returns data as a file instead of JSON.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response isn't filtered. Instead, copy the request, remove `parameters=` and run the request in a different client, for example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// **Accepted field names:**
    /// - `frequency` (in, nin, ne, eq)
    /// - `method` (in, nin, eq, ne)
    /// - `event` (in, nin, eq, ne)
    /// - `target` (ct, nct, eq, ne)
    /// - `status` (eq, ne)
    /// - `parentOrgId` (ne, eq, nin, in)
    /// - `approvalUserId` (eq, ne)
    /// - `approvalUserEmail` (eq, ne)
    ///
    /// Accepted comparison operators - enclosed between parentheses:
    /// - eq or empty => equal
    /// - gt => greater than
    /// - ge => greater or equal
    /// - lt => less than
    /// - le => less or equal
    /// - ne => not equal
    /// - ct => contains
    /// - nct => not contains
    /// - in => inside array
    /// - nin => not inside array
    ///
    /// Accepted parameters:
    /// - limitRecord : max number of records for query (default="20", "0" or negative value for all)
    /// - fromRecord : initial record in query
    ///
    /// Example: totalAmount(gt)=20 return all records with totalAmount greater than 20.00
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use payabli_api::prelude::*;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let config = ClientConfig {
    ///         ..Default::default()
    ///     };
    ///     let client = ApiClient::new(config).expect("Failed to build client");
    ///     client
    ///         .bill
    ///         .list_bills_org(
    ///             123,
    ///             &ListBillsOrgQueryRequest {
    ///                 from_record: Some(251),
    ///                 limit_record: Some(0),
    ///                 sort_by: Some("desc(field_name)".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn list_bills_org(
        &self,
        org_id: i64,
        request: &ListBillsOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<BillQueryResponse, ApiError> {
        let endpoint_auth_headers = self
            .http_client
            .resolve_endpoint_auth_headers(
                &options,
                &[&["BearerAuth"] as &[&str], &["APIKeyAuth"] as &[&str]],
            )
            .await?;
        let options = {
            let mut o = options.unwrap_or_default();
            for (header_key, header_value) in endpoint_auth_headers {
                o.additional_headers.insert(header_key, header_value);
            }
            Some(o)
        };
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/bills/org/{}", org_id),
                None,
                QueryBuilder::new()
                    .serialize("exportFormat", request.export_format.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .string("sortBy", request.sort_by.clone())
                    .build(),
                options,
            )
            .await
    }
}
