pub use crate::prelude::*;

/// Request for Payout (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PayoutRequest {
    /// When `true`, Payabli authorizes the payout for same-day ACH processing instead of standard ACH. Same-day ACH must be enabled for the paypoint, otherwise the authorization fails with a `400` response and `responseCode` `3492`. Only ACH payouts honor this flag. Wire and RTP payouts ignore it.
    ///
    /// Because this endpoint captures immediately, pass `autoConvertSameDayAch` with a value of `true` to fall back to standard ACH if the capture runs after the same-day ACH cutoff.
    #[serde(rename = "sameDayACH")]
    #[serde(skip)]
    pub same_day_ach: Option<bool>,
    /// When `true`, Payabli won't automatically create a bill for this payout transaction.
    #[serde(rename = "doNotCreateBills")]
    #[serde(skip)]
    pub do_not_create_bills: Option<bool>,
    /// When `true`, the payout bypasses the requirement for unique bills, identified by vendor invoice number. This allows you to make more than one payout for a bill, like a split payment.
    #[serde(rename = "allowDuplicatedBills")]
    #[serde(skip)]
    pub allow_duplicated_bills: Option<bool>,
    /// When `true`, Payabli updates the vendor's stored default payment method to the method used in this payout.
    #[serde(rename = "updateVendorPaymentMethod")]
    #[serde(skip)]
    pub update_vendor_payment_method: Option<bool>,
    /// Controls what happens to a payout authorized with `sameDayACH` set to `true` when the capture runs after the same-day ACH cutoff. When `true`, Payabli converts the payout to a standard ACH payment and captures it. When `false`, the capture is declined.
    ///
    /// This parameter has no effect on payouts that weren't authorized for same-day ACH.
    #[serde(rename = "autoConvertSameDayAch")]
    #[serde(skip)]
    pub auto_convert_same_day_ach: Option<bool>,
    #[serde(default)]
    pub body: AuthorizePayoutBody,
}

impl PayoutRequest {
    pub fn builder() -> PayoutRequestBuilder {
        <PayoutRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayoutRequestBuilder {
    same_day_ach: Option<bool>,
    do_not_create_bills: Option<bool>,
    allow_duplicated_bills: Option<bool>,
    update_vendor_payment_method: Option<bool>,
    auto_convert_same_day_ach: Option<bool>,
    body: Option<AuthorizePayoutBody>,
}

impl PayoutRequestBuilder {
    pub fn same_day_ach(mut self, value: bool) -> Self {
        self.same_day_ach = Some(value);
        self
    }

    pub fn do_not_create_bills(mut self, value: bool) -> Self {
        self.do_not_create_bills = Some(value);
        self
    }

    pub fn allow_duplicated_bills(mut self, value: bool) -> Self {
        self.allow_duplicated_bills = Some(value);
        self
    }

    pub fn update_vendor_payment_method(mut self, value: bool) -> Self {
        self.update_vendor_payment_method = Some(value);
        self
    }

    pub fn auto_convert_same_day_ach(mut self, value: bool) -> Self {
        self.auto_convert_same_day_ach = Some(value);
        self
    }

    pub fn body(mut self, value: AuthorizePayoutBody) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayoutRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](PayoutRequestBuilder::body)
    pub fn build(self) -> Result<PayoutRequest, BuildError> {
        Ok(PayoutRequest {
            same_day_ach: self.same_day_ach,
            do_not_create_bills: self.do_not_create_bills,
            allow_duplicated_bills: self.allow_duplicated_bills,
            update_vendor_payment_method: self.update_vendor_payment_method,
            auto_convert_same_day_ach: self.auto_convert_same_day_ach,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
