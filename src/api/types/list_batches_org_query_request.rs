pub use crate::prelude::*;

/// Query parameters for ListBatchesOrg
///
/// Request type for the ListBatchesOrgQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListBatchesOrgQueryRequest {
    #[serde(rename = "exportFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_format: Option<ExportFormat>,
    /// The number of records to skip before starting to collect the result set.
    #[serde(rename = "fromRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_record: Option<i64>,
    /// Max number of records to return for the query. Use `0` or negative value to return all records.
    #[serde(rename = "limitRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_record: Option<i64>,
    /// Collection of field names, conditions, and values used to filter the query.
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, Option<String>>>,
    /// The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}
