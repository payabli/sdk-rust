use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct ExportClient {
    pub http_client: HttpClient,
}

impl ExportClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [List all apps for org](/developers/api-reference/boarding/get-list-of-applications-for-an-organization) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Export a list of boarding applications for an organization. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `format` - Format for the export, either XLSX or CSV.
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    /// - `createdAt` (gt, ge, lt, le, eq, ne)
    /// - `startDate` (gt, ge, lt, le, eq, ne)
    /// - `dbaname`  (ct, nct)
    /// - `legalname`  (ct, nct)
    /// - `ein`  (ct, nct)
    /// - `address`  (ct, nct)
    /// - `city`  (ct, nct)
    /// - `state`  (ct, nct)
    /// - `phone`  (ct, nct)
    /// - `mcc`  (ct, nct)
    /// - `owntype`  (ct, nct)
    /// - `ownerName`  (ct, nct)
    /// - `contactName`  (ct, nct)
    /// - `status`  (eq, ne)
    /// - `orgParentname`  (ct, nct)
    ///
    /// List of comparison accepted - enclosed between parentheses:
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
    /// List of parameters accepted:
    /// - `limitRecord` : max number of records for query (default="20", "0" or negative value for all)
    /// - `fromRecord` : initial record in query
    ///
    /// Example: `dbaname(ct)=hoa` returns all records with a `dbaname` containing "hoa"
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
    ///         .export
    ///         .export_applications(
    ///             &ExportFormat1::Csv,
    ///             123,
    ///             &ExportApplicationsQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_applications(
        &self,
        format: &ExportFormat1,
        org_id: i64,
        request: &ExportApplicationsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!("Export/boarding/{}/{}", format, org_id),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [List batch details](/developers/api-reference/query/get-list-of-batchdetails-for-an-entrypoint) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Export batch details for a paypoint. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `format` - Format for the export, either XLSX or CSV.
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// **List of field names accepted:**
    ///
    /// - `settlementDate` (gt, ge, lt, le, eq, ne)
    /// - `depositDate` (gt, ge, lt, le, eq, ne)
    /// - `transId`  (ne, eq, ct, nct)
    /// - `gatewayTransId`  (ne, eq, ct, nct)
    /// - `method`   (in, nin, eq, ne)
    /// - `settledAmount`  (gt, ge, lt, le, eq, ne)
    /// - `operation`    (in, nin, eq, ne)
    /// - `source`   (in, nin, eq, ne)
    /// - `batchNumber`  (ct, nct, eq, ne)
    /// - `payaccountLastfour`   (nct, ct)
    /// - `payaccountType`   (ne, eq, in, nin)
    /// - `customerFirstname`   (ct, nct, eq, ne)
    /// - `customerLastname`    (ct, nct, eq, ne)
    /// - `customerName`   (ct, nct)
    /// - `customerId`  (eq, ne)
    /// - `customerNumber`  (ct, nct, eq, ne)
    /// - `customerCompanyname`    (ct, nct, eq, ne)
    /// - `customerAddress` (ct, nct, eq, ne)
    /// - `customerCity`    (ct, nct, eq, ne)
    /// - `customerZip` (ct, nct, eq, ne)
    /// - `customerState` (ct, nct, eq, ne)
    /// - `customerCountry` (ct, nct, eq, ne)
    /// - `customerPhone` (ct, nct, eq, ne)
    /// - `customerEmail` (ct, nct, eq, ne)
    /// - `customerShippingAddress` (ct, nct, eq, ne)
    /// - `customerShippingCity`    (ct, nct, eq, ne)
    /// - `customerShippingZip` (ct, nct, eq, ne)
    /// - `customerShippingState` (ct, nct, eq, ne)
    /// - `customerShippingCountry` (ct, nct, eq, ne)
    /// - `orgId`  (eq) *mandatory when entry=org*
    /// - `isHold` (eq, ne)
    /// - `paypointId`  (ne, eq)
    /// - `paypointLegal`  (ne, eq, ct, nct)
    /// - `paypointDba`  (ne, eq, ct, nct)
    /// - `orgName`  (ne, eq, ct, nct)
    /// - `batchId` (ct, nct, eq, neq)
    /// - `additional-xxx`  (ne, eq, ct, nct) where xxx is the additional field name
    ///
    /// List of parameters accepted:
    /// - limitRecord: max number of records for query (default="20", "0" or negative value for all)
    /// - fromRecord: initial record in query
    ///
    /// Example: `amount(gt)=20` return all records with amount greater than 20.00
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
    ///         .export
    ///         .export_batch_details(
    ///             &ExportFormat1::Csv,
    ///             &"8cfec329267".to_string(),
    ///             &ExportBatchDetailsQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_batch_details(
        &self,
        format: &ExportFormat1,
        entry: &str,
        request: &ExportBatchDetailsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!("Export/batchDetails/{}/{}", format, entry),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [List batch details for org](/developers/api-reference/query/get-list-of-batchdetails-for-an-organization) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Export batch details for an organization. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `format` - Format for the export, either XLSX or CSV.
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// **List of field names accepted:**
    ///
    /// - `settlementDate` (gt, ge, lt, le, eq, ne)
    /// - `depositDate` (gt, ge, lt, le, eq, ne)
    /// - `transId`  (ne, eq, ct, nct)
    /// - `gatewayTransId`  (ne, eq, ct, nct)
    /// - `method`   (in, nin, eq, ne)
    /// - `settledAmount`  (gt, ge, lt, le, eq, ne)
    /// - `operation`    (in, nin, eq, ne)
    /// - `source`   (in, nin, eq, ne)
    /// - `batchNumber`  (ct, nct, eq, ne)
    /// - `payaccountLastfour`   (nct, ct)
    /// - `payaccountType`   (ne, eq, in, nin)
    /// - `customerFirstname`   (ct, nct, eq, ne)
    /// - `customerLastname`    (ct, nct, eq, ne)
    /// - `customerName`   (ct, nct)
    /// - `customerId`  (eq, ne)
    /// - `customerNumber`  (ct, nct, eq, ne)
    /// - `customerCompanyname`    (ct, nct, eq, ne)
    /// - `customerAddress` (ct, nct, eq, ne)
    /// - `customerCity`    (ct, nct, eq, ne)
    /// - `customerZip` (ct, nct, eq, ne)
    /// - `customerState` (ct, nct, eq, ne)
    /// - `customerCountry` (ct, nct, eq, ne)
    /// - `customerPhone` (ct, nct, eq, ne)
    /// - `customerEmail` (ct, nct, eq, ne)
    /// - `customerShippingAddress` (ct, nct, eq, ne)
    /// - `customerShippingCity`    (ct, nct, eq, ne)
    /// - `customerShippingZip` (ct, nct, eq, ne)
    /// - `customerShippingState` (ct, nct, eq, ne)
    /// - `customerShippingCountry` (ct, nct, eq, ne)
    /// - `orgId`  (eq) *mandatory when entry=org*
    /// - `isHold` (eq, ne)
    /// - `paypointId`  (ne, eq)
    /// - `paypointLegal`  (ne, eq, ct, nct)
    /// - `paypointDba`  (ne, eq, ct, nct)
    /// - `orgName`  (ne, eq, ct, nct)
    /// - `batchId` (ct, nct, eq, neq)
    /// - `additional-xxx`  (ne, eq, ct, nct) where xxx is the additional field name
    ///
    /// List of parameters accepted:
    /// - limitRecord: max number of records for query (default="20", "0" or negative value for all)
    /// - fromRecord: initial record in query
    ///
    /// Example: `amount(gt)=20` return all records with amount greater than 20.00
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
    ///         .export
    ///         .export_batch_details_org(
    ///             &ExportFormat1::Csv,
    ///             123,
    ///             &ExportBatchDetailsOrgQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_batch_details_org(
        &self,
        format: &ExportFormat1,
        org_id: i64,
        request: &ExportBatchDetailsOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!("Export/batchDetails/{}/org/{}", format, org_id),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [List batches for paypoint](/developers/api-reference/query/get-list-of-batches-for-an-entrypoint) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Export a list of batches for an entrypoint. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `format` - Format for the export, either XLSX or CSV.
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    /// - `batchDate` (gt, ge, lt, le, eq, ne)
    /// - `batchNumber` (ne, eq)
    /// - `connectorName` (ne, eq, ct, nct)
    /// - `method` (in, nin, eq, ne)
    /// - `batchAmount` (gt, ge, lt, le, eq, ne)
    /// - `feeBatchAmount` (gt, ge, lt, le, eq, ne)
    /// - `netBatchAmount` (gt, ge, lt, le, eq, ne)
    /// - `releaseAmount` (gt, ge, lt, le, eq, ne)
    /// - `heldAmount` (gt, ge, lt, le, eq, ne)
    /// - `status` (in, nin, eq, ne)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    /// - `paypointId` (ne, eq)
    /// - `externalPaypointID` (ct, nct, eq, ne)
    /// - `expectedDepositDate` (gt, ge, lt, le, eq, ne)
    /// - `batchRecords` (gt, ge, lt, le, eq, ne)
    /// - `transferId` (ne, eq)
    /// - `transferDate` (gt, ge, lt, le, eq, ne)
    /// - `grossAmount` (gt, ge, lt, le, eq, ne)
    /// - `chargeBackAmount` (gt, ge, lt, le, eq, ne)
    /// - `returnedAmount` (gt, ge, lt, le, eq, ne)
    /// - `billingFeeAmount` (gt, ge, lt, le, eq, ne)
    /// - `thirdPartyPaidAmount` (gt, ge, lt, le, eq, ne)
    /// - `netFundedAmount` (gt, ge, lt, le, eq, ne)
    /// - `adjustmentAmount` (gt, ge, lt, le, eq, ne)
    /// - `processor` (ne, eq, ct, nct)
    /// - `transferStatus` (ne, eq, in, nin)
    ///
    /// List of parameters accepted:
    /// - limitRecord: max number of records for query (default="20", "0" or negative value for all)
    /// - fromRecord: initial record in query
    ///
    /// Example: `batchAmount(gt)=20` returns all records with a `batchAmount` greater than 20.00
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
    ///         .export
    ///         .export_batches(
    ///             &ExportFormat1::Csv,
    ///             &"8cfec329267".to_string(),
    ///             &ExportBatchesQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_batches(
        &self,
        format: &ExportFormat1,
        entry: &str,
        request: &ExportBatchesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!("Export/batches/{}/{}", format, entry),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [List batches for org](/developers/api-reference/query/get-list-of-batches-for-an-organization) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Export a list of batches for an organization. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `format` - Format for the export, either XLSX or CSV.
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    /// - `batchDate` (gt, ge, lt, le, eq, ne)
    /// - `batchNumber` (ne, eq)
    /// - `connectorName` (ne, eq, ct, nct)
    /// - `method` (in, nin, eq, ne)
    /// - `batchAmount` (gt, ge, lt, le, eq, ne)
    /// - `feeBatchAmount` (gt, ge, lt, le, eq, ne)
    /// - `netBatchAmount` (gt, ge, lt, le, eq, ne)
    /// - `releaseAmount` (gt, ge, lt, le, eq, ne)
    /// - `heldAmount` (gt, ge, lt, le, eq, ne)
    /// - `status` (in, nin, eq, ne)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    /// - `paypointId` (ne, eq)
    /// - `externalPaypointID` (ct, nct, eq, ne)
    /// - `expectedDepositDate` (gt, ge, lt, le, eq, ne)
    /// - `batchRecords` (gt, ge, lt, le, eq, ne)
    /// - `transferId` (ne, eq)
    /// - `transferDate` (gt, ge, lt, le, eq, ne)
    /// - `grossAmount` (gt, ge, lt, le, eq, ne)
    /// - `chargeBackAmount` (gt, ge, lt, le, eq, ne)
    /// - `returnedAmount` (gt, ge, lt, le, eq, ne)
    /// - `billingFeeAmount` (gt, ge, lt, le, eq, ne)
    /// - `thirdPartyPaidAmount` (gt, ge, lt, le, eq, ne)
    /// - `netFundedAmount` (gt, ge, lt, le, eq, ne)
    /// - `adjustmentAmount` (gt, ge, lt, le, eq, ne)
    /// - `processor` (ne, eq, ct, nct)
    /// - `transferStatus` (ne, eq, in, nin)
    ///
    /// List of parameters accepted:
    /// - `limitRecord`: max number of records for query (default="20", "0" or negative value for all)
    /// - `fromRecord`: initial record in query
    /// Example: `batchAmount(gt)=20` returns all records with a `batchAmount` greater than 20.00
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
    ///         .export
    ///         .export_batches_org(
    ///             &ExportFormat1::Csv,
    ///             123,
    ///             &ExportBatchesOrgQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_batches_org(
        &self,
        format: &ExportFormat1,
        org_id: i64,
        request: &ExportBatchesOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!("Export/batches/{}/org/{}", format, org_id),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [List payout batches for paypoint](/developers/api-reference/query/get-list-of-moneyout-batches-for-an-entrypoint) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Export a list of money out batches for a paypoint. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `format` - Format for the export, either XLSX or CSV.
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    /// - `batchDate` (gt, ge, lt, le, eq, ne)
    /// - `batchNumber` (ne, eq)
    /// - `batchAmount` (gt, ge, lt, le, eq, ne)
    /// - `status` (in, nin, eq, ne)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct, nin, in)
    /// - `paypointId` (ne, eq)
    /// - `externalPaypointID` (ct, nct, eq, ne)
    /// List of parameters accepted:
    /// - limitRecord: max number of records for query (default="20", "0" or negative value for all)
    /// - fromRecord: initial record in query
    ///
    /// Example: `batchAmount(gt)=20` returns all records with a `batchAmount` greater than 20.00"
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
    ///         .export
    ///         .export_batches_out(
    ///             &ExportFormat1::Csv,
    ///             &"8cfec329267".to_string(),
    ///             &ExportBatchesOutQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_batches_out(
        &self,
        format: &ExportFormat1,
        entry: &str,
        request: &ExportBatchesOutQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!("Export/batchesOut/{}/{}", format, entry),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [List payout batches for org](/developers/api-reference/query/get-list-of-moneyout-batches-for-an-org) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Export a list of money out batches for an organization. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `format` - Format for the export, either XLSX or CSV.
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    /// - `batchDate` (gt, ge, lt, le, eq, ne)
    /// - `batchNumber` (ne, eq)
    /// - `batchAmount` (gt, ge, lt, le, eq, ne)
    /// - `status` (in, nin, eq, ne)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct, nin, in)
    /// - `paypointId` (ne, eq)
    /// - `externalPaypointID` (ct, nct, eq, ne)
    /// List of parameters accepted:
    /// - limitRecord: max number of records for query (default="20", "0" or negative value for all)
    /// - fromRecord: initial record in query
    ///
    /// Example: `batchAmount(gt)=20` returns all records with a `batchAmount` greater than 20.00"
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
    ///         .export
    ///         .export_batches_out_org(
    ///             &ExportFormat1::Csv,
    ///             123,
    ///             &ExportBatchesOutOrgQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_batches_out_org(
        &self,
        format: &ExportFormat1,
        org_id: i64,
        request: &ExportBatchesOutOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!("Export/batchesOut/{}/org/{}", format, org_id),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [List bills by paypoint](/developers/api-reference/bill/get-list-of-bills-for-entrypoint) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Export a list of bills for an entrypoint. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `format` - Format for the export, either XLSX or CSV.
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    /// - `status` (in, nin, eq, ne)
    /// - `billNumber` (ct, nct, eq, ne)
    /// - `billDate` (gt, ge, lt, le, eq, ne)
    /// - `billDueDate` (gt, ge, lt, le, eq, ne)
    /// - `vendorNumber` (ct, nct, eq, ne)
    /// - `vendorName` (ct, nct, eq, ne)
    /// - `ein` (ct, nct, eq, ne)
    /// - `paymentMethod` (ct, nct, eq, ne)
    /// - `paymentId` (ct, nct, eq, ne)
    /// - `paymentgroup` (ct, nct, eq, ne)
    /// - `totalAmount` (gt, ge, lt, le, eq, ne)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    ///
    /// List of comparison accepted - enclosed between parentheses:
    /// - eq or empty => equal
    /// - gt => greater than
    /// - ge => greater or equal
    /// - lt => less than
    /// - le => less or equal
    /// - ne => not equal
    /// - ct => contains
    /// - nct => not contains
    /// - in => inside array separated by "|"
    /// - nin => not inside array separated by "|"
    ///
    /// List of parameters accepted:
    /// - limitRecord : max number of records for query (default="20", "0" or negative value for all)
    /// - fromRecord : initial record in query
    ///
    /// Example: totalAmount(gt)=20  return all records with totalAmount greater than 20.00
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
    ///         .export
    ///         .export_bills(
    ///             &ExportFormat1::Csv,
    ///             &"8cfec329267".to_string(),
    ///             &ExportBillsQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_bills(
        &self,
        format: &ExportFormat1,
        entry: &str,
        request: &ExportBillsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!("Export/bills/{}/{}", format, entry),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [List bills by organization](/developers/api-reference/bill/get-list-of-bills-for-organization) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Export a list of bills for an organization. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `format` - Format for the export, either XLSX or CSV.
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    /// - `status` (in, nin, eq, ne)
    /// - `billNumber` (ct, nct, eq, ne)
    /// - `billDate` (gt, ge, lt, le, eq, ne)
    /// - `billDueDate` (gt, ge, lt, le, eq, ne)
    /// - `vendorNumber` (ct, nct, eq, ne)
    /// - `vendorName` (ct, nct, eq, ne)
    /// - `ein` (ct, nct, eq, ne)
    /// - `paymentMethod` (ct, nct, eq, ne)
    /// - `paymentId` (ct, nct, eq, ne)
    /// - `paymentgroup` (ct, nct, eq, ne)
    /// - `totalAmount` (gt, ge, lt, le, eq, ne)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    ///
    /// List of comparison accepted - enclosed between parentheses:
    /// - eq or empty => equal
    /// - gt => greater than
    /// - ge => greater or equal
    /// - lt => less than
    /// - le => less or equal
    /// - ne => not equal
    /// - ct => contains
    /// - nct => not contains
    /// - in => inside array separated by "|"
    /// - nin => not inside array separated by "|"
    ///
    /// List of parameters accepted:
    /// - limitRecord : max number of records for query (default="20", "0" or negative value for all)
    /// - fromRecord : initial record in query
    ///
    /// Example: totalAmount(gt)=20  return all records with totalAmount greater than 20.00
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
    ///         .export
    ///         .export_bills_org(
    ///             &ExportFormat1::Csv,
    ///             123,
    ///             &ExportBillsOrgQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_bills_org(
        &self,
        format: &ExportFormat1,
        org_id: i64,
        request: &ExportBillsOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!("Export/bills/{}/org/{}", format, org_id),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [List disputes by paypoint](/developers/api-reference/chargebacks/get-list-of-chargebacks-and-returned-transactions-for-an-entrypoint) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Export a list of chargebacks and ACH returns for an entrypoint. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `format` - Format for the export, either XLSX or CSV.
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    /// - `chargebackDate` (gt, ge, lt, le, eq, ne)
    /// - `transId` (ne, eq, ct, nct)
    /// - `method` (in, nin, eq, ne)
    /// - `netAmount` (gt, ge, lt, le, eq, ne)
    /// - `reasonCode` (in, nin, eq, ne)
    /// - `reason` (ct, nct, eq, ne)
    /// - `caseNumber` (ct, nct, eq, ne)
    /// - `status` (in, nin, eq, ne)
    /// - `accountType` (in, nin, eq, ne)
    /// - `payaccountLastfour` (nct, ct)
    /// - `payaccountType` (ne, eq, in, nin)
    /// - `customerFirstname` (ct, nct, eq, ne)
    /// - `customerLastname` (ct, nct, eq, ne)
    /// - `customerName` (ct, nct)
    /// - `customerId` (eq, ne)
    /// - `customerNumber` (ct, nct, eq, ne)
    /// - `customerCompanyname` (ct, nct, eq, ne)
    /// - `customerAddress` (ct, nct, eq, ne)
    /// - `customerCity` (ct, nct, eq, ne)
    /// - `customerZip` (ct, nct, eq, ne)
    /// - `customerState` (ct, nct, eq, ne)
    /// - `customerCountry` (ct, nct, eq, ne)
    /// - `customerPhone` (ct, nct, eq, ne)
    /// - `customerEmail` (ct, nct, eq, ne)
    /// - `customerShippingAddress` (ct, nct, eq, ne)
    /// - `customerShippingCity` (ct, nct, eq, ne)
    /// - `customerShippingZip` (ct, nct, eq, ne)
    /// - `customerShippingState` (ct, nct, eq, ne)
    /// - `customerShippingCountry` (ct, nct, eq, ne)
    /// - `orgId` (eq) *mandatory when entry=org*
    /// - `paypointId` (ne, eq)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    /// - `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name
    ///
    /// List of comparison accepted - enclosed between parentheses:
    /// - eq or empty => equal
    /// - gt => greater than
    /// - ge => greater or equal
    /// - lt => less than
    /// - le => less or equal
    /// - ne => not equal
    /// - ct => contains
    /// - nct => not contains
    /// - in => inside array separated by "|"
    /// - nin => not inside array separated by "|"
    ///
    /// List of parameters accepted:
    /// - limitRecord : max number of records for query (default="20", "0" or negative value for all)
    /// - fromRecord : initial record in query
    ///
    /// Example: `netAmount(gt)=20` returns all records with a `netAmount` greater than 20.00
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
    ///         .export
    ///         .export_chargebacks(
    ///             &ExportFormat1::Csv,
    ///             &"8cfec329267".to_string(),
    ///             &ExportChargebacksQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_chargebacks(
        &self,
        format: &ExportFormat1,
        entry: &str,
        request: &ExportChargebacksQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!("Export/chargebacks/{}/{}", format, entry),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [List disputes by organization](/developers/api-reference/chargebacks/get-list-of-chargebacks-and-returned-transactions-for-an-org) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Export a list of chargebacks and ACH returns for an organization. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `format` - Format for the export, either XLSX or CSV.
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    /// - `chargebackDate` (gt, ge, lt, le, eq, ne)
    /// - `transId` (ne, eq, ct, nct)
    /// - `method` (in, nin, eq, ne)
    /// - `netAmount` (gt, ge, lt, le, eq, ne)
    /// - `reasonCode` (in, nin, eq, ne)
    /// - `reason` (ct, nct, eq, ne)
    /// - `caseNumber` (ct, nct, eq, ne)
    /// - `status` (in, nin, eq, ne)
    /// - `accountType` (in, nin, eq, ne)
    /// - `payaccountLastfour` (nct, ct)
    /// - `payaccountType` (ne, eq, in, nin)
    /// - `customerFirstname` (ct, nct, eq, ne)
    /// - `customerLastname` (ct, nct, eq, ne)
    /// - `customerName` (ct, nct)
    /// - `customerId` (eq, ne)
    /// - `customerNumber` (ct, nct, eq, ne)
    /// - `customerCompanyname` (ct, nct, eq, ne)
    /// - `customerAddress` (ct, nct, eq, ne)
    /// - `customerCity` (ct, nct, eq, ne)
    /// - `customerZip` (ct, nct, eq, ne)
    /// - `customerState` (ct, nct, eq, ne)
    /// - `customerCountry` (ct, nct, eq, ne)
    /// - `customerPhone` (ct, nct, eq, ne)
    /// - `customerEmail` (ct, nct, eq, ne)
    /// - `customerShippingAddress` (ct, nct, eq, ne)
    /// - `customerShippingCity` (ct, nct, eq, ne)
    /// - `customerShippingZip` (ct, nct, eq, ne)
    /// - `customerShippingState` (ct, nct, eq, ne)
    /// - `customerShippingCountry` (ct, nct, eq, ne)
    /// - `orgId` (eq) *mandatory when entry=org*
    /// - `paypointId` (ne, eq)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    /// - `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name
    ///
    /// List of comparison accepted - enclosed between parentheses:
    /// - eq or empty => equal
    /// - gt => greater than
    /// - ge => greater or equal
    /// - lt => less than
    /// - le => less or equal
    /// - ne => not equal
    /// - ct => contains
    /// - nct => not contains
    /// - in => inside array separated by "|"
    /// - nin => not inside array separated by "|"
    ///
    /// List of parameters accepted:
    /// - limitRecord : max number of records for query (default="20", "0" or negative value for all)
    /// - fromRecord : initial record in query
    ///
    /// Example: `netAmount(gt)=20` returns all records with a `netAmount` greater than 20.00
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
    ///         .export
    ///         .export_chargebacks_org(
    ///             &ExportFormat1::Csv,
    ///             123,
    ///             &ExportChargebacksOrgQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_chargebacks_org(
        &self,
        format: &ExportFormat1,
        org_id: i64,
        request: &ExportChargebacksOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!("Export/chargebacks/{}/org/{}", format, org_id),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [List customers by paypoint](/developers/api-reference/customer/get-list-of-customers-for-an-entrypoint) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Export a list of customers for an entrypoint. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `format` - Format for the export, either XLSX or CSV.
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// **List of field names accepted:**
    /// - `createdDate` (gt, ge, lt, le, eq, ne)
    /// - `customernumber` (ne, eq, ct, nct)
    /// - `firstname` (ne, eq, ct, nct)
    /// - `lastname` (ne, eq, ct, nct)
    /// - `name` (ct, nct)
    /// - `address` (ne, eq, ct, nct)
    /// - `city` (ne, eq, ct, nct)
    /// - `country` (ne, eq, ct, nct)
    /// - `zip` (ne, eq, ct, nct)
    /// - `state` (ne, eq, ct, nct)
    /// - `shippingaddress` (ne, eq, ct, nct)
    /// - `shippingcity` (ne, eq, ct, nct)
    /// - `shippingcountry` (ne, eq, ct, nct)
    /// - `shippingzip` (ne, eq, ct, nct)
    /// - `shippingstate` (ne, eq, ct, nct)
    /// - `phone` (ne, eq, ct, nct)
    /// - `email` (ne, eq, ct, nct)
    /// - `company` (ne, eq, ct, nct)
    /// - `username` (ne, eq, ct, nct)
    /// - `balance` (gt, ge, lt, le, eq, ne)
    /// - `status` (in, nin, eq, ne)
    /// - `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name
    /// - `orgId` (eq) *mandatory when entry=org*
    /// - `paypointId` (ne, eq)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    ///
    /// **List of comparison accepted - enclosed between parentheses:**
    /// - eq or empty => equal
    /// - gt => greater than
    /// - ge => greater or equal
    /// - lt => less than
    /// - le => less or equal
    /// - ne => not equal
    /// - ct => contains
    /// - nct => not contains
    /// - in => inside array separated by "|"
    /// - nin => not inside array separated by "|"
    ///
    /// **List of parameters accepted:**
    /// - limitRecord: max number of records for query (default="20", "0" or negative value for all)
    /// - fromRecord: initial record in query
    ///
    /// **Example:**
    /// balance(gt)=20 return all records with balance greater than 20.00
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
    ///         .export
    ///         .export_customers(
    ///             &ExportFormat1::Csv,
    ///             &"8cfec329267".to_string(),
    ///             &ExportCustomersQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_customers(
        &self,
        format: &ExportFormat1,
        entry: &str,
        request: &ExportCustomersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!("Export/customers/{}/{}", format, entry),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [List customers by organization](/developers/api-reference/customer/get-list-of-customers-for-an-organization) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Exports a list of customers for an organization. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `format` - Format for the export, either XLSX or CSV.
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// **List of field names accepted:**
    /// - `createdDate` (gt, ge, lt, le, eq, ne)
    /// - `customernumber` (ne, eq, ct, nct)
    /// - `firstname` (ne, eq, ct, nct)
    /// - `lastname` (ne, eq, ct, nct)
    /// - `name` (ct, nct)
    /// - `address` (ne, eq, ct, nct)
    /// - `city` (ne, eq, ct, nct)
    /// - `country` (ne, eq, ct, nct)
    /// - `zip` (ne, eq, ct, nct)
    /// - `state` (ne, eq, ct, nct)
    /// - `shippingaddress` (ne, eq, ct, nct)
    /// - `shippingcity` (ne, eq, ct, nct)
    /// - `shippingcountry` (ne, eq, ct, nct)
    /// - `shippingzip` (ne, eq, ct, nct)
    /// - `shippingstate` (ne, eq, ct, nct)
    /// - `phone` (ne, eq, ct, nct)
    /// - `email` (ne, eq, ct, nct)
    /// - `company` (ne, eq, ct, nct)
    /// - `username` (ne, eq, ct, nct)
    /// - `balance` (gt, ge, lt, le, eq, ne)
    /// - `status` (in, nin, eq, ne)
    /// - `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name
    /// - `orgId` (eq) *mandatory when entry=org*
    /// - `paypointId` (ne, eq)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    ///
    /// **List of comparison accepted - enclosed between parentheses:**
    /// - eq or empty => equal
    /// - gt => greater than
    /// - ge => greater or equal
    /// - lt => less than
    /// - le => less or equal
    /// - ne => not equal
    /// - ct => contains
    /// - nct => not contains
    /// - in => inside array separated by "|"
    /// - nin => not inside array separated by "|"
    ///
    /// **List of parameters accepted:**
    /// - limitRecord: max number of records for query (default="20", "0" or negative value for all)
    /// - fromRecord: initial record in query
    ///
    /// **Example:**
    /// balance(gt)=20 return all records with balance greater than 20.00
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
    ///         .export
    ///         .export_customers_org(
    ///             &ExportFormat1::Csv,
    ///             123,
    ///             &ExportCustomersOrgQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_customers_org(
        &self,
        format: &ExportFormat1,
        org_id: i64,
        request: &ExportCustomersOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!("Export/customers/{}/org/{}", format, org_id),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [List invoices by paypoint](/developers/api-reference/invoice/get-list-of-invoices-for-an-entrypoint) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Export list of invoices for an entrypoint. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `format` - Format for the export, either XLSX or CSV.
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    /// - `invoiceDate` (gt, ge, lt, le, eq, ne)
    /// - `dueDate` (gt, ge, lt, le, eq, ne)
    /// - `sentDate` (gt, ge, lt, le, eq, ne)
    /// - `frequency`  (in, nin,ne, eq)
    /// - `invoiceType`   (eq, ne)
    /// - `payTerms`   (in, nin, eq, ne)
    /// - `paypointId`  (ne, eq)
    /// - `totalAmount`  (gt, ge, lt, le, eq, ne)
    /// - `paidAmount`  (gt, ge, lt, le, eq, ne)
    /// - `status`   (in, nin, eq, ne)
    /// - `invoiceNumber`   (ct, nct, eq, ne)
    /// - `purchaseOrder`   (ct, nct, eq, ne)
    /// - `itemProductCode` (ct, nct)
    /// - `itemDescription` (ct, nct)
    /// - `customerFirstname`   (ct, nct, eq, ne)
    /// - `customerLastname`    (ct, nct, eq, ne)
    /// - `customerName`   (ct, nct)
    /// - `customerId`  (eq, ne)
    /// - `customerNumber`  (ct, nct, eq, ne)
    /// - `customerCompanyname`    (ct, nct, eq, ne)
    /// - `customerAddress` (ct, nct, eq, ne)
    /// - `customerCity`    (ct, nct, eq, ne)
    /// - `customerZip` (ct, nct, eq, ne)
    /// - `customerState` (ct, nct, eq, ne)
    /// - `customerCountry` (ct, nct, eq, ne)
    /// - `customerPhone` (ct, nct, eq, ne)
    /// - `customerEmail` (ct, nct, eq, ne)
    /// - `customerShippingAddress` (ct, nct, eq, ne)
    /// - `customerShippingCity` (ct, nct, eq, ne)
    /// - `customerShippingZip` (ct, nct, eq, ne)
    /// - `customerShippingState` (ct, nct, eq, ne)
    /// - `customerShippingCountry` (ct, nct, eq, ne)
    /// - `orgId`  (eq)
    /// - `paylinkId`  (ne, eq)
    /// - `paypointLegal`  (ne, eq, ct, nct)
    /// - `paypointDba`  (ne, eq, ct, nct)
    /// - `orgName`  (ne, eq, ct, nct)
    /// - `additional-xxx`  (ne, eq, ct, nct) where xxx is the additional field name
    ///
    /// List of comparison accepted - enclosed between parentheses:
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
    /// List of parameters accepted:
    /// - `limitRecord` : max number of records for query (default="20", "0" or negative value for all)
    /// - `fromRecord` : initial record in query
    ///
    /// Example: `totalAmount(gt)=20` returns all records with `totalAmount` greater than 20.00
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
    ///         .export
    ///         .export_invoices(
    ///             &ExportFormat1::Csv,
    ///             &"8cfec329267".to_string(),
    ///             &ExportInvoicesQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_invoices(
        &self,
        format: &ExportFormat1,
        entry: &str,
        request: &ExportInvoicesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!("Export/invoices/{}/{}", format, entry),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [List invoices by organization](/developers/api-reference/invoice/get-list-of-invoices-for-an-organization) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Export a list of invoices for an organization. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `format` - Format for the export, either XLSX or CSV.
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    /// - `invoiceDate` (gt, ge, lt, le, eq, ne)
    /// - `dueDate` (gt, ge, lt, le, eq, ne)
    /// - `sentDate` (gt, ge, lt, le, eq, ne)
    /// - `frequency` (in, nin,ne, eq)
    /// - `invoiceType` (eq, ne)
    /// - `payTerms` (in, nin, eq, ne)
    /// - `paypointId` (ne, eq)
    /// - `totalAmount` (gt, ge, lt, le, eq, ne)
    /// - `paidAmount` (gt, ge, lt, le, eq, ne)
    /// - `status` (in, nin, eq, ne)
    /// - `invoiceNumber` (ct, nct, eq, ne)
    /// - `purchaseOrder` (ct, nct, eq, ne)
    /// - `itemProductCode` (ct, nct)
    /// - `itemDescription` (ct, nct)
    /// - `customerFirstname` (ct, nct, eq, ne)
    /// - `customerLastname` (ct, nct, eq, ne)
    /// - `customerName` (ct, nct)
    /// - `customerId` (eq, ne)
    /// - `customerNumber` (ct, nct, eq, ne)
    /// - `customerCompanyname` (ct, nct, eq, ne)
    /// - `customerAddress` (ct, nct, eq, ne)
    /// - `customerCity` (ct, nct, eq, ne)
    /// - `customerZip` (ct, nct, eq, ne)
    /// - `customerState` (ct, nct, eq, ne)
    /// - `customerCountry` (ct, nct, eq, ne)
    /// - `customerPhone` (ct, nct, eq, ne)
    /// - `customerEmail` (ct, nct, eq, ne)
    /// - `customerShippingAddress` (ct, nct, eq, ne)
    /// - `customerShippingCity` (ct, nct, eq, ne)
    /// - `customerShippingZip` (ct, nct, eq, ne)
    /// - `customerShippingState` (ct, nct, eq, ne)
    /// - `customerShippingCountry` (ct, nct, eq, ne)
    /// - `orgId` (eq)
    /// - `paylinkId` (ne, eq)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    /// - `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name
    ///
    /// List of comparison accepted - enclosed between parentheses:
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
    /// List of parameters accepted:
    /// - limitRecord : max number of records for query (default="20", "0" or negative value for all)
    /// - fromRecord : initial record in query
    ///
    /// Example: totalAmount(gt)=20  return all records with totalAmount greater than 20.00
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
    ///         .export
    ///         .export_invoices_org(
    ///             &ExportFormat1::Csv,
    ///             123,
    ///             &ExportInvoicesOrgQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_invoices_org(
        &self,
        format: &ExportFormat1,
        org_id: i64,
        request: &ExportInvoicesOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!("Export/invoices/{}/org/{}", format, org_id),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [List suborganizations by organization](/developers/api-reference/organization/get-list-of-organizations-for-entrypoint) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Export a list of child organizations (suborganizations) for a parent organization.
    ///
    /// # Arguments
    ///
    /// * `format` - Format for the export, either XLSX or CSV.
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    /// - `name` (ct, nct, eq, ne)
    /// - `type` (ne, eq)
    /// - `contactName` (ct, nct, eq, ne)
    /// - `contactTitle` (ct, nct, eq, ne)
    /// - `contactEmail` (ct, nct, eq, ne)
    /// - `contactPhone` (ct, nct, eq, ne)
    /// - `city` (ct, nct, eq, ne)
    /// - `state` (in, nin, eq, ne)
    /// - `address` (ct, nct, eq, ne)
    /// - `country` (ct, nct, eq, ne)
    /// - `zip` (ct, nct, eq, ne)
    /// - `hasBilling` any value greater than zero is taken as TRUE otherwise is FALSE
    /// - `hasResidual` any value greater than zero is taken as TRUE otherwise is FALSE
    ///
    /// List of comparison accepted - enclosed between parentheses:
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
    /// List of parameters accepted:
    /// - limitRecord : max number of records for query (default="20", "0" or negative value for all)
    /// - fromRecord : initial record in query
    ///
    /// Example: name(ct)=hoa  return all records where name contains "hoa"
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
    ///         .export
    ///         .export_organizations(
    ///             &ExportFormat1::Csv,
    ///             123,
    ///             &ExportOrganizationsQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_organizations(
        &self,
        format: &ExportFormat1,
        org_id: i64,
        request: &ExportOrganizationsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!("Export/organizations/{}/org/{}", format, org_id),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [List payouts by paypoint](/developers/api-reference/query/get-list-of-payouts-for-entrypoint) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Export a list of payouts and their statuses for an entrypoint. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `format` - Format for the export, either XLSX or CSV.
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    /// - `status` (in, nin, eq, ne)
    /// - `transactionDate` (gt, ge, lt, le, eq, ne)
    /// - `billNumber` (ct, nct)
    /// - `vendorNumber` (ct, nct, eq, ne)
    /// - `vendorName` (ct, nct, eq, ne)
    /// - `paymentMethod` (ct, nct, eq, ne)
    /// - `paymentId` (ct, nct, eq, ne)
    /// - `paymentgroup` (ct, nct, eq, ne)
    /// - `totalAmount` (gt, ge, lt, le, eq, ne)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    ///
    /// List of comparison accepted - enclosed between parentheses:
    /// - eq or empty => equal
    /// - gt => greater than
    /// - ge => greater or equal
    /// - lt => less than
    /// - le => less or equal
    /// - ne => not equal
    /// - ct => contains
    /// - nct => not contains
    /// - in => inside array separated by "|"
    /// - nin => not inside array separated by "|"
    ///
    /// List of parameters accepted:
    /// - limitRecord: max number of records for query (default="20", "0" or negative value for all)
    /// - fromRecord: initial record in query
    ///
    /// Example: totalAmount(gt)=20 return all records with totalAmount greater than 20.00
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
    ///         .export
    ///         .export_payout(
    ///             &ExportFormat1::Csv,
    ///             &"8cfec329267".to_string(),
    ///             &ExportPayoutQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_payout(
        &self,
        format: &ExportFormat1,
        entry: &str,
        request: &ExportPayoutQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!("Export/payouts/{}/{}", format, entry),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [List payouts by org](/developers/api-reference/query/get-list-of-payouts-for-organization) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Export a list of payouts and their details for an organization. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `format` - Format for the export, either XLSX or CSV.
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    /// - `status` (in, nin, eq, ne)
    /// - `transactionDate` (gt, ge, lt, le, eq, ne)
    /// - `billNumber` (ct, nct)
    /// - `vendorNumber` (ct, nct, eq, ne)
    /// - `vendorName` (ct, nct, eq, ne)
    /// - `paymentMethod` (ct, nct, eq, ne)
    /// - `paymentId` (ct, nct, eq, ne)
    /// - `paymentgroup` (ct, nct, eq, ne)
    /// - `totalAmount` (gt, ge, lt, le, eq, ne)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    ///
    /// List of comparison accepted - enclosed between parentheses:
    /// - eq or empty => equal
    /// - gt => greater than
    /// - ge => greater or equal
    /// - lt => less than
    /// - le => less or equal
    /// - ne => not equal
    /// - ct => contains
    /// - nct => not contains
    /// - in => inside array separated by "|"
    /// - nin => not inside array separated by "|"
    ///
    /// List of parameters accepted:
    /// - limitRecord: max number of records for query (default="20", "0" or negative value for all)
    /// - fromRecord: initial record in query
    ///
    /// Example: totalAmount(gt)=20 return all records with totalAmount greater than 20.00
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
    ///         .export
    ///         .export_payout_org(
    ///             &ExportFormat1::Csv,
    ///             123,
    ///             &ExportPayoutOrgQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_payout_org(
        &self,
        format: &ExportFormat1,
        org_id: i64,
        request: &ExportPayoutOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!("Export/payouts/{}/org/{}", format, org_id),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [List paypoints by organization](/developers/api-reference/paypoint/get-list-of-paypoints-for-an-organization) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Export a list of paypoints in an organization. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `format` - Format for the export, either XLSX or CSV.
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    /// - `createdAt` (gt, ge, lt, le, eq, ne)
    /// - `startDate` (gt, ge, lt, le, eq, ne)
    /// - `dbaname` (ct, nct)
    /// - `legalname` (ct, nct)
    /// - `ein` (ct, nct)
    /// - `address` (ct, nct)
    /// - `city` (ct, nct)
    /// - `state` (ct, nct)
    /// - `phone` (ct, nct)
    /// - `mcc` (ct, nct)
    /// - `owntype` (ct, nct)
    /// - `ownerName` (ct, nct)
    /// - `contactName` (ct, nct)
    /// - `orgParentname` (ct, nct)
    ///
    /// List of comparison accepted - enclosed between parentheses:
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
    /// List of parameters accepted:
    /// - limitRecord : max number of records for query (default="20", "0" or negative value for all)
    /// - fromRecord : initial record in query
    ///
    /// Example: `dbaname(ct)=hoa` returns all records with `dbaname` containing "hoa"
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
    ///         .export
    ///         .export_paypoints(
    ///             &ExportFormat1::Csv,
    ///             123,
    ///             &ExportPaypointsQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_paypoints(
        &self,
        format: &ExportFormat1,
        org_id: i64,
        request: &ExportPaypointsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!("Export/paypoints/{}/{}", format, org_id),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [List settled transactions for paypoint](/developers/api-reference/query/get-list-of-settled-transactions-for-an-entrypoint) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Export a list of settled transactions for an entrypoint. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `format` - Format for the export, either XLSX or CSV.
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    /// - `settlementDate` (gt, ge, lt, le, eq, ne)
    /// - `transId` (ne, eq, ct, nct)
    /// - `gatewayTransId` (ne, eq, ct, nct)
    /// - `method` (in, nin, eq, ne)
    /// - `settledAmount` (gt, ge, lt, le, eq, ne)
    /// - `operation` (in, nin, eq, ne)
    /// - `source` (in, nin, eq, ne)
    /// - `batchNumber` (ct, nct, eq, ne)
    /// - `payaccountLastfour` (nct, ct)
    /// - `payaccountType` (ne, eq, in, nin)
    /// - `customerFirstname` (ct, nct, eq, ne)
    /// - `customerLastname` (ct, nct, eq, ne)
    /// - `customerName` (ct, nct)
    /// - `customerId` (eq, ne)
    /// - `customerNumber` (ct, nct, eq, ne)
    /// - `customerCompanyname` (ct, nct, eq, ne)
    /// - `customerAddress` (ct, nct, eq, ne)
    /// - `customerCity` (ct, nct, eq, ne)
    /// - `customerZip` (ct, nct, eq, ne)
    /// - `customerState` (ct, nct, eq, ne)
    /// - `customerCountry` (ct, nct, eq, ne)
    /// - `customerPhone` (ct, nct, eq, ne)
    /// - `customerEmail` (ct, nct, eq, ne)
    /// - `customerShippingAddress` (ct, nct, eq, ne)
    /// - `customerShippingCity` (ct, nct, eq, ne)
    /// - `customerShippingZip` (ct, nct, eq, ne)
    /// - `customerShippingState` (ct, nct, eq, ne)
    /// - `customerShippingCountry` (ct, nct, eq, ne)
    /// - `orgId` (eq) *mandatory when entry=org*
    /// - `paypointId` (ne, eq)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    /// - `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name
    ///
    /// List of comparison accepted - enclosed between parentheses:
    /// - eq or empty => equal
    /// - gt => greater than
    /// - ge => greater or equal
    /// - lt => less than
    /// - le => less or equal
    /// - ne => not equal
    /// - ct => contains
    /// - nct => not contains
    /// - in => inside array separated by "|"
    /// - nin => not inside array separated by "|"
    ///
    /// List of parameters accepted:
    /// - limitRecord: max number of records for query (default="20", "0" or negative value for all)
    /// - fromRecord: initial record in query
    ///
    /// Example: `settledAmount(gt)=20` returns all records with a `settledAmount` greater than 20.00
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
    ///         .export
    ///         .export_settlements(
    ///             &ExportFormat1::Csv,
    ///             &"8cfec329267".to_string(),
    ///             &ExportSettlementsQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_settlements(
        &self,
        format: &ExportFormat1,
        entry: &str,
        request: &ExportSettlementsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!("Export/settlements/{}/{}", format, entry),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [List settled transactions for org](/developers/api-reference/query/get-list-of-settled-transactions-for-an-org) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Export a list of settled transactions for an organization. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `format` - Format for the export, either XLSX or CSV.
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    /// - `settlementDate` (gt, ge, lt, le, eq, ne)
    /// - `transId` (ne, eq, ct, nct)
    /// - `gatewayTransId` (ne, eq, ct, nct)
    /// - `method` (in, nin, eq, ne)
    /// - `settledAmount` (gt, ge, lt, le, eq, ne)
    /// - `operation` (in, nin, eq, ne)
    /// - `source` (in, nin, eq, ne)
    /// - `batchNumber` (ct, nct, eq, ne)
    /// - `payaccountLastfour` (nct, ct)
    /// - `payaccountType` (ne, eq, in, nin)
    /// - `customerFirstname` (ct, nct, eq, ne)
    /// - `customerLastname` (ct, nct, eq, ne)
    /// - `customerName` (ct, nct)
    /// - `customerId` (eq, ne)
    /// - `customerNumber` (ct, nct, eq, ne)
    /// - `customerCompanyname` (ct, nct, eq, ne)
    /// - `customerAddress` (ct, nct, eq, ne)
    /// - `customerCity` (ct, nct, eq, ne)
    /// - `customerZip` (ct, nct, eq, ne)
    /// - `customerState` (ct, nct, eq, ne)
    /// - `customerCountry` (ct, nct, eq, ne)
    /// - `customerPhone` (ct, nct, eq, ne)
    /// - `customerEmail` (ct, nct, eq, ne)
    /// - `customerShippingAddress` (ct, nct, eq, ne)
    /// - `customerShippingCity` (ct, nct, eq, ne)
    /// - `customerShippingZip` (ct, nct, eq, ne)
    /// - `customerShippingState` (ct, nct, eq, ne)
    /// - `customerShippingCountry` (ct, nct, eq, ne)
    /// - `orgId` (eq) *mandatory when entry=org*
    /// - `paypointId` (ne, eq)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    /// - `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name
    ///
    /// List of comparison accepted - enclosed between parentheses:
    /// - eq or empty => equal
    /// - gt => greater than
    /// - ge => greater or equal
    /// - lt => less than
    /// - le => less or equal
    /// - ne => not equal
    /// - ct => contains
    /// - nct => not contains
    /// - in => inside array separated by "|"
    /// - nin => not inside array separated by "|"
    ///
    /// List of parameters accepted:
    /// - limitRecord: max number of records for query (default="20", "0" or negative value for all)
    /// - fromRecord: initial record in query
    ///
    /// Example: `settledAmount(gt)=20` returns all records with a `settledAmount` greater than 20.00
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
    ///         .export
    ///         .export_settlements_org(
    ///             &ExportFormat1::Csv,
    ///             123,
    ///             &ExportSettlementsOrgQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_settlements_org(
        &self,
        format: &ExportFormat1,
        org_id: i64,
        request: &ExportSettlementsOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!("Export/settlements/{}/org/{}", format, org_id),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [List subscriptions by paypoint](/developers/api-reference/subscription/get-list-of-subscriptions-for-an-entrypoint) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Export a list of subscriptions for an entrypoint. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `format` - Format for the export, either XLSX or CSV.
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    /// - `startDate` (gt, ge, lt, le, eq, ne)
    /// - `endDate` (gt, ge, lt, le, eq, ne)
    /// - `nextDate` (gt, ge, lt, le, eq, ne)
    /// - `frequency` (in, nin, ne, eq)
    /// - `method` (in, nin, eq, ne)
    /// - `totalAmount` (gt, ge, lt, le, eq, ne)
    /// - `netAmount` (gt, ge, lt, le, eq, ne)
    /// - `feeAmount` (gt, ge, lt, le, eq, ne)
    /// - `status` (in, nin, eq, ne)
    /// - `untilcancelled` (eq, ne)
    /// - `payaccountLastfour` (nct, ct)
    /// - `payaccountType` (ne, eq, in, nin)
    /// - `customerFirstname` (ct, nct, eq, ne)
    /// - `customerLastname` (ct, nct, eq, ne)
    /// - `customerName` (ct, nct)
    /// - `customerId` (eq, ne)
    /// - `customerNumber` (ct, nct, eq, ne)
    /// - `customerCompanyname` (ct, nct, eq, ne)
    /// - `customerAddress` (ct, nct, eq, ne)
    /// - `customerCity` (ct, nct, eq, ne)
    /// - `customerZip` (ct, nct, eq, ne)
    /// - `customerState` (ct, nct, eq, ne)
    /// - `customerCountry` (ct, nct, eq, ne)
    /// - `customerPhone` (ct, nct, eq, ne)
    /// - `customerEmail` (ct, nct, eq, ne)
    /// - `customerShippingAddress` (ct, nct, eq, ne)
    /// - `customerShippingCity` (ct, nct, eq, ne)
    /// - `customerShippingZip` (ct, nct, eq, ne)
    /// - `customerShippingState` (ct, nct, eq, ne)
    /// - `customerShippingCountry` (ct, nct, eq, ne)
    /// - `orgId` (eq)
    /// - `paypointId` (ne, eq)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    /// - `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name
    ///
    /// List of comparison accepted - enclosed between parentheses:
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
    /// List of parameters accepted:
    /// - limitRecord : max number of records for query (default="20", "0" or negative value for all)
    /// - fromRecord : initial record in query
    ///
    /// Example: `netAmount(gt)=20` returns all records with a `netAmount` greater than 20.00
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
    ///         .export
    ///         .export_subscriptions(
    ///             &ExportFormat1::Csv,
    ///             &"8cfec329267".to_string(),
    ///             &ExportSubscriptionsQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_subscriptions(
        &self,
        format: &ExportFormat1,
        entry: &str,
        request: &ExportSubscriptionsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!("Export/subscriptions/{}/{}", format, entry),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [List subscriptions by organization](/developers/api-reference/subscription/get-list-of-subscriptions-for-an-org) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Export a list of subscriptions for an organization. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `format` - Format for the export, either XLSX or CSV.
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    /// - `startDate` (gt, ge, lt, le, eq, ne)
    /// - `endDate` (gt, ge, lt, le, eq, ne)
    /// - `nextDate` (gt, ge, lt, le, eq, ne)
    /// - `frequency` (in, nin, ne, eq)
    /// - `method` (in, nin, eq, ne)
    /// - `totalAmount` (gt, ge, lt, le, eq, ne)
    /// - `netAmount` (gt, ge, lt, le, eq, ne)
    /// - `feeAmount` (gt, ge, lt, le, eq, ne)
    /// - `status` (in, nin, eq, ne)
    /// - `untilcancelled` (eq, ne)
    /// - `payaccountLastfour` (nct, ct)
    /// - `payaccountType` (ne, eq, in, nin)
    /// - `customerFirstname` (ct, nct, eq, ne)
    /// - `customerLastname` (ct, nct, eq, ne)
    /// - `customerName` (ct, nct)
    /// - `customerId` (eq, ne)
    /// - `customerNumber` (ct, nct, eq, ne)
    /// - `customerCompanyname` (ct, nct, eq, ne)
    /// - `customerAddress` (ct, nct, eq, ne)
    /// - `customerCity` (ct, nct, eq, ne)
    /// - `customerZip` (ct, nct, eq, ne)
    /// - `customerState` (ct, nct, eq, ne)
    /// - `customerCountry` (ct, nct, eq, ne)
    /// - `customerPhone` (ct, nct, eq, ne)
    /// - `customerEmail` (ct, nct, eq, ne)
    /// - `customerShippingAddress` (ct, nct, eq, ne)
    /// - `customerShippingCity` (ct, nct, eq, ne)
    /// - `customerShippingZip` (ct, nct, eq, ne)
    /// - `customerShippingState` (ct, nct, eq, ne)
    /// - `customerShippingCountry` (ct, nct, eq, ne)
    /// - `orgId` (eq)
    /// - `paypointId` (ne, eq)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    /// - `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name
    ///
    /// List of comparison accepted - enclosed between parentheses:
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
    /// List of parameters accepted:
    /// - limitRecord : max number of records for query (default="20", "0" or negative value for all)
    /// - fromRecord : initial record in query
    ///
    /// Example: `netAmount(gt)=20` returns all records with a `netAmount` greater than 20.00
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
    ///         .export
    ///         .export_subscriptions_org(
    ///             &ExportFormat1::Csv,
    ///             123,
    ///             &ExportSubscriptionsOrgQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_subscriptions_org(
        &self,
        format: &ExportFormat1,
        org_id: i64,
        request: &ExportSubscriptionsOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!("Export/subscriptions/{}/org/{}", format, org_id),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [List transactions for paypoint](/developers/api-reference/query/get-list-of-transactions-for-an-entrypoint) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Export a list of transactions for an entrypoint in a file in XLSX or CSV format. Use filters to limit results. If you don't specify a date range in the request, the last two months of data are returned.
    ///
    /// # Arguments
    ///
    /// * `format` - Format for the export, either XLSX or CSV.
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    /// - `transactionDate` (gt, ge, lt, le, eq, ne)
    /// - `transId` (ne, eq, ct, nct)
    /// - `gatewayTransId` (ne, eq, ct, nct)
    /// - `orderId` (ne, eq)
    /// - `idTrans` (ne, eq)
    /// - `orgId` (ne, eq)
    /// - `paypointId` (ne, eq)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    /// - `method` (in, nin, eq, ne)
    /// - `totalAmount` (gt, ge, lt, le, eq, ne)
    /// - `netAmount` (gt, ge, lt, le, eq, ne)
    /// - `feeAmount` (gt, ge, lt, le, eq, ne)
    /// - `operation` (in, nin, eq, ne)
    /// - `source` (in, nin, eq, ne)
    /// - `status` (in, nin, eq, ne)
    /// - `settlementStatus` (in, nin, eq, ne)
    /// - `batchNumber` (nct, ct)
    /// - `payaccountLastfour` (nct, ct)
    /// - `payaccountType` (ne, eq, in, nin)
    /// - `customerFirstname` (ct, nct, eq, ne)
    /// - `customerLastname` (ct, nct, eq, ne)
    /// - `customerName` (ct, nct)
    /// - `customerId` (eq, ne)
    /// - `customerNumber` (ct, nct, eq, ne)
    /// - `customerCompanyname` (ct, nct, eq, ne)
    /// - `customerAddress` (ct, nct, eq, ne)
    /// - `customerCity` (ct, nct, eq, ne)
    /// - `customerZip` (ct, nct, eq, ne)
    /// - `customerState` (ct, nct, eq, ne)
    /// - `customerCountry` (ct, nct, eq, ne)
    /// - `customerPhone` (ct, nct, eq, ne)
    /// - `customerEmail` (ct, nct, eq, ne)
    /// - `customerShippingAddress` (ct, nct, eq, ne)
    /// - `customerShippingCity` (ct, nct, eq, ne)
    /// - `customerShippingZip` (ct, nct, eq, ne)
    /// - `customerShippingState` (ct, nct, eq, ne)
    /// - `customerShippingCountry` (ct, nct, eq, ne)
    /// - `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name
    ///
    /// List of comparison accepted - enclosed between parentheses:
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
    /// List of parameters accepted:
    /// - limitRecord : max number of records for query (default="20", "0" or negative value for all)
    /// - fromRecord : initial record in query
    ///
    /// Example: `netAmount(gt)=20` returns all records with a `netAmount` greater than 20.00
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
    ///         .export
    ///         .export_transactions(
    ///             &ExportFormat1::Csv,
    ///             &"8cfec329267".to_string(),
    ///             &ExportTransactionsQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_transactions(
        &self,
        format: &ExportFormat1,
        entry: &str,
        request: &ExportTransactionsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!("Export/transactions/{}/{}", format, entry),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [List transactions for org](/developers/api-reference/query/get-list-of-transactions-for-an-organization) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Export a list of transactions for an org in a file in XLSX or CSV format. Use filters to limit results. If you don't specify a date range in the request, the last two months of data are returned.
    ///
    /// # Arguments
    ///
    /// * `format` - Format for the export, either XLSX or CSV.
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    /// - `transactionDate` (gt, ge, lt, le, eq, ne)
    /// - `transId` (ne, eq, ct, nct)
    /// - `gatewayTransId` (ne, eq, ct, nct)
    /// - `orderId` (ne, eq)
    /// - `idTrans` (ne, eq)
    /// - `orgId` (ne, eq)
    /// - `paypointId` (ne, eq)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    /// - `method` (in, nin, eq, ne)
    /// - `totalAmount` (gt, ge, lt, le, eq, ne)
    /// - `netAmount` (gt, ge, lt, le, eq, ne)
    /// - `feeAmount` (gt, ge, lt, le, eq, ne)
    /// - `operation` (in, nin, eq, ne)
    /// - `source` (in, nin, eq, ne)
    /// - `status` (in, nin, eq, ne)
    /// - `settlementStatus` (in, nin, eq, ne)
    /// - `batchNumber` (nct, ct)
    /// - `payaccountLastfour` (nct, ct)
    /// - `payaccountType` (ne, eq, in, nin)
    /// - `customerFirstname` (ct, nct, eq, ne)
    /// - `customerLastname` (ct, nct, eq, ne)
    /// - `customerName` (ct, nct)
    /// - `customerId` (eq, ne)
    /// - `customerNumber` (ct, nct, eq, ne)
    /// - `customerCompanyname` (ct, nct, eq, ne)
    /// - `customerAddress` (ct, nct, eq, ne)
    /// - `customerCity` (ct, nct, eq, ne)
    /// - `customerZip` (ct, nct, eq, ne)
    /// - `customerState` (ct, nct, eq, ne)
    /// - `customerCountry` (ct, nct, eq, ne)
    /// - `customerPhone` (ct, nct, eq, ne)
    /// - `customerEmail` (ct, nct, eq, ne)
    /// - `customerShippingAddress` (ct, nct, eq, ne)
    /// - `customerShippingCity` (ct, nct, eq, ne)
    /// - `customerShippingZip` (ct, nct, eq, ne)
    /// - `customerShippingState` (ct, nct, eq, ne)
    /// - `customerShippingCountry` (ct, nct, eq, ne)
    /// - `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name
    ///
    /// List of comparison accepted - enclosed between parentheses:
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
    /// List of parameters accepted:
    /// - limitRecord : max number of records for query (default="20", "0" or negative value for all)
    /// - fromRecord : initial record in query
    ///
    /// Example: `netAmount(gt)=20` returns all records with a `netAmount` greater than 20.00
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
    ///         .export
    ///         .export_transactions_org(
    ///             &ExportFormat1::Csv,
    ///             123,
    ///             &ExportTransactionsOrgQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_transactions_org(
        &self,
        format: &ExportFormat1,
        org_id: i64,
        request: &ExportTransactionsOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!("Export/transactions/{}/org/{}", format, org_id),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [Get transfer details](/developers/api-reference/query/get-list-of-transfer-details) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Export a list of transfer details for an entrypoint. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `format` - Format for the export, either XLSX or CSV.
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
    /// * `transfer_id` - Transfer identifier.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    ///
    /// - `grossAmount` (gt, ge, lt, le, eq, ne)
    ///
    /// - `chargeBackAmount` (gt, ge, lt, le, eq, ne)
    ///
    /// - `returnedAmount` (gt, ge, lt, le, eq, ne)
    ///
    /// - `billingFeeAmount` (gt, ge, lt, le, eq, ne)
    ///
    /// - `thirdPartyPaidAmount` (gt, ge, lt, le, eq, ne)
    ///
    /// - `netFundedAmount` (gt, ge, lt, le, eq, ne)
    ///
    /// - `adjustmentAmount` (gt, ge, lt, le, eq, ne)
    ///
    /// - `transactionId` (eq, ne, in, nin)
    ///
    /// - `category` (eq, ne, ct, nct)
    ///
    /// - `type` (eq, ne, in, nin)
    ///
    /// - `method` (eq, ne, in, nin)
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
    ///         .export
    ///         .export_transfer_details(
    ///             &ExportFormat1::Csv,
    ///             &"8cfec329267".to_string(),
    ///             4521,
    ///             &ExportTransferDetailsQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 sort_by: Some("desc(field_name)".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_transfer_details(
        &self,
        format: &ExportFormat1,
        entry: &str,
        transfer_id: i64,
        request: &ExportTransferDetailsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!(
                    "Export/transferDetails/{}/{}/{}",
                    format, entry, transfer_id
                ),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .string("sortBy", request.sort_by.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [List transfers](/developers/api-reference/query/get-list-of-transfers) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Get a list of transfers for an entrypoint. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    /// - `transferDate` (gt, ge, lt, le, eq, ne)
    ///
    /// - `grossAmount` (gt, ge, lt, le, eq, ne)
    ///
    /// - `chargeBackAmount` (gt, ge, lt, le, eq, ne)
    ///
    /// - `returnedAmount` (gt, ge, lt, le, eq, ne)
    ///
    /// - `billingFeeAmount` (gt, ge, lt, le, eq, ne)
    ///
    /// - `thirdPartyPaidAmount` (gt, ge, lt, le, eq, ne)
    ///
    /// - `netFundedAmount` (gt, ge, lt, le, eq, ne)
    ///
    /// - `adjustmentAmount` (gt, ge, lt, le, eq, ne)
    ///
    /// - `processor` (ne, eq, ct, nct)
    ///
    /// - `transferStatus` (ne, eq, in, nin)
    ///
    /// - `batchNumber` (ne, eq, ct, nct)
    ///
    /// - `batchId` (ne, eq, in, nin)
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
    ///         .export
    ///         .export_transfers(
    ///             &"8cfec329267".to_string(),
    ///             &ExportTransfersQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 sort_by: Some("desc(field_name)".to_string()),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_transfers(
        &self,
        entry: &str,
        request: &ExportTransfersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!("Export/transfers/{}", entry),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .string("sortBy", request.sort_by.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [List vendors by paypoint](/developers/api-reference/vendor/get-list-of-vendors-for-entrypoint) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Export a list of vendors for an entrypoint. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `format` - Format for the export, either XLSX or CSV.
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    /// - `method` (in, nin, eq, ne)
    /// - `enrollmentStatus` (in, nin, eq, ne)
    /// - `status` (in, nin, eq, ne)
    /// - `vendorNumber` (ct, nct, eq, ne)
    /// - `name` (ct, nct, eq, ne)
    /// - `ein` (ct, nct, eq, ne)
    /// - `phone` (ct, nct, eq, ne)
    /// - `email` (ct, nct, eq, ne)
    /// - `address` (ct, nct, eq, ne)
    /// - `city` (ct, nct, eq, ne)
    /// - `state` (ct, nct, eq, ne)
    /// - `country` (ct, nct, eq, ne)
    /// - `zip` (ct, nct, eq, ne)
    /// - `mcc` (ct, nct, eq, ne)
    /// - `locationCode` (ct, nct, eq, ne)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    ///
    /// List of comparison accepted - enclosed between parentheses:
    /// - eq or empty => equal
    /// - gt => greater than
    /// - ge => greater or equal
    /// - lt => less than
    /// - le => less or equal
    /// - ne => not equal
    /// - ct => contains
    /// - nct => not contains
    /// - in => inside array separated by "|"
    /// - nin => not inside array separated by "|"
    ///
    /// List of parameters accepted:
    /// - limitRecord : max number of records for query (default="20", "0" or negative value for all)
    /// - fromRecord : initial record in query
    ///
    /// Example: `netAmount(gt)=20` returns all records with a `netAmount` greater than 20.00
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
    ///         .export
    ///         .export_vendors(
    ///             &ExportFormat1::Csv,
    ///             &"8cfec329267".to_string(),
    ///             &ExportVendorsQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_vendors(
        &self,
        format: &ExportFormat1,
        entry: &str,
        request: &ExportVendorsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!("Export/vendors/{}/{}", format, entry),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }

    /// <Warning>
    /// This endpoint is deprecated. To export this data, use [List vendors by organization](/developers/api-reference/vendor/get-list-of-vendors-for-organization) with the `exportFormat` query parameter instead.
    /// </Warning>
    ///
    /// Export a list of vendors for an organization. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `format` - Format for the export, either XLSX or CSV.
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
    ///
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
    ///
    /// For example:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/transactions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    /// - `method` (in, nin, eq, ne)
    /// - `enrollmentStatus` (in, nin, eq, ne)
    /// - `status` (in, nin, eq, ne)
    /// - `vendorNumber` (ct, nct, eq, ne)
    /// - `name` (ct, nct, eq, ne)
    /// - `ein` (ct, nct, eq, ne)
    /// - `phone` (ct, nct, eq, ne)
    /// - `email` (ct, nct, eq, ne)
    /// - `address` (ct, nct, eq, ne)
    /// - `city` (ct, nct, eq, ne)
    /// - `state` (ct, nct, eq, ne)
    /// - `country` (ct, nct, eq, ne)
    /// - `zip` (ct, nct, eq, ne)
    /// - `mcc` (ct, nct, eq, ne)
    /// - `locationCode` (ct, nct, eq, ne)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    ///
    /// List of comparison accepted - enclosed between parentheses:
    /// - eq or empty => equal
    /// - gt => greater than
    /// - ge => greater or equal
    /// - lt => less than
    /// - le => less or equal
    /// - ne => not equal
    /// - ct => contains
    /// - nct => not contains
    /// - in => inside array separated by "|"
    /// - nin => not inside array separated by "|"
    ///
    /// List of parameters accepted:
    /// - limitRecord : max number of records for query (default="20", "0" or negative value for all)
    /// - fromRecord : initial record in query
    ///
    /// Example: `netAmount(gt)=20` returns all records with a `netAmount` greater than 20.00
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
    ///         .export
    ///         .export_vendors_org(
    ///             &ExportFormat1::Csv,
    ///             123,
    ///             &ExportVendorsOrgQueryRequest {
    ///                 columns_export: Some("BatchDate:Batch_Date,PaypointName:Legal_name".to_string()),
    ///                 from_record: Some(251),
    ///                 limit_record: Some(1000),
    ///                 ..Default::default()
    ///             },
    ///             None,
    ///         )
    ///         .await;
    /// }
    /// ```
    pub async fn export_vendors_org(
        &self,
        format: &ExportFormat1,
        org_id: i64,
        request: &ExportVendorsOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<File, ApiError> {
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
                &format!("Export/vendors/{}/org/{}", format, org_id),
                None,
                QueryBuilder::new()
                    .string("columnsExport", request.columns_export.clone())
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .build(),
                options,
            )
            .await
    }
}
