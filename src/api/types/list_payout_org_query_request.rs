pub use crate::prelude::*;

/// Query parameters for ListPayoutOrg
///
/// Request type for the ListPayoutOrgQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListPayoutOrgQueryRequest {
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, Option<String>>>,
    /// The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}
