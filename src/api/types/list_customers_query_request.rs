pub use crate::prelude::*;

/// Query parameters for ListCustomers
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListCustomersQueryRequest {
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
    /// Collection of field names, conditions, and values used to filter the query.
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
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more details.
    ///
    /// **Accepted field names:**
    ///
    /// - `createdDate` (gt, ge, lt, le, eq, ne)
    /// - `customernumber` (ne, eq, ct, nct)
    /// - `firstname` (ne, eq, ct, nct)
    /// - `lastname` (ne, eq, ct, nct)
    /// - `name` (ct, nct)
    /// - `address` (ne, eq, ct, nct)
    /// - `city` (ne, eq, ct, nct)
    /// - `country` (ne, eq, ct, nct)
    /// - `zip` (ne, eq, ct, nct)
    /// - `state` (ne, eq, ct, nct)
    /// - `shippingaddress` (ne, eq, ct, nct)
    /// - `shippingcity` (ne, eq, ct, nct)
    /// - `shippingcountry` (ne, eq, ct, nct)
    /// - `shippingzip` (ne, eq, ct, nct)
    /// - `shippingstate` (ne, eq, ct, nct)
    /// - `phone` (ne, eq, ct, nct)
    /// - `email` (ne, eq, ct, nct)
    /// - `company` (ne, eq, ct, nct)
    /// - `username` (ne, eq, ct, nct)
    /// - `balance` (gt, ge, lt, le, eq, ne)
    /// - `status` (in, nin, eq, ne)
    /// - `additional-xxx` (ne, eq, ct, nct) where xxx is the additional field name
    /// - `orgId` (eq) *mandatory when entry=org*
    /// - `paypointId` (ne, eq)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct)
    ///
    /// **Accepted comparison operators:**
    ///
    /// - `eq` or empty => equal
    /// - `gt` => greater than
    /// - `ge` => greater or equal
    /// - `lt` => less than
    /// - `le` => less or equal
    /// - `ne` => not equal
    /// - `ct` => contains
    /// - `nct` => not contains
    /// - `in` => inside array separated by "|"
    /// - `nin` => not inside array separated by "|"
    ///
    /// **Accepted parameters:**
    /// - `limitRecord`: Max number of records for query (default="20", "0" or negative value for all)
    /// - `fromRecord`: Initial record in query
    ///
    /// **Example Usage:**
    /// `balance(gt)=20` will return all records with a balance greater than 20.00.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, Option<String>>>,
    /// The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

impl ListCustomersQueryRequest {
    pub fn builder() -> ListCustomersQueryRequestBuilder {
        <ListCustomersQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListCustomersQueryRequestBuilder {
    export_format: Option<ExportFormat>,
    from_record: Option<i64>,
    limit_record: Option<i64>,
    parameters: Option<HashMap<String, Option<String>>>,
    sort_by: Option<String>,
}

impl ListCustomersQueryRequestBuilder {
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

    /// Consumes the builder and constructs a [`ListCustomersQueryRequest`].
    pub fn build(self) -> Result<ListCustomersQueryRequest, BuildError> {
        Ok(ListCustomersQueryRequest {
            export_format: self.export_format,
            from_record: self.from_record,
            limit_record: self.limit_record,
            parameters: self.parameters,
            sort_by: self.sort_by,
        })
    }
}
