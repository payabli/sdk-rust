use crate::api::*;
use crate::{ApiError, ClientConfig, HttpClient, QueryBuilder, RequestOptions};
use reqwest::Method;

pub struct QueryClient {
    pub http_client: HttpClient,
}

impl QueryClient {
    pub fn new(config: ClientConfig) -> Result<Self, ApiError> {
        Ok(Self {
            http_client: HttpClient::new(config.clone())?,
        })
    }

    /// Retrieve a list of batches and their details, including settled and
    /// unsettled transactions for a paypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
    ///
    /// # Arguments
    ///
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
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
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.
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
    /// **List of comparison accepted:**
    /// - `eq` or empty => equal
    /// - `gt` => greater than
    /// - `ge` => greater or equal
    /// - `lt` => less than
    /// - `le` => less or equal
    /// - `ne` => not equal
    /// - `ct` => contains
    /// - `nct` => not contains
    /// - `in` => inside array separated by "|"
    /// - `nin` => not inside array separated by "|"
    ///
    /// **List of parameters accepted:**
    ///
    /// - `limitRecord`: max number of records for query (default="20", "0" or negative value for all)
    /// - `fromRecord`: initial record in query
    ///
    /// Example: `settledAmount(gt)=20` returns all records with a `settledAmount` greater than 20.00.
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_batch_details(
        &self,
        entry: &Entry,
        request: &ListBatchDetailsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryBatchesDetailResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/batchDetails/{}", entry.0),
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

    /// Retrieve a list of batches and their details, including settled and unsettled transactions for an organization. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
    ///
    /// # Arguments
    ///
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
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
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.
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
    /// **List of comparison accepted:**
    /// - `eq` or empty => equal
    /// - `gt` => greater than
    /// - `ge` => greater or equal
    /// - `lt` => less than
    /// - `le` => less or equal
    /// - `ne` => not equal
    /// - `ct` => contains
    /// - `nct` => not contains
    /// - `in` => inside array separated by "|"
    /// - `nin` => not inside array separated by "|"
    ///
    /// **List of parameters accepted:**
    ///
    /// - `limitRecord`: max number of records for query (default="20", "0" or negative value for all)
    /// - `fromRecord`: initial record in query
    ///
    /// Example: `settledAmount(gt)=20` returns all records with a `settledAmount` greater than 20.00.
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_batch_details_org(
        &self,
        org_id: i64,
        request: &ListBatchDetailsOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryResponseSettlements, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/batchDetails/org/{}", org_id),
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

    /// Retrieve a list of batches for a paypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
    ///
    /// # Arguments
    ///
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
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
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.
    ///
    /// **List of field names accepted:**
    ///
    /// - `batchDate` (gt, ge, lt, le, eq, ne)
    /// - `batchNumber` (ne, eq)
    /// - `method` (in, nin, eq, ne)
    /// - `connectorName` (ne, eq, ct, nct)
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
    /// - `depositDate` (gt, ge, lt, le, eq, ne)
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
    /// **List of parameters accepted:**
    /// - `limitRecord`: max number of records for query (default="20", "0" or negative value for all)
    /// - `fromRecord`: initial record in query
    ///
    /// Example: `batchAmount(gt)=20` returns all records with a `batchAmount` greater than 20.00
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_batches(
        &self,
        entry: &Entry,
        request: &ListBatchesQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryBatchesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/batches/{}", entry.0),
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

    /// Retrieve a list of batches for an org. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
    ///
    /// # Arguments
    ///
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
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
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.
    ///
    /// **List of field names accepted:**
    ///
    /// - `batchDate` (gt, ge, lt, le, eq, ne)
    /// - `batchNumber` (ne, eq)
    /// - `method` (in, nin, eq, ne)
    /// - `connectorName` (ne, eq, ct, nct)
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
    /// - `depositDate` (gt, ge, lt, le, eq, ne)
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
    /// **List of parameters accepted:**
    /// - `limitRecord`: max number of records for query (default="20", "0" or negative value for all)
    /// - `fromRecord`: initial record in query
    ///
    /// Example: `batchAmount(gt)=20` returns all records with a `batchAmount` greater than 20.00
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_batches_org(
        &self,
        org_id: i64,
        request: &ListBatchesOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryBatchesResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/batches/org/{}", org_id),
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

    /// Retrieve a list of MoneyOut batches for a paypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
    ///
    /// # Arguments
    ///
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query. See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.
    ///
    /// **List of field names accepted**:
    ///
    /// - `batchDate` (gt, ge, lt, le, eq, ne)
    /// - `batchNumber` (ne, eq)
    /// - `batchAmount` (gt, ge, lt, le, eq, ne)
    /// - `parentOrgId` (ne, eq, nin, in)
    /// - `status` (in, nin, eq, ne)
    /// - `orgId` (eq)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    /// - `paypointId` (ne, eq)
    /// - `externalPaypointID` (ct, nct, eq, ne)
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_batches_out(
        &self,
        entry: &Entry,
        request: &ListBatchesOutQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryBatchesOutResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/batchesOut/{}", entry.0),
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

    /// Retrieve a list of MoneyOut batches for an org. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
    ///
    /// # Arguments
    ///
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
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
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.
    ///
    /// **List of field names accepted**:
    ///
    /// - `batchDate` (gt, ge, lt, le, eq, ne)
    /// - `batchNumber` (ne, eq)
    /// - `batchAmount` (gt, ge, lt, le, eq, ne)
    /// - `parentOrgId` (ne, eq, nin, in)
    /// - `status` (in, nin, eq, ne)
    /// - `orgId` (eq)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    /// - `paypointId` (ne, eq)
    /// - `externalPaypointID` (ct, nct, eq, ne)
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_batches_out_org(
        &self,
        org_id: i64,
        request: &ListBatchesOutOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryBatchesOutResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/batchesOut/org/{}", org_id),
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

    /// Retrieves a list of chargebacks and returned transactions for a paypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
    ///
    /// # Arguments
    ///
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
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
    /// **List of field names accepted:**
    /// - `chargebackDate` (gt, ge, lt, le, eq, ne)
    /// - `transId`  (ne, eq, ct, nct)
    /// - `method`   (in, nin, eq, ne)
    /// - `netAmount`  (gt, ge, lt, le, eq, ne)
    /// - `reasonCode`   (in, nin, eq, ne)
    /// - `reason`  (ct, nct, eq, ne)
    /// - `replyDate` (gt, ge, lt, le, eq, ne)
    /// - `caseNumber`  (ct, nct, eq, ne)
    /// - `status`   (in, nin, eq, ne)
    /// - `accountType`   (in, nin, eq, ne)
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
    /// - `paypointId`  (ne, eq)
    /// - `paypointLegal`  (ne, eq, ct, nct)
    /// - `paypointDba`  (ne, eq, ct, nct)
    /// - `orgName`  (ne, eq, ct, nct)
    /// - `additional-xxx`  (ne, eq, ct, nct) where xxx is the additional field name
    ///
    /// **List of comparison accepted - enclosed between parentheses:**
    /// - `eq` or empty => equal
    /// - `gt` => greater than
    /// - `ge` => greater or equal
    /// - `lt` => less than
    /// - `le` => less or equal
    /// - `ne` => not equal
    /// - `ct` => contains
    /// - `nct` => not contains
    /// - `in` => inside array separated by "|"
    /// - `nin` => not inside array separated by "|"
    ///
    /// **List of parameters accepted:**
    /// - `limitRecord`: max number of records for query (default="20", "0" or negative value for all)
    /// - `fromRecord`: initial record in query
    ///
    /// Example: `netAmount(gt)=20` returns all records with a `netAmount` greater than 20.00
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_chargebacks(
        &self,
        entry: &Entry,
        request: &ListChargebacksQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryChargebacksResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/chargebacks/{}", entry.0),
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

    /// Retrieve a list of chargebacks and returned transactions for an org. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
    ///
    /// # Arguments
    ///
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
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
    /// </Info> See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// **List of field names accepted:**
    ///
    /// - `chargebackDate` (gt, ge, lt, le, eq, ne)
    /// - `transId`  (ne, eq, ct, nct)
    /// - `method`   (in, nin, eq, ne)
    /// - `netAmount`  (gt, ge, lt, le, eq, ne)
    /// - `reasonCode`   (in, nin, eq, ne)
    /// - `reason`  (ct, nct, eq, ne)
    /// - `replyDate` (gt, ge, lt, le, eq, ne)
    /// - `caseNumber`  (ct, nct, eq, ne)
    /// - `status`   (in, nin, eq, ne)
    /// - `accountType`   (in, nin, eq, ne)
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
    /// - `paypointId`  (ne, eq)
    /// - `paypointLegal`  (ne, eq, ct, nct)
    /// - `paypointDba`  (ne, eq, ct, nct)
    /// - `orgName`  (ne, eq, ct, nct)
    /// - `additional-xxx`  (ne, eq, ct, nct) where xxx is the additional field name
    ///
    /// **List of comparison accepted - enclosed between parentheses:**
    ///
    /// - `eq` or empty => equal
    /// - `gt` => greater than
    /// - `ge` => greater or equal
    /// - `lt` => less than
    /// - `le` => less or equal
    /// - `ne` => not equal
    /// - `ct` => contains
    /// - `nct` => not contains
    /// - `in` => inside array separated by "|"
    /// - `nin` => not inside array separated by "|"
    ///
    /// **List of parameters accepted:**
    /// - `limitRecord`: max number of records for query (default="20", "0" or negative value for all)
    /// - `fromRecord`: initial record in query
    ///
    /// Example: `netAmount(gt)=20` returns all records with a `netAmount` greater than 20.00
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_chargebacks_org(
        &self,
        org_id: i64,
        request: &ListChargebacksOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryChargebacksResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/chargebacks/org/{}", org_id),
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

    /// Retrieves a list of customers for a paypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
    ///
    /// # Arguments
    ///
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
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
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more details.
    ///
    /// **List of Accepted Field Names:**
    ///
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
    /// **List of Accepted Comparisons:**
    ///
    /// - `eq` or empty => equal
    /// - `gt` => greater than
    /// - `ge` => greater or equal
    /// - `lt` => less than
    /// - `le` => less or equal
    /// - `ne` => not equal
    /// - `ct` => contains
    /// - `nct` => not contains
    /// - `in` => inside array separated by "|"
    /// - `nin` => not inside array separated by "|"
    ///
    /// **Accepted Parameters:**
    /// - `limitRecord`: Max number of records for query (default="20", "0" or negative value for all)
    /// - `fromRecord`: Initial record in query
    ///
    /// **Example Usage:**
    /// `balance(gt)=20` will return all records with a balance greater than 20.00.
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_customers(
        &self,
        entry: &Entry,
        request: &ListCustomersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryCustomerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/customers/{}", entry.0),
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

    /// Retrieves a list of customers for an org. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
    ///
    /// # Arguments
    ///
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
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
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more details.
    ///
    /// **List of Accepted Field Names:**
    ///
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
    /// **List of Accepted Comparisons:**
    ///
    /// - `eq` or empty => equal
    /// - `gt` => greater than
    /// - `ge` => greater or equal
    /// - `lt` => less than
    /// - `le` => less or equal
    /// - `ne` => not equal
    /// - `ct` => contains
    /// - `nct` => not contains
    /// - `in` => inside array separated by "|"
    /// - `nin` => not inside array separated by "|"
    ///
    /// **Accepted Parameters:**
    /// - `limitRecord`: Max number of records for query (default="20", "0" or negative value for all)
    /// - `fromRecord`: Initial record in query
    ///
    /// **Example Usage:**
    /// `balance(gt)=20` will return all records with a balance greater than 20.00.
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_customers_org(
        &self,
        org_id: i64,
        request: &ListCustomersOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryCustomerResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/customers/org/{}", org_id),
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

    /// Returns a list of all reports generated in the last 60 days for a single entrypoint. Use filters to limit results.
    ///
    /// # Arguments
    ///
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
    /// </Info>
    ///
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    /// - `reportName` (ct, nct, eq, ne)
    /// - `createdAt` (gt, ge, lt, le, eq, ne)
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
    /// Example: reportName(ct)=tr  return all records containing the string "tr"
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_notification_reports(
        &self,
        entry: &Entry,
        request: &ListNotificationReportsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryResponseNotificationReports, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/notificationReports/{}", entry.0),
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

    /// Returns a list of all reports generated in the last 60 days for an organization. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query <Info>
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
    /// - `reportName` (ct, nct, eq, ne)
    /// - `createdAt` (gt, ge, lt, le, eq, ne)
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
    /// Example: reportName(ct)=tr  return all records containing the string "tr"
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_notification_reports_org(
        &self,
        org_id: i64,
        request: &ListNotificationReportsOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryResponseNotificationReports, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/notificationReports/org/{}", org_id),
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

    /// Returns a list of notifications for an entrypoint. Use filters to limit results.
    ///
    /// # Arguments
    ///
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
    /// </Info>
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    /// - `frequency` (in, nin,ne, eq)
    /// - `method` (in, nin, eq, ne)
    /// - `event` (in, nin, eq, ne)
    /// - `target` (ct, nct, eq, ne)
    /// - `status` (eq, ne)
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
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_notifications(
        &self,
        entry: &Entry,
        request: &ListNotificationsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryResponseNotifications, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/notifications/{}", entry.0),
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

    /// Return a list of notifications for an organization. Use filters to limit results.
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
    /// - `frequency` (in, nin,ne, eq)
    /// - `method` (in, nin, eq, ne)
    /// - `event` (in, nin, eq, ne)
    /// - `target` (ct, nct, eq, ne)
    /// - `status` (eq, ne)
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
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_notifications_org(
        &self,
        org_id: i64,
        request: &ListNotificationsOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryResponseNotifications, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/notifications/org/{}", org_id),
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

    /// Retrieves a list of an organization's suborganizations and their full details such as orgId, users, and settings. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
    ///
    /// # Arguments
    ///
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
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
    /// **List of field names accepted:**
    ///
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
    /// - `orgParentname`  (ct, nct)
    /// - `boardingId` (eq, ne)
    /// - `entryName`  (ct, nct)
    ///
    /// **List of comparison accepted - enclosed between parentheses:**
    ///
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
    /// **List of parameters accepted:**
    ///
    /// - `limitRecord` : max number of records for query (default="20", "0" or negative value for all)
    /// - `fromRecord` : initial record in query
    ///
    /// Example: `dbaname(ct)=hoa` returns all records with a `dbaname` containing "hoa"
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_organizations(
        &self,
        org_id: i64,
        request: &ListOrganizationsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<ListOrganizationsResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/organizations/{}", org_id),
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

    /// Retrieves a list of money out transactions (payouts) for a paypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
    ///
    /// # Arguments
    ///
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
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
    /// List of field names accepted:
    ///
    /// - `status` (in, nin, eq, ne)
    /// - `transactionDate` (gt, ge, lt, le, eq, ne)
    /// - `billNumber` (ct, nct)
    /// - `vendorNumber` (ct, nct, eq, ne)
    /// - `vendorName` (ct, nct, eq, ne)
    /// - `paymentMethod` (ct, nct, eq, ne, in, nin)
    /// - `paymentId` (ct, nct, eq, ne)
    /// - `parentOrgId` (ne, eq, nin, in)
    /// - `batchNumber` (ct, nct, eq, ne)
    /// - `totalAmount` (gt, ge, lt, le, eq, ne)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `accountId` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    /// - `externalPaypointID` (ct, nct, eq, ne)
    /// - `paypointId` (eq, ne)
    /// - `vendorId` (eq, ne)
    /// - `vendorEIN` (ct, nct, eq, ne)
    /// - `vendorPhone` (ct, nct, eq, ne)
    /// - `vendorEmail` (ct, nct, eq, ne)
    /// - `vendorAddress` (ct, nct, eq, ne)
    /// - `vendorCity` (ct, nct, eq, ne)
    /// - `vendorState` (ct, nct, eq, ne)
    /// - `vendorCountry` (ct, nct, eq, ne)
    /// - `vendorZip` (ct, nct, eq, ne)
    /// - `vendorMCC` (ct, nct, eq, ne)
    /// - `vendorLocationCode` (ct, nct, eq, ne)
    /// - `vendorCustomField1` (ct, nct, eq, ne)
    /// - `vendorCustomField2` (ct, nct, eq, ne)
    /// - `comments` (ct, nct)
    /// - `payaccountCurrency` (ne, eq, in, nin)
    /// - `remitAddress` (ct, nct)
    /// - `source` (ct, nct, eq, ne)
    /// - `updatedOn` (gt, ge, lt, le, eq, ne)
    /// - `feeAmount` (gt, ge, lt, le, eq, ne)
    /// - `lotNumber` (ct, nct)
    /// - `customerVendorAccount` (ct, nct, eq, ne)
    /// - `batchId` (eq, ne)
    /// - `AchTraceNumber` (eq, ne)
    /// - `payoutProgram`(eq, ne) the options are `managed` or `odp`. For example, `payoutProgram(eq)=managed` returns all records with a `payoutProgram` equal to `managed`.
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
    /// - in => inside array separated by \"|\"
    /// - nin => not inside array separated by \"|\"
    ///
    /// List of parameters accepted:
    ///
    /// - limitRecord : max number of records for query (default=\"20\", \"0\" or negative value for all)
    /// - fromRecord : initial record in query
    /// - sortBy : indicate field name and direction to sort the results
    ///
    /// Example: `netAmount(gt)=20` returns all records with a `netAmount` greater than 20.00
    ///
    /// Example: `sortBy=desc(netamount)` returns all records sorted by `netAmount` descending
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_payout(
        &self,
        entry: &Entry,
        request: &ListPayoutQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryPayoutTransaction, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/payouts/{}", entry.0),
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

    /// Retrieves a list of money out transactions (payouts) for an organization. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
    ///
    /// # Arguments
    ///
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
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
    /// List of field names accepted:
    ///
    /// - `status` (in, nin, eq, ne)
    /// - `transactionDate` (gt, ge, lt, le, eq, ne)
    /// - `billNumber` (ct, nct)
    /// - `vendorNumber` (ct, nct, eq, ne)
    /// - `vendorName` (ct, nct, eq, ne)
    /// - `parentOrgId` (ne, eq, nin, in)
    /// - `paymentMethod` (ct, nct, eq, ne, in, nin)
    /// - `paymentId` (ct, nct, eq, ne)
    /// - `batchNumber` (ct, nct, eq, ne)
    /// - `totalAmount` (gt, ge, lt, le, eq, ne)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `accountId` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    /// - `externalPaypointID` (ct, nct, eq, ne)
    /// - `paypointId` (eq, ne)
    /// - `vendorId` (eq, ne)
    /// - `vendorEIN` (ct, nct, eq, ne)
    /// - `vendorPhone` (ct, nct, eq, ne)
    /// - `vendorEmail` (ct, nct, eq, ne)
    /// - `vendorAddress` (ct, nct, eq, ne)
    /// - `vendorCity` (ct, nct, eq, ne)
    /// - `vendorState` (ct, nct, eq, ne)
    /// - `vendorCountry` (ct, nct, eq, ne)
    /// - `vendorZip` (ct, nct, eq, ne)
    /// - `vendorMCC` (ct, nct, eq, ne)
    /// - `vendorLocationCode` (ct, nct, eq, ne)
    /// - `vendorCustomField1` (ct, nct, eq, ne)
    /// - `vendorCustomField2` (ct, nct, eq, ne)
    /// - `comments` (ct, nct)
    /// - `payaccountCurrency` (ne, eq, in, nin)
    /// - `remitAddress` (ct, nct)
    /// - `source` (ct, nct, eq, ne)
    /// - `updatedOn` (gt, ge, lt, le, eq, ne)
    /// - `feeAmount` (gt, ge, lt, le, eq, ne)
    /// - `lotNumber` (ct, nct)
    /// - `customerVendorAccount` (ct, nct, eq, ne)
    /// - `batchId` (eq, ne)
    /// - `AchTraceNumber` (eq, ne)
    /// - `payoutProgram`(eq, ne) the options are `managed` or `odp`. For example, `payoutProgram(eq)=managed` returns all records with a `payoutProgram` equal to `managed`.
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
    /// - in => inside array separated by \"|\"
    /// - nin => not inside array separated by \"|\"
    ///
    /// List of parameters accepted:
    ///
    /// - limitRecord : max number of records for query (default=\"20\", \"0\" or negative value for all)
    /// - fromRecord : initial record in query
    /// - sortBy : indicate field name and direction to sort the results
    ///
    /// Example: `netAmount(gt)=20` returns all records with a `netAmount` greater than 20.00
    ///
    /// Example: `sortBy=desc(netamount)` returns all records sorted by `netAmount` descending
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_payout_org(
        &self,
        org_id: i64,
        request: &ListPayoutOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryPayoutTransaction, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/payouts/org/{}", org_id),
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

    /// Returns a list of paypoints in an organization. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
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
    /// **List of field names accepted:**
    ///
    /// - `createdAt` (gt, ge, lt, le, eq, ne)
    /// - `lastModified` (gt, ge, lt, le, eq, ne)
    /// - `startDate` (gt, ge, lt, le, eq, ne)
    /// - `dbaname`  (ct, nct)
    /// - `status` (eq, ne)
    /// - `legalname`  (ct, nct)
    /// - `externalPaypointID` (ct, nct)
    /// - `ein`  (ct, nct)
    /// - `address`  (ct, nct)
    /// - `city`  (ct, nct)
    /// - `state`  (ct, nct)
    /// - `phone`  (ct, nct)
    /// - `mcc`  (ct, nct)
    /// - `owntype`  (ct, nct)
    /// - `ownerName`  (ct, nct)
    /// - `contactName`  (ct, nct)
    /// - `paypointId` (eq, ne)
    /// - `orgParentname`  (ct, nct, in, nin)
    /// - `boardingId` (eq, ne)
    /// - `entryName`  (ct, nct)
    /// - `externalOrgID` (ct, nct)
    ///
    /// **List of comparison accepted - enclosed between parentheses:**
    ///
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
    /// **List of parameters accepted:**
    ///
    /// - `limitRecord` : max number of records for query (default="20", "0" or negative value for all)
    /// - `fromRecord` : initial record in query
    ///
    /// Example: `dbaname(ct)=hoa` returns all records with a `dbaname` containing "hoa"
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_paypoints(
        &self,
        org_id: i64,
        request: &ListPaypointsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryEntrypointResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/paypoints/{}", org_id),
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

    /// Retrieve a list of settled transactions for a paypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
    ///
    /// # Arguments
    ///
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
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
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.
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
    /// **List of comparison accepted:**
    /// - `eq` or empty => equal
    /// - `gt` => greater than
    /// - `ge` => greater or equal
    /// - `lt` => less than
    /// - `le` => less or equal
    /// - `ne` => not equal
    /// - `ct` => contains
    /// - `nct` => not contains
    /// - `in` => inside array separated by "|"
    /// - `nin` => not inside array separated by "|"
    ///
    /// **List of parameters accepted:**
    ///
    /// - `limitRecord`: max number of records for query (default="20", "0" or negative value for all)
    /// - `fromRecord`: initial record in query
    ///
    /// Example: `settledAmount(gt)=20` returns all records with a `settledAmount` greater than 20.00.
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_settlements(
        &self,
        entry: &Entry,
        request: &ListSettlementsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryResponseSettlements, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/settlements/{}", entry.0),
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

    /// Retrieve a list of settled transactions for an organization. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
    ///
    /// # Arguments
    ///
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
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
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.
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
    /// **List of comparison accepted:**
    /// - `eq` or empty => equal
    /// - `gt` => greater than
    /// - `ge` => greater or equal
    /// - `lt` => less than
    /// - `le` => less or equal
    /// - `ne` => not equal
    /// - `ct` => contains
    /// - `nct` => not contains
    /// - `in` => inside array separated by "|"
    /// - `nin` => not inside array separated by "|"
    ///
    /// **List of parameters accepted:**
    ///
    /// - `limitRecord`: max number of records for query (default="20", "0" or negative value for all)
    /// - `fromRecord`: initial record in query
    ///
    /// Example: `settledAmount(gt)=20` returns all records with a `settledAmount` greater than 20.00.
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_settlements_org(
        &self,
        org_id: i64,
        request: &ListSettlementsOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryResponseSettlements, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/settlements/org/{}", org_id),
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

    /// Returns a list of subscriptions for a single paypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
    ///
    /// # Arguments
    ///
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
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
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.
    ///
    /// **List of field names accepted:**
    ///
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
    /// - `payaccountCurrency` (ne, eq, in, nin)
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
    /// - `externalPaypointId` (ct, nct, ne, eq)
    /// - `subId` (eq, ne)
    /// - `orderDescription` (ct, nct)
    /// - `cycles` (eq, ne, gt, ge, lt, le)
    /// - `leftcycles` (eq, ne, gt, ge, lt, le)
    /// - `createdAt` (eq, ne, gt, ge, lt, le)
    /// - `updatedOn` (eq, ne, gt, ge, lt, le)
    /// - `invoiceNumber` (ct, nct)
    /// - `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name
    ///
    /// **List of comparison operators accepted:**
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
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_subscriptions(
        &self,
        entry: &Entry,
        request: &ListSubscriptionsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QuerySubscriptionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/subscriptions/{}", entry.0),
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

    /// Returns a list of subscriptions for a single org. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
    ///
    /// # Arguments
    ///
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
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
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.
    ///
    /// **List of field names accepted:**
    ///
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
    /// - `payaccountCurrency` (ne, eq, in, nin)
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
    /// - `externalPaypointId` (ct, nct, ne, eq)
    /// - `subId` (eq, ne)
    /// - `orderDescription` (ct, nct)
    /// - `cycles` (eq, ne, gt, ge, lt, le)
    /// - `leftcycles` (eq, ne, gt, ge, lt, le)
    /// - `createdAt` (eq, ne, gt, ge, lt, le)
    /// - `updatedOn` (eq, ne, gt, ge, lt, le)
    /// - `invoiceNumber` (ct, nct)
    /// - `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name
    ///
    /// **List of comparison operators accepted:**
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
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_subscriptions_org(
        &self,
        org_id: i64,
        request: &ListSubscriptionsOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QuerySubscriptionResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/subscriptions/org/{}", org_id),
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

    /// Retrieve a list of transactions for a paypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
    /// By default, this endpoint returns only transactions from the last 60 days. To query transactions outside of this period, include `transactionDate` filters.
    /// For example, this request parameters filter for transactions between April 01, 2024 and April 09, 2024.
    /// ``` curl --request GET \
    /// --url https://sandbox.payabli.com/api/Query/transactions/org/1?limitRecord=20&fromRecord=0&transactionDate(ge)=2024-04-01T00:00:00&transactionDate(le)=2024-04-09T23:59:59\
    /// --header 'requestToken: <api-key>'
    ///
    /// ```
    ///
    /// # Arguments
    ///
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
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
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.
    ///
    /// **List of field names accepted:**
    ///
    /// - `transactionDate` (gt, ge, lt, le, eq, ne)
    /// - `transId` (ne, eq, ct, nct, in, nin)
    /// - `gatewayTransId` (ne, eq, ct, nct)
    /// - `orderId` (ne, eq)
    /// - `scheduleId` (ne, eq)
    /// - `returnId` (ne, eq)
    /// - `refundId` (ne, eq)
    /// - `idTrans` (ne, eq)
    /// - `orgId` (ne, eq)
    /// - `paypointId` (ne, eq)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    /// - `externalPaypointId` (ct, nct, eq, ne)
    /// - `method` (in, nin, eq, ne)
    /// - `totalAmount` (gt, ge, lt, le, eq, ne)
    /// - `netAmount` (gt, ge, lt, le, eq, ne)
    /// - `feeAmount` (gt, ge, lt, le, eq, ne)
    /// - `operation` (in, nin, eq, ne)
    /// - `source` (in, nin, eq, ne, ct, nct)
    /// - `status` (in, nin, eq, ne)
    /// - `settlementStatus` (in, nin, eq, ne)
    /// - `batchNumber` (nct, ct)
    /// - `invoiceNumber` (ct, nct)
    /// - `ipAddress` (eq, ne)
    /// - `authCode` (ct, nct)
    /// - `orderDescription` (ct, nct)
    /// - `payaccountLastfour` (nct, ct)
    /// - `payaccountType` (ne, eq, in, nin)
    /// - `payaccountCurrency` (ne, eq, in, nin)
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
    /// - `deviceId` (ct, nct, in, nin, eq, ne)
    /// - `AchSecCode` ( ct, nct, in, nin, eq, ne)
    /// - `AchHolderType` (ct, nct, in, nin, eq, and ne)
    /// - `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name related to customer data
    /// - 'invoiceAdditional-xxx' (ne, eq, ct, nct) where xxx is the additional field name related to invoice data
    ///
    /// **List of comparison operators accepted:**
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
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_transactions(
        &self,
        entry: &Entry,
        request: &ListTransactionsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryResponseTransactions, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/transactions/{}", entry.0),
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

    /// Retrieve a list of transactions for an organization. Use filters to
    /// limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
    ///
    ///
    /// By default, this endpoint returns only transactions from the last 60 days. To query transactions outside of this period, include `transactionDate` filters.
    ///
    /// For example, this request parameters filter for transactions between April 01, 2024 and April 09, 2024.
    ///
    /// ```
    /// curl --request GET \
    /// --url https://sandbox.payabli.com/api/Query/transactions/org/1?limitRecord=20&fromRecord=0&transactionDate(ge)=2024-04-01T00:00:00&transactionDate(le)=2024-04-09T23:59:59\
    /// --header 'requestToken: <api-key>'
    ///
    /// ```
    ///
    /// # Arguments
    ///
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
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
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.
    ///
    /// **List of field names accepted:**
    ///
    /// - `transactionDate` (gt, ge, lt, le, eq, ne)
    /// - `transId` (ne, eq, ct, nct, in, nin)
    /// - `gatewayTransId` (ne, eq, ct, nct)
    /// - `orderId` (ne, eq)
    /// - `scheduleId` (ne, eq)
    /// - `returnId` (ne, eq)
    /// - `refundId` (ne, eq)
    /// - `idTrans` (ne, eq)
    /// - `orgId` (ne, eq)
    /// - `paypointId` (ne, eq)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    /// - `externalPaypointId` (ct, nct, eq, ne)
    /// - `method` (in, nin, eq, ne)
    /// - `totalAmount` (gt, ge, lt, le, eq, ne)
    /// - `netAmount` (gt, ge, lt, le, eq, ne)
    /// - `feeAmount` (gt, ge, lt, le, eq, ne)
    /// - `operation` (in, nin, eq, ne)
    /// - `source` (in, nin, eq, ne, ct, nct)
    /// - `status` (in, nin, eq, ne)
    /// - `settlementStatus` (in, nin, eq, ne)
    /// - `batchNumber` (nct, ct)
    /// - `invoiceNumber` (ct, nct)
    /// - `authCode` (ct, nct)
    /// - `orderDescription` (ct, nct)
    /// - `payaccountLastfour` (nct, ct)
    /// - `payaccountType` (ne, eq, in, nin)
    /// - `payaccountCurrency` (ne, eq, in, nin)
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
    /// - `deviceId` (ct, nct, in, nin, eq, ne)
    /// - `AchSecCode` ( ct, nct, in, nin, eq, ne)
    /// - `AchHolderType`` (ct, nct, in, nin, eq, and ne)
    /// - `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name related to customer data
    /// - 'invoiceAdditional-xxx' (ne, eq, ct, nct) where xxx is the additional field name related to invoice data
    ///
    /// **List of comparison operators accepted:**
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
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_transactions_org(
        &self,
        org_id: i64,
        request: &ListTransactionsOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryResponseTransactions, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/transactions/org/{}", org_id),
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

    /// Retrieve a list of transfer details records for a paypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
    ///
    /// # Arguments
    ///
    /// * `transfer_id` - The numeric identifier for the transfer, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `parameters` - Collection of field names, conditions, and values used to filter
    /// the query.
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
    /// See [Filters and Conditions
    /// Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference)
    /// for more information.
    ///
    ///
    /// **List of field names accepted:**
    ///
    /// - `grossAmount` (gt, ge, lt, le, eq, ne)
    /// - `chargeBackAmount` (gt, ge, lt, le, eq, ne)
    /// - `returnedAmount` (gt, ge, lt, le, eq, ne)
    /// - `billingFeeAmount` (gt, ge, lt, le, eq, ne)
    /// - `thirdPartyPaidAmount` (gt, ge, lt, le, eq, ne)
    /// - `netFundedAmount` (gt, ge, lt, le, eq, ne)
    /// - `adjustmentAmount` (gt, ge, lt, le, eq, ne)
    /// - `splitFundingAmount` (gt, ge, lt, le, eq, ne)
    /// - `operation` (in, nin, eq, ne)
    /// - `transactionId` (eq, ne, in, nin)
    /// - `category` (eq, ne, ct, nct)
    /// - `type` (eq, ne, in, nin)
    /// - `method` (eq, ne, in, nin)
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_transfer_details(
        &self,
        entry: &Entry,
        transfer_id: i64,
        request: &ListTransferDetailsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryTransferDetailResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/transferDetails/{}/{}", entry.0, transfer_id),
                None,
                QueryBuilder::new()
                    .serialize("exportFormat", request.export_format.clone())
                    .int("fromRecord", request.from_record.clone())
                    .serialize("limitRecord", request.limit_record.clone())
                    .serialize("parameters", request.parameters.clone())
                    .string("sortBy", request.sort_by.clone())
                    .build(),
                options,
            )
            .await
    }

    /// Retrieve a list of transfers for a paypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
    ///
    /// # Arguments
    ///
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query. See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.
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
    /// List of field names accepted:
    ///
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
    /// - `batchNumber` (ne, eq, ct, nct)
    /// - `batchId` (ne, eq, in, nin)
    /// - `transferId` (in, nin, eq, ne)
    /// - `bankAccountNumber` (ct, nct, ne, eq)
    /// - `bankRoutingNumber` (ct, nct, ne, eq)
    /// - `batchCurrency` (in, nin, ne, eq)
    /// - `parentOrgName` (ct, nct, ne, eq)
    /// - `parentOrgId` (ct, nct, ne, eq)
    /// - `externalPaypointID` (ct, nct)
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_transfers(
        &self,
        entry: &Entry,
        request: &ListTransfersQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TransferQueryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/transfers/{}", entry.0),
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

    /// Retrieve a list of transfers for an org. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
    ///
    /// # Arguments
    ///
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query. See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.
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
    /// List of field names accepted:
    ///
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
    /// - `batchNumber` (ne, eq, ct, nct)
    /// - `batchId` (ne, eq, in, nin)
    /// - `transferId` (in, nin, eq, ne)
    /// - `bankAccountNumber` (ct, nct, ne, eq)
    /// - `bankRoutingNumber` (ct, nct, ne, eq)
    /// - `batchCurrency` (in, nin, ne, eq)
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_transfers_org(
        &self,
        org_id: &Orgid,
        request: &ListTransfersOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<TransferQueryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/transfers/org/{}", org_id.0),
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

    /// Get list of users for an org. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
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
    /// **List of field names accepted:**
    ///
    /// - `createdDate` (gt, ge, lt, le, eq, ne)
    /// - `name`  (ne, eq, ct, nct)
    /// - `email`  (ne, eq, ct, nct)
    /// - `status`   (in, nin, eq, ne)
    /// - `role.xxx`  (ne, eq, ct, nct) where xxx is the role field: `roleLabel` or `roleValue`
    ///
    /// **List of comparison accepted - enclosed between parentheses:**
    ///
    /// - `eq` or empty => equal
    /// - `gt` => greater than
    /// - `ge` => greater or equal
    /// - `lt` => less than
    /// - `le` => less or equal
    /// - `ne` => not equal
    /// - `ct` => contains
    /// - `nct` => not contains
    /// - `in` => inside array separated by "|"
    /// - `nin` => not inside array separated by "|"
    ///
    /// **List of parameters accepted:**
    /// - `limitRecord`: max number of records for query (default="20", "0" or negative value for all)
    /// - `fromRecord`: initial record in query
    ///
    /// Example: `name(ct)=john`  return all records with name containing 'john'.
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_users_org(
        &self,
        org_id: i64,
        request: &ListUsersOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryUserResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/users/org/{}", org_id),
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

    /// Get list of users for a paypoint. Use filters to limit results.
    ///
    /// # Arguments
    ///
    /// * `entry` - The paypoint's entrypoint identifier. [Learn more](/api-reference/api-overview#entrypoint-vs-entry)
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
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
    /// **List of field names accepted:**
    ///
    /// - `createdDate` (gt, ge, lt, le, eq, ne)
    /// - `name`  (ne, eq, ct, nct)
    /// - `email`  (ne, eq, ct, nct)
    /// - `status`   (in, nin, eq, ne)
    /// - `role.xxx`  (ne, eq, ct, nct) where xxx is the role field: `roleLabel` or `roleValue`
    ///
    /// **List of comparison accepted - enclosed between parentheses:**
    ///
    /// - `eq` or empty => equal
    /// - `gt` => greater than
    /// - `ge` => greater or equal
    /// - `lt` => less than
    /// - `le` => less or equal
    /// - `ne` => not equal
    /// - `ct` => contains
    /// - `nct` => not contains
    /// - `in` => inside array separated by "|"
    /// - `nin` => not inside array separated by "|"
    ///
    /// **List of parameters accepted:**
    /// - `limitRecord`: max number of records for query (default="20", "0" or negative value for all)
    /// - `fromRecord`: initial record in query
    ///
    /// Example: `name(ct)=john`  return all records with name containing 'john'
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_users_paypoint(
        &self,
        entry: &String,
        request: &ListUsersPaypointQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryUserResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/users/point/{}", entry),
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

    /// Retrieve a list of vendors for an entrypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
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
    /// </Info>
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for help.
    ///
    /// List of field names accepted:
    /// - `method` (in, nin, eq, ne)
    /// - `enrollmentStatus` (in,nin, eq, ne)
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
    /// - `parentOrgId` (ne, eq, nin, in)
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
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_vendors(
        &self,
        entry: &String,
        request: &ListVendorsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryResponseVendors, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/vendors/{}", entry),
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

    /// Retrieve a list of vendors for an organization. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
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
    /// - `method` (in, nin, eq, ne)
    /// - `enrollmentStatus` (in,nin, eq, ne)
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
    /// - `parentOrgId` (ne, eq, nin, in)
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
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_vendors_org(
        &self,
        org_id: i64,
        request: &ListVendorsOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<QueryResponseVendors, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/vendors/org/{}", org_id),
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

    /// Retrieve a list of vcards (virtual credit cards) issued for an entrypoint. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
    ///
    /// # Arguments
    ///
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
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
    /// List of field names accepted:
    ///
    /// - `status` (in, nin, eq, ne)
    /// - `createdAt` (gt, ge, lt, le, eq, ne)
    /// - `cardToken` (ct, nct, eq, ne)
    /// - `lastFour` (ct, nct, eq, ne)
    /// - `expirationDate` (ct, nct, eq, ne)
    /// - `payoutId` (ct, nct, eq, ne, in, nin)
    /// - `vendorId` (ct, nct, eq, ne, in, nin)
    /// - `miscData1` (ct, nct, eq, ne)
    /// - `miscData2` (ct, nct, eq, ne)
    /// - `currentUses` (gt, ge, lt, le, eq, ne)
    /// - `amount` (gt, ge, lt, le, eq, ne)
    /// - `balance` (gt, ge, lt, le, eq, ne)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    /// - `externalPaypointId` (ct, nct, eq, ne)
    /// - `paypointId` (in, nin, eq, ne)
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
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_vcards(
        &self,
        entry: &Entry,
        request: &ListVcardsQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<VCardQueryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/vcards/{}", entry.0),
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

    /// Retrieve a list of vcards (virtual credit cards) issued for an organization. Use filters to limit results. Include the `exportFormat` query parameter to return the results as a file instead of a JSON response.
    ///
    /// # Arguments
    ///
    /// * `org_id` - The numeric identifier for organization, assigned by Payabli.
    /// * `from_record` - The number of records to skip before starting to collect the result set.
    /// * `limit_record` - Max number of records to return for the query. Use `0` or negative value to return all records.
    /// * `parameters` - Collection of field names, conditions, and values used to filter the query.
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
    /// List of field names accepted:
    ///
    /// - `status` (in, nin, eq, ne)
    /// - `createdAt` (gt, ge, lt, le, eq, ne)
    /// - `cardToken` (ct, nct, eq, ne)
    /// - `lastFour` (ct, nct, eq, ne)
    /// - `expirationDate` (ct, nct, eq, ne)
    /// - `payoutId` (ct, nct, eq, ne, in, nin)
    /// - `vendorId` (ct, nct, eq, ne, in, nin)
    /// - `miscData1` (ct, nct, eq, ne)
    /// - `miscData2` (ct, nct, eq, ne)
    /// - `currentUses` (gt, ge, lt, le, eq, ne)
    /// - `amount` (gt, ge, lt, le, eq, ne)
    /// - `balance` (gt, ge, lt, le, eq, ne)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    /// - `externalPaypointId` (ct, nct, eq, ne)
    /// - `paypointId` (in, nin, eq, ne)
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
    /// * `sort_by` - The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    /// * `options` - Additional request options such as headers, timeout, etc.
    ///
    /// # Returns
    ///
    /// JSON response from the API
    pub async fn list_vcards_org(
        &self,
        org_id: i64,
        request: &ListVcardsOrgQueryRequest,
        options: Option<RequestOptions>,
    ) -> Result<VCardQueryResponse, ApiError> {
        self.http_client
            .execute_request(
                Method::GET,
                &format!("Query/vcards/org/{}", org_id),
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
