pub use crate::prelude::*;

/// Query parameters for ListBoardingLinks
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListBoardingLinksQueryRequest {
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
    /// - `lastUpdated` (gt, ge, lt, le, eq, ne)
    /// - `templateName` (ct, nct)
    /// - `referenceName` (ct, nct)
    /// - `acceptRegister` (eq, ne)
    /// - `acceptAuth` (eq, ne)
    /// - `templateCode` (ct, nct)
    /// - `templateId` (eq, ne)
    /// - `orgParentname` (ct, nct)
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
    /// Example: templateName(ct)=hoa return all records with template title containing "hoa"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, Option<String>>>,
    /// The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

impl ListBoardingLinksQueryRequest {
    pub fn builder() -> ListBoardingLinksQueryRequestBuilder {
        <ListBoardingLinksQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListBoardingLinksQueryRequestBuilder {
    from_record: Option<i64>,
    limit_record: Option<i64>,
    parameters: Option<HashMap<String, Option<String>>>,
    sort_by: Option<String>,
}

impl ListBoardingLinksQueryRequestBuilder {
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

    /// Consumes the builder and constructs a [`ListBoardingLinksQueryRequest`].
    pub fn build(self) -> Result<ListBoardingLinksQueryRequest, BuildError> {
        Ok(ListBoardingLinksQueryRequest {
            from_record: self.from_record,
            limit_record: self.limit_record,
            parameters: self.parameters,
            sort_by: self.sort_by,
        })
    }
}
