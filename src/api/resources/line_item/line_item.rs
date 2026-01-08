use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct LineItemClient {
    pub http_client: HttpClient,
}

impl LineItemClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Adds products and services to an entrypoint's catalog. These are used as line items for invoicing and transactions. In the response, "responseData" displays the item's code.
    ///
    /// # Arguments
    ///
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn add_item(
        &self,
        entry: &String,
        request: &LineItem,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse6, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("LineItem/{}", entry),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// Deletes an item.
    ///
    /// # Arguments
    ///
    /// * `line_item_id` - ID for the line item (also known as a product, service, or item).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete_item(
        &self,
        line_item_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<DeleteItemResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::DELETE,
                &format!("LineItem/{}", line_item_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Gets an item by ID.
    ///
    /// # Arguments
    ///
    /// * `line_item_id` - ID for the line item (also known as a product, service, or item).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_item(
        &self,
        line_item_id: i64,
        options: Option<RequestOptions>,
    ) -> Result<LineItemQueryRecord, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("LineItem/{}", line_item_id),
                None,
                None,
                options,
            )
            .await
    }

    /// Retrieves a list of line items and their details from an entrypoint. Line items are also known as items, products, and services. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
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
    ///
    /// </Info>
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    ///
    /// - `categories` (ct, nct)
    /// - `code` (ne, eq, ct, nct)
    /// - `commodityCode` (ne, eq, ct, nct)
    /// - `createdDate` (gt, ge, lt, le, eq, ne)
    /// - `description` (ne, eq, ct, nct)
    /// - `externalPaypointID` (ct, nct, ne, eq)
    /// - `mode` (eq, ne)
    /// - `name` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `paypointId` (ne, eq)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `quantity` (gt, ge, lt, le, eq, ne)
    /// - `uom` (ne, eq, ct, nct)
    /// - `updatedDate` (gt, ge, lt, le, eq, ne)
    /// - `value` (gt, ge, lt, le, eq, ne)
    ///
    /// List of comparison accepted - enclosed between parentheses:
    ///
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
    /// Example: name(ct)=john return all records with name containing john
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_line_items(
        &self,
        entry: &String,
        request: &ListLineItemsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryResponseItems, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/lineitems/{}", entry),
                None,
                QueryBuilder::new()
                    .int("fromRecord", request.from_record.clone())
                    .int("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .string("sortBy", request.sort_by.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Updates an item.
    ///
    /// # Arguments
    ///
    /// * `line_item_id` - ID for the line item (also known as a product, service, or item).
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn update_item(
        &self,
        line_item_id: i64,
        request: &LineItem,
        options: Option<RequestOptions>,
    ) -> Result<PayabliApiResponse6, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("LineItem/{}", line_item_id),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }
}
