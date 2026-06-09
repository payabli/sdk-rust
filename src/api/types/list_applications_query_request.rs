pub use crate::prelude::*;

/// Query parameters for ListApplications
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListApplicationsQueryRequest {
    /// Export format for file downloads. When specified, returns data as a file instead of JSON.
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

impl ListApplicationsQueryRequest {
    pub fn builder() -> ListApplicationsQueryRequestBuilder {
        <ListApplicationsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListApplicationsQueryRequestBuilder {
    export_format: Option<ExportFormat>,
    from_record: Option<i64>,
    limit_record: Option<i64>,
    parameters: Option<HashMap<String, Option<String>>>,
    sort_by: Option<String>,
}

impl ListApplicationsQueryRequestBuilder {
    pub fn export_format(mut self, value: ExportFormat) -> Self {
        self.export_format = Some(value);
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

    /// Consumes the builder and constructs a [`ListApplicationsQueryRequest`].
    pub fn build(self) -> Result<ListApplicationsQueryRequest, BuildError> {
        Ok(ListApplicationsQueryRequest {
            export_format: self.export_format,
            from_record: self.from_record,
            limit_record: self.limit_record,
            parameters: self.parameters,
            sort_by: self.sort_by,
        })
    }
}
