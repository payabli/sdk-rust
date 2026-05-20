pub use crate::prelude::*;

/// The PaymentDetail object for microdeposit (MakeCredit) transactions.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaymentDetailCredit {
    /// Currency code ISO-4217. If not code is provided the currency in the paypoint setting is taken. Default is **USD**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Service fee to be deducted from the total amount. This amount must be a number, percentages aren't accepted. If you are using a percentage-based fee schedule, you must calculate the value manually.
    #[serde(rename = "serviceFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub service_fee: Option<f64>,
    /// Total amount to be charged. If a service fee is provided, then this amount should include the service fee.
    #[serde(rename = "totalAmount")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_amount: f64,
}

impl PaymentDetailCredit {
    pub fn builder() -> PaymentDetailCreditBuilder {
        <PaymentDetailCreditBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentDetailCreditBuilder {
    currency: Option<String>,
    service_fee: Option<f64>,
    total_amount: Option<f64>,
}

impl PaymentDetailCreditBuilder {
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

    /// Consumes the builder and constructs a [`PaymentDetailCredit`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total_amount`](PaymentDetailCreditBuilder::total_amount)
    pub fn build(self) -> Result<PaymentDetailCredit, BuildError> {
        Ok(PaymentDetailCredit {
            currency: self.currency,
            service_fee: self.service_fee,
            total_amount: self
                .total_amount
                .ok_or_else(|| BuildError::missing_field("total_amount"))?,
        })
    }
}
