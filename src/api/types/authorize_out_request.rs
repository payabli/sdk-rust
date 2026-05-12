pub use crate::prelude::*;

/// Request for AuthorizeOut (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AuthorizeOutRequest {
    /// When `true`, the authorization bypasses the requirement for unique bills, identified by vendor invoice number. This allows you to make more than one payout authorization for a bill, like a split payment.
    #[serde(rename = "allowDuplicatedBills")]
    #[serde(skip_serializing)]
    pub allow_duplicated_bills: Option<bool>,
    /// When `true`, Payabli won't automatically create a bill for this payout transaction.
    #[serde(rename = "doNotCreateBills")]
    #[serde(skip_serializing)]
    pub do_not_create_bills: Option<bool>,
    /// When `true`, the request creates a new vendor record, regardless of whether the vendor already exists.
    #[serde(rename = "forceVendorCreation")]
    #[serde(skip_serializing)]
    pub force_vendor_creation: Option<bool>,
    #[serde(default)]
    pub body: AuthorizePayoutBody,
}

impl AuthorizeOutRequest {
    pub fn builder() -> AuthorizeOutRequestBuilder {
        <AuthorizeOutRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthorizeOutRequestBuilder {
    allow_duplicated_bills: Option<bool>,
    do_not_create_bills: Option<bool>,
    force_vendor_creation: Option<bool>,
    body: Option<AuthorizePayoutBody>,
}

impl AuthorizeOutRequestBuilder {
    pub fn allow_duplicated_bills(mut self, value: bool) -> Self {
        self.allow_duplicated_bills = Some(value);
        self
    }

    pub fn do_not_create_bills(mut self, value: bool) -> Self {
        self.do_not_create_bills = Some(value);
        self
    }

    pub fn force_vendor_creation(mut self, value: bool) -> Self {
        self.force_vendor_creation = Some(value);
        self
    }

    pub fn body(mut self, value: AuthorizePayoutBody) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AuthorizeOutRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](AuthorizeOutRequestBuilder::body)
    pub fn build(self) -> Result<AuthorizeOutRequest, BuildError> {
        Ok(AuthorizeOutRequest {
            allow_duplicated_bills: self.allow_duplicated_bills,
            do_not_create_bills: self.do_not_create_bills,
            force_vendor_creation: self.force_vendor_creation,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
