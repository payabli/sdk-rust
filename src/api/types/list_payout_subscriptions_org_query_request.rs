pub use crate::prelude::*;

/// Query parameters for ListPayoutSubscriptionsOrg
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ListPayoutSubscriptionsOrgQueryRequest {
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
    /// --url https://api-sandbox.payabli.com/api/Query/payoutsubscriptions/org/236?parameters=totalAmount(gt)=1000&limitRecord=20
    ///
    /// should become:
    ///
    /// --url https://api-sandbox.payabli.com/api/Query/payoutsubscriptions/org/236?totalAmount(gt)=1000&limitRecord=20
    /// </Info>
    /// See [Filters and Conditions Reference](/developers/developer-guides/pay-ops-reporting-engine-overview#filters-and-conditions-reference) for more information.
    /// **Accepted field names:**
    ///
    /// - `startDate` (gt, ge, lt, le, eq, ne)
    /// - `endDate` (gt, ge, lt, le, eq, ne)
    /// - `nextDate` (gt, ge, lt, le, eq, ne)
    /// - `frequency` (in, nin, ne, eq)
    /// - `method` (in, nin, eq, ne)
    /// - `totalAmount` (gt, ge, lt, le, eq, ne)
    /// - `netAmount` (gt, ge, lt, le, eq, ne)
    /// - `feeAmount` (gt, ge, lt, le, eq, ne)
    /// - `status` (in, nin, eq, ne)
    /// - `untilcancelled` (eq, ne)
    /// - `payaccountLastfour` (ct, nct)
    /// - `payaccountType` (ne, eq, in, nin)
    /// - `payaccountCurrency` (ne, eq, in, nin)
    /// - `paypointId` (ne, eq)
    /// - `paypointLegal` (ne, eq, ct, nct)
    /// - `paypointDba` (ne, eq, ct, nct)
    /// - `orgName` (ne, eq, ct, nct, nin, in)
    /// - `parentOrgId` (ne, eq, nin, in)
    /// - `subscriptionId` (eq, ne)
    /// - `orderDescription` (ct, nct)
    /// - `cycles` (eq, ne, gt, ge, lt, le)
    /// - `leftcycles` (eq, ne, gt, ge, lt, le)
    /// - `createdAt` (eq, ne, gt, ge, lt, le)
    /// - `updatedOn` (eq, ne, gt, ge, lt, le)
    /// - `vendorNumber` (ne, eq, ct, nct)
    /// - `name` (ne, eq, ct, nct)
    /// - `phone` (ne, eq, ct, nct)
    /// - `email` (ne, eq, ct, nct)
    /// - `address` (ne, eq, ct, nct)
    /// - `remitAddress` (ct, nct)
    /// - `city` (ne, eq, ct, nct)
    /// - `state` (ne, eq, ct, nct)
    /// - `country` (ne, eq, ct, nct)
    /// - `zip` (ne, eq, ct, nct)
    ///
    /// **Accepted comparison operators:**
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, Option<String>>>,
    /// The field name to use for sorting results. Use `desc(field_name)` to sort descending by `field_name`, and use `asc(field_name)` to sort ascending by `field_name`.
    #[serde(rename = "sortBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
}

impl ListPayoutSubscriptionsOrgQueryRequest {
    pub fn builder() -> ListPayoutSubscriptionsOrgQueryRequestBuilder {
        <ListPayoutSubscriptionsOrgQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPayoutSubscriptionsOrgQueryRequestBuilder {
    export_format: Option<ExportFormat>,
    from_record: Option<i64>,
    limit_record: Option<i64>,
    parameters: Option<HashMap<String, Option<String>>>,
    sort_by: Option<String>,
}

impl ListPayoutSubscriptionsOrgQueryRequestBuilder {
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

    /// Consumes the builder and constructs a [`ListPayoutSubscriptionsOrgQueryRequest`].
    pub fn build(self) -> Result<ListPayoutSubscriptionsOrgQueryRequest, BuildError> {
        Ok(ListPayoutSubscriptionsOrgQueryRequest {
            export_format: self.export_format,
            from_record: self.from_record,
            limit_record: self.limit_record,
            parameters: self.parameters,
            sort_by: self.sort_by,
        })
    }
}
