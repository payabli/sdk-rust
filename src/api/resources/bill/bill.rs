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
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn add_bill(
        &self,
        entry: &String,
        request: &BillOutData,
        options: Option<RequestOptions>,
    ) -> Result<BillResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("Bill/single/{}", entry),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
                options,
            )
            .await
    }

    /// Delete a file attached to a bill.
    ///
    /// # Arguments
    ///
    /// * `id_bill` - Payabli ID for the bill. Get this ID by querying `/api/Query/bills/` for the entrypoint or the organization.
    /// * `filename` - The filename in Payabli. Filename is `zipName` in response to a
    /// request to `/api/Invoice/{idInvoice}`. Here, the filename is
    /// `0_Bill.pdf`.
    ///
    /// ```json
    /// "DocumentsRef": {
    /// "zipfile": "inva_269.zip",
    /// "filelist": [
    /// {
    /// "originalName": "Bill.pdf",
    /// "zipName": "0_Bill.pdf",
    /// "descriptor": null
    /// }
    /// ]
    /// }
    /// ```
    /// * `return_object` - When `true`, the request returns the file content as a Base64-encoded string.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn delete_attached_from_bill(
        &self,
        id_bill: i64,
        filename: &String,
        request: &DeleteAttachedFromBillQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<BillResponse, ApiError> {
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
    pub async fn delete_bill(
        &self,
        id_bill: i64,
        options: Option<RequestOptions>,
    ) -> Result<BillResponse, ApiError> {
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
    pub async fn edit_bill(
        &self,
        id_bill: i64,
        request: &BillOutData,
        options: Option<RequestOptions>,
    ) -> Result<EditBillResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("Bill/{}", id_bill),
                Some(serde_json::to_value(request).unwrap_or_default()),
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
    /// * `filename` - The filename in Payabli. Filename is `zipName` in response to a request to `/api/Invoice/{idInvoice}`. Here, the filename is `0_Bill.pdf``.
    /// "DocumentsRef": {
    /// "zipfile": "inva_269.zip",
    /// "filelist": [
    /// {
    /// "originalName": "Bill.pdf",
    /// "zipName": "0_Bill.pdf",
    /// "descriptor": null
    /// }
    /// ]
    /// }
    /// * `return_object` - When `true`, the request returns the file content as a Base64-encoded string.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn get_attached_from_bill(
        &self,
        id_bill: i64,
        filename: &String,
        request: &GetAttachedFromBillQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<FileContent, ApiError> {
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
    pub async fn get_bill(
        &self,
        id_bill: i64,
        options: Option<RequestOptions>,
    ) -> Result<GetBillResponse, ApiError> {
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

    /// Retrieve a list of bills for an entrypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
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
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response isn't filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
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
    /// List of comparison accepted - enclosed between parentheses:
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
    /// List of parameters accepted:
    /// - `limitRecord` : max number of records for query (default="20", "0" or negative value for all)
    /// - `fromRecord` : initial record in query
    /// Example: `totalAmount(gt)=20` returns all records with a `totalAmount` that's greater than 20.00
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_bills(
        &self,
        entry: &String,
        request: &ListBillsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<BillQueryResponse, ApiError> {
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
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query
    /// <Info>
    /// **You must remove `parameters=` from the request before you send it, otherwise Payabli will ignore the filters.**
    ///
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response isn't filtered. Instead, copy the request, remove `parameters=` and run the request in a different client.
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
    /// - `frequency` (in, nin, ne, eq)
    /// - `method` (in, nin, eq, ne)
    /// - `event` (in, nin, eq, ne)
    /// - `target` (ct, nct, eq, ne)
    /// - `status` (eq, ne)
    /// - `parentOrgId` (ne, eq, nin, in)
    /// - `approvalUserId` (eq, ne)
    /// - `approvalUserEmail` (eq, ne)
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
    /// Example: totalAmount(gt)=20 return all records with totalAmount greater than 20.00
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_bills_org(
        &self,
        org_id: i64,
        request: &ListBillsOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<BillQueryResponse, ApiError> {
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
    pub async fn modify_approval_bill(
        &self,
        id_bill: i64,
        request: &Vec<String>,
        options: Option<RequestOptions>,
    ) -> Result<ModifyApprovalBillResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::PUT,
                &format!("Bill/approval/{}", id_bill),
                Some(serde_json::to_value(request).unwrap_or_default()),
                None,
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
    pub async fn send_to_approval_bill(
        &self,
        id_bill: i64,
        request: &SendToApprovalBillRequest,
        options: Option<RequestOptions>,
    ) -> Result<BillResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::POST,
                &format!("Bill/approval/{}", id_bill),
                Some(serde_json::to_value(&request.body).unwrap_or_default()),
                QueryBuilder::new()
                    .bool("autocreateUser", request.autocreate_user.clone())
                    .build(),
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
    pub async fn set_approved_bill(
        &self,
        id_bill: i64,
        approved: &String,
        request: &SetApprovedBillQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<SetApprovedBillResponse, ApiError> {
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
}
