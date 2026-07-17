pub use crate::prelude::*;

/// Request for getpaid (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetpaidRequest {
    /// When `true`, enables real-time validation of ACH account and routing numbers. This is an add-on feature, contact Payabli for more information.
    #[serde(rename = "achValidation")]
    #[serde(skip)]
    pub ach_validation: Option<AchValidation>,
    /// When `true`, the request creates a new customer record, regardless of whether customer identifiers match an existing customer. Defaults to `false`.
    #[serde(rename = "forceCustomerCreation")]
    #[serde(skip)]
    pub force_customer_creation: Option<ForceCustomerCreation>,
    /// When `true`, transactionDetails object is returned in the response. See a full example of the `transactionDetails` object in the [Transaction integration guide](/developers/developer-guides/money-in-transaction-add#includedetailstrue-response).
    #[serde(rename = "includeDetails")]
    #[serde(skip)]
    pub include_details: Option<bool>,
    pub body: TransRequestBody,
}

impl GetpaidRequest {
    pub fn builder() -> GetpaidRequestBuilder {
        <GetpaidRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetpaidRequestBuilder {
    ach_validation: Option<AchValidation>,
    force_customer_creation: Option<ForceCustomerCreation>,
    include_details: Option<bool>,
    body: Option<TransRequestBody>,
}

impl GetpaidRequestBuilder {
    pub fn ach_validation(mut self, value: AchValidation) -> Self {
        self.ach_validation = Some(value);
        self
    }

    pub fn force_customer_creation(mut self, value: ForceCustomerCreation) -> Self {
        self.force_customer_creation = Some(value);
        self
    }

    pub fn include_details(mut self, value: bool) -> Self {
        self.include_details = Some(value);
        self
    }

    pub fn body(mut self, value: TransRequestBody) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetpaidRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](GetpaidRequestBuilder::body)
    pub fn build(self) -> Result<GetpaidRequest, BuildError> {
        Ok(GetpaidRequest {
            ach_validation: self.ach_validation,
            force_customer_creation: self.force_customer_creation,
            include_details: self.include_details,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
