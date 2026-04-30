pub use crate::prelude::*;

/// Query parameters for ListDevices
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListDevicesQueryRequest {
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
    /// --url https://api-sandbox.payabli.com/api/Query/devices/8cfec329267?parameters=status=1&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/devices/8cfec329267?status=1&limitRecord=20
    /// </Info>
    ///
    /// See [Filters and Conditions
    /// Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference)
    /// for more information.
    ///
    /// **List of field names accepted:**
    ///
    ///
    /// - `deviceId` (eq, ne, ct, nct)
    ///
    /// - `serialNumber` (eq, ne, ct, nct)
    ///
    /// - `friendlyName` (eq, ne, ct, nct)
    ///
    /// - `description` (eq, ne, ct, nct)
    ///
    /// - `model` (eq, ne, ct, nct)
    ///
    /// - `make` (eq, ne, ct, nct)
    ///
    /// - `macAddress` (eq, ne, ct, nct)
    ///
    /// - `registrationCode` (eq, ne, ct, nct)
    ///
    /// - `status` (eq, ne, in, nin)
    ///
    /// - `deviceType` (eq, ne, in, nin)
    ///
    /// - `deviceOs` (eq, ne, in, nin)
    ///
    /// - `activationAttempts` (eq, ne, gt, ge, lt, le)
    ///
    /// - `createdDate` (gt, ge, lt, le, eq, ne)
    ///
    /// - `updatedDate` (gt, ge, lt, le, eq, ne)
    ///
    /// - `lastHealthCheck` (gt, ge, lt, le, eq, ne)
    ///
    /// - `activationExpiry` (gt, ge, lt, le, eq, ne). This filter corresponds to the `activationCodeExpiry` response field.
    ///
    /// - `paypointId` (eq, ne)
    ///
    /// - `paypointDba` (eq, ne, ct, nct)
    ///
    /// - `paypointLegal` (eq, ne, ct, nct)
    ///
    /// - `paypointEntry` (eq, ne, ct, nct)
    ///
    /// - `externalPaypointId` (eq, ne, ct, nct)
    ///
    /// - `parentOrgId` (eq, ne)
    ///
    /// - `parentOrgName` (eq, ne, ct, nct)
    ///
    ///
    /// **List of comparison operators accepted:**
    ///
    /// - `eq` or empty => equal
    ///
    /// - `gt` => greater than
    ///
    /// - `ge` => greater or equal
    ///
    /// - `lt` => less than
    ///
    /// - `le` => less or equal
    ///
    /// - `ne` => not equal
    ///
    /// - `ct` => contains
    ///
    /// - `nct` => not contains
    ///
    /// - `in` => inside array
    ///
    /// - `nin` => not inside array
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, Option<String>>>,
    /// The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

impl ListDevicesQueryRequest {
    pub fn builder() -> ListDevicesQueryRequestBuilder {
        <ListDevicesQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDevicesQueryRequestBuilder {
    export_format: Option<ExportFormat>,
    from_record: Option<i64>,
    limit_record: Option<i64>,
    parameters: Option<HashMap<String, Option<String>>>,
    sort_by: Option<String>,
}

impl ListDevicesQueryRequestBuilder {
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

    /// Consumes the builder and constructs a [`ListDevicesQueryRequest`].
    pub fn build(self) -> Result<ListDevicesQueryRequest, BuildError> {
        Ok(ListDevicesQueryRequest {
            export_format: self.export_format,
            from_record: self.from_record,
            limit_record: self.limit_record,
            parameters: self.parameters,
            sort_by: self.sort_by,
        })
    }
}
