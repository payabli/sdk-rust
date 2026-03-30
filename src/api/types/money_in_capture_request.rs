pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CaptureRequest {
    #[serde(rename = "paymentDetails")]
    #[serde(default)]
    pub payment_details: CapturePaymentDetails,
}

impl CaptureRequest {
    pub fn builder() -> CaptureRequestBuilder {
        <CaptureRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CaptureRequestBuilder {
    payment_details: Option<CapturePaymentDetails>,
}

impl CaptureRequestBuilder {
    pub fn payment_details(mut self, value: CapturePaymentDetails) -> Self {
        self.payment_details = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CaptureRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`payment_details`](CaptureRequestBuilder::payment_details)
    pub fn build(self) -> Result<CaptureRequest, BuildError> {
        Ok(CaptureRequest {
            payment_details: self
                .payment_details
                .ok_or_else(|| BuildError::missing_field("payment_details"))?,
        })
    }
}
