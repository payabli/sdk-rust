pub use crate::prelude::*;

/// Query parameters for ListVcards
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListVcardsQueryRequest {
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
    /// - `status` (eq, ne, ct, nct, sw, ew)
    /// - `createdAt` (gt, ge, lt, le, eq, ne)
    /// - `cardToken` (ct, nct, eq, ne)
    /// - `lastFour` (ct, nct, eq, ne)
    /// - `expirationDate` (ct, nct, eq, ne)
    /// - `payoutId` (eq, ne, gt, ge, lt, le)
    /// - `vendorId` (eq, ne, gt, ge, lt, le)
    /// - `miscData1` (ct, nct, eq, ne)
    /// - `miscData2` (ct, nct, eq, ne)
    /// - `currentUses` (gt, ge, lt, le, eq, ne)
    /// - `amount` (gt, ge, lt, le, eq, ne)
    /// - `balance` (gt, ge, lt, le, eq, ne)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (eq, ne, ct, nct, sw, ew)
    /// - `externalPaypointId` (ct, nct, eq, ne)
    /// - `paypointId` (eq, ne, gt, ge, lt, le)
    /// - `cardType` (eq, ne, gt, ge, lt, le)
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
    /// - sw => starts with
    /// - ew => ends with
    /// - in => inside array separated by "|"
    /// - nin => not inside array separated by "|"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, Option<String>>>,
    /// The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

impl ListVcardsQueryRequest {
    pub fn builder() -> ListVcardsQueryRequestBuilder {
        <ListVcardsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListVcardsQueryRequestBuilder {
    export_format: Option<ExportFormat>,
    from_record: Option<i64>,
    limit_record: Option<i64>,
    parameters: Option<HashMap<String, Option<String>>>,
    sort_by: Option<String>,
}

impl ListVcardsQueryRequestBuilder {
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

    /// Consumes the builder and constructs a [`ListVcardsQueryRequest`].
    pub fn build(self) -> Result<ListVcardsQueryRequest, BuildError> {
        Ok(ListVcardsQueryRequest {
            export_format: self.export_format,
            from_record: self.from_record,
            limit_record: self.limit_record,
            parameters: self.parameters,
            sort_by: self.sort_by,
        })
    }
}
