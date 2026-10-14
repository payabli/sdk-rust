pub use crate::prelude::*;

/// Signature captured on a card-present device. Always `null` for payout subscriptions.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayoutSubscriptionPaymentDataPascalPayorSignature {
    /// Availability of the signature. One of `stored`, `pending`, or `unavailable`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Image format of the stored signature, `png` or `jpeg`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<String>,
    /// Base64-encoded signature image. Only set when `status` is `stored`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<String>,
}

impl PayoutSubscriptionPaymentDataPascalPayorSignature {
    pub fn builder() -> PayoutSubscriptionPaymentDataPascalPayorSignatureBuilder {
        <PayoutSubscriptionPaymentDataPascalPayorSignatureBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayoutSubscriptionPaymentDataPascalPayorSignatureBuilder {
    status: Option<String>,
    format: Option<String>,
    content: Option<String>,
}

impl PayoutSubscriptionPaymentDataPascalPayorSignatureBuilder {
    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn format(mut self, value: impl Into<String>) -> Self {
        self.format = Some(value.into());
        self
    }

    pub fn content(mut self, value: impl Into<String>) -> Self {
        self.content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PayoutSubscriptionPaymentDataPascalPayorSignature`].
    pub fn build(self) -> Result<PayoutSubscriptionPaymentDataPascalPayorSignature, BuildError> {
        Ok(PayoutSubscriptionPaymentDataPascalPayorSignature {
            status: self.status,
            format: self.format,
            content: self.content,
        })
    }
}
