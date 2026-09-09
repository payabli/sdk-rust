pub use crate::prelude::*;

/// Request for AuthorizeOut (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AuthorizeOutRequest {
    /// When `true`, the authorization bypasses the requirement for unique bills, identified by vendor invoice number. This allows you to make more than one payout authorization for a bill, like a split payment.
    #[serde(rename = "allowDuplicatedBills")]
    #[serde(skip)]
    pub allow_duplicated_bills: Option<bool>,
    /// When `true`, Payabli won't automatically create a bill for this payout transaction.
    #[serde(rename = "doNotCreateBills")]
    #[serde(skip)]
    pub do_not_create_bills: Option<bool>,
    /// When `true`, Payabli authorizes the payout for same-day ACH processing instead of standard ACH. Same-day ACH must be enabled for the paypoint, otherwise the authorization fails with a `400` response and `responseCode` `3492`. Only ACH payouts honor this flag. Wire and RTP payouts ignore it.
    ///
    /// Same-day ACH has a daily cutoff. Capture the transaction before the cutoff, or pass `autoConvertSameDayAch` with a value of `true` when you capture it.
    #[serde(rename = "sameDayACH")]
    #[serde(skip)]
    pub same_day_ach: Option<bool>,
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
    same_day_ach: Option<bool>,
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

    pub fn same_day_ach(mut self, value: bool) -> Self {
        self.same_day_ach = Some(value);
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
            same_day_ach: self.same_day_ach,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
