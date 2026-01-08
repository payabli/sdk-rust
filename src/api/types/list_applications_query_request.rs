pub use crate::prelude::*;

/// Query parameters for ListApplications
///
/// Request type for the ListApplicationsQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListApplicationsQueryRequest {
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
    /// - `status` (in, nin, eq,ne)
    /// - `orgParentname` (ct, nct)
    /// - `externalpaypointID` (ct, nct, eq, ne)
    /// - `repCode` (ct, nct, eq, ne)
    /// - `repName` (ct, nct, eq, ne)
    /// - `repOffice` (ct, nct, eq, ne)
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, Option<String>>>,
    /// The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}
