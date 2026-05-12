pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CapturePaymentDetails {
    /// Total amount to be captured, including the `serviceFee` amount. The amount can't be greater the original
    /// total amount of the transaction, and can't be more than 15% lower than the original amount.
    #[serde(rename = "totalAmount")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_amount: f64,
    /// Service fee to capture for the transaction.
    #[serde(rename = "serviceFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub service_fee: Option<f64>,
}

impl CapturePaymentDetails {
    pub fn builder() -> CapturePaymentDetailsBuilder {
        <CapturePaymentDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CapturePaymentDetailsBuilder {
    total_amount: Option<f64>,
    service_fee: Option<f64>,
}

impl CapturePaymentDetailsBuilder {
    pub fn total_amount(mut self, value: f64) -> Self {
        self.total_amount = Some(value);
        self
    }

    pub fn service_fee(mut self, value: f64) -> Self {
        self.service_fee = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CapturePaymentDetails`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total_amount`](CapturePaymentDetailsBuilder::total_amount)
    pub fn build(self) -> Result<CapturePaymentDetails, BuildError> {
        Ok(CapturePaymentDetails {
            total_amount: self
                .total_amount
                .ok_or_else(|| BuildError::missing_field("total_amount"))?,
            service_fee: self.service_fee,
        })
    }
}
