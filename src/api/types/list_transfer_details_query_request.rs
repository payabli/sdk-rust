pub use crate::prelude::*;

/// Query parameters for ListTransferDetails
///
/// Request type for the ListTransferDetailsQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListTransferDetailsQueryRequest {
    #[serde(rename = "exportFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub export_format: Option<ExportFormat>,
    /// The number of records to skip before starting to collect the result set.
    #[serde(rename = "fromRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_record: Option<i64>,
    #[serde(rename = "limitRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_record: Option<LimitRecord>,
    /// Collection of field names, conditions, and values used to filter
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, Option<String>>>,
    /// The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}
