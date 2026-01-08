pub use crate::prelude::*;

/// Query parameters for ListBills
///
/// Request type for the ListBillsQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListBillsQueryRequest {
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
    /// Collection of field names, conditions, and values used to filter the query
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, Option<String>>>,
    /// The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}
