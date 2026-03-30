pub use crate::prelude::*;

/// Request for getpaid (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetpaidRequest {
    #[serde(rename = "achValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_validation: Option<AchValidation>,
    #[serde(rename = "forceCustomerCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_customer_creation: Option<ForceCustomerCreation>,
    /// When `true`, transactionDetails object is returned in the response. See a full example of the `transactionDetails` object in the [Transaction integration guide](/developers/developer-guides/money-in-transaction-add#includedetailstrue-response).
    #[serde(rename = "includeDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
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
