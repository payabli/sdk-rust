pub use crate::prelude::*;

/// Object containing payment details.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RequestOutAuthorizePaymentDetails {
    #[serde(rename = "checkNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_number: Option<VendorCheckNumber>,
    /// Currency code ISO-4217. If no code is provided, then the currency in the paypoint setting is used. Default is **USD**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Service fee to be deducted from the total amount. This amount must be a number, percentages aren't accepted. If you are using a percentage-based fee schedule, you must calculate the value manually.
    #[serde(rename = "serviceFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub service_fee: Option<f64>,
    /// Total amount to be charged. If a service fee is included, then this amount should include the service fee.
    #[serde(rename = "totalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_amount: Option<f64>,
    /// Indicates whether the payout should be bundled into a single transaction or processed separately. If set to `true`, each bill will be processed as a separate payout. If `false` or not provided, then multiple bills will be paid with a single payout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unbundled: Option<bool>,
}

impl RequestOutAuthorizePaymentDetails {
    pub fn builder() -> RequestOutAuthorizePaymentDetailsBuilder {
        <RequestOutAuthorizePaymentDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequestOutAuthorizePaymentDetailsBuilder {
    check_number: Option<VendorCheckNumber>,
    currency: Option<String>,
    service_fee: Option<f64>,
    total_amount: Option<f64>,
    unbundled: Option<bool>,
}

impl RequestOutAuthorizePaymentDetailsBuilder {
    pub fn check_number(mut self, value: VendorCheckNumber) -> Self {
        self.check_number = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn service_fee(mut self, value: f64) -> Self {
        self.service_fee = Some(value);
        self
    }

    pub fn total_amount(mut self, value: f64) -> Self {
        self.total_amount = Some(value);
        self
    }

    pub fn unbundled(mut self, value: bool) -> Self {
        self.unbundled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RequestOutAuthorizePaymentDetails`].
    pub fn build(self) -> Result<RequestOutAuthorizePaymentDetails, BuildError> {
        Ok(RequestOutAuthorizePaymentDetails {
            check_number: self.check_number,
            currency: self.currency,
            service_fee: self.service_fee,
            total_amount: self.total_amount,
            unbundled: self.unbundled,
        })
    }
}
