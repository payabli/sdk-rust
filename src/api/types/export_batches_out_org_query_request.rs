pub use crate::prelude::*;

/// Query parameters for ExportBatchesOutOrg
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ExportBatchesOutOrgQueryRequest {
    #[serde(rename = "columnsExport")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub columns_export: Option<String>,
    /// The number of records to skip before starting to collect the result set.
    #[serde(rename = "fromRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_record: Option<i64>,
    /// The number of records to return for the query. The maximum is 30,000 records. When this parameter isn't sent, the API returns up to 25,000 records.
    #[serde(rename = "limitRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_record: Option<i64>,
    /// Collection of field names, conditions, and values used to filter the query
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, Option<String>>>,
}

impl ExportBatchesOutOrgQueryRequest {
    pub fn builder() -> ExportBatchesOutOrgQueryRequestBuilder {
        <ExportBatchesOutOrgQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExportBatchesOutOrgQueryRequestBuilder {
    columns_export: Option<String>,
    from_record: Option<i64>,
    limit_record: Option<i64>,
    parameters: Option<HashMap<String, Option<String>>>,
}

impl ExportBatchesOutOrgQueryRequestBuilder {
    pub fn columns_export(mut self, value: impl Into<String>) -> Self {
        self.columns_export = Some(value.into());
        self
    }

    pub fn from_record(mut self, value: i64) -> Self {
        self.from_record = Some(value);
        self
    }

    pub fn limit_record(mut self, value: i64) -> Self {
        self.limit_record = Some(value);
        self
    }

    pub fn parameters(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.parameters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ExportBatchesOutOrgQueryRequest`].
    pub fn build(self) -> Result<ExportBatchesOutOrgQueryRequest, BuildError> {
        Ok(ExportBatchesOutOrgQueryRequest {
            columns_export: self.columns_export,
            from_record: self.from_record,
            limit_record: self.limit_record,
            parameters: self.parameters,
        })
    }
}
