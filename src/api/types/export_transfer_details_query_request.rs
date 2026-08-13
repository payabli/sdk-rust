pub use crate::prelude::*;

/// Query parameters for ExportTransferDetails
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ExportTransferDetailsQueryRequest {
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
    /// Because of a technical limitation, you can't make a request that includes filters from the API console on this page. The response won't be filtered. Instead, copy the request, remove `parameters=` and run the request in a different client, for example:
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
    /// **Accepted field names:**
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, Option<String>>>,
    /// The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

impl ExportTransferDetailsQueryRequest {
    pub fn builder() -> ExportTransferDetailsQueryRequestBuilder {
        <ExportTransferDetailsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ExportTransferDetailsQueryRequestBuilder {
    columns_export: Option<String>,
    from_record: Option<i64>,
    limit_record: Option<i64>,
    parameters: Option<HashMap<String, Option<String>>>,
    sort_by: Option<String>,
}

impl ExportTransferDetailsQueryRequestBuilder {
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

    pub fn sort_by(mut self, value: impl Into<String>) -> Self {
        self.sort_by = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ExportTransferDetailsQueryRequest`].
    pub fn build(self) -> Result<ExportTransferDetailsQueryRequest, BuildError> {
        Ok(ExportTransferDetailsQueryRequest {
            columns_export: self.columns_export,
            from_record: self.from_record,
            limit_record: self.limit_record,
            parameters: self.parameters,
            sort_by: self.sort_by,
        })
    }
}
