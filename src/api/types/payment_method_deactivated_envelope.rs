pub use crate::prelude::*;

/// Standard boarding response envelope wrapping a deactivated payment method.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodDeactivatedEnvelope {
    /// Whether the request succeeded.
    #[serde(default)]
    pub success: bool,
    /// A human-readable result message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The trace identifier for the request, useful for support.
    #[serde(rename = "traceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    /// The deactivated payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<PaymentMethodResult>,
}

impl PaymentMethodDeactivatedEnvelope {
    pub fn builder() -> PaymentMethodDeactivatedEnvelopeBuilder {
        <PaymentMethodDeactivatedEnvelopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodDeactivatedEnvelopeBuilder {
    success: Option<bool>,
    message: Option<String>,
    trace_id: Option<String>,
    data: Option<PaymentMethodResult>,
}

impl PaymentMethodDeactivatedEnvelopeBuilder {
    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn trace_id(mut self, value: impl Into<String>) -> Self {
        self.trace_id = Some(value.into());
        self
    }

    pub fn data(mut self, value: PaymentMethodResult) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodDeactivatedEnvelope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](PaymentMethodDeactivatedEnvelopeBuilder::success)
    pub fn build(self) -> Result<PaymentMethodDeactivatedEnvelope, BuildError> {
        Ok(PaymentMethodDeactivatedEnvelope {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            message: self.message,
            trace_id: self.trace_id,
            data: self.data,
        })
    }
}
