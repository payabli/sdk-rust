pub use crate::prelude::*;

/// Standard boarding response envelope wrapping the created payment methods.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentMethodsCreatedEnvelope {
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
    /// The created payment methods, in the order they were sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<PaymentMethodResult>>,
}

impl PaymentMethodsCreatedEnvelope {
    pub fn builder() -> PaymentMethodsCreatedEnvelopeBuilder {
        <PaymentMethodsCreatedEnvelopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentMethodsCreatedEnvelopeBuilder {
    success: Option<bool>,
    message: Option<String>,
    trace_id: Option<String>,
    data: Option<Vec<PaymentMethodResult>>,
}

impl PaymentMethodsCreatedEnvelopeBuilder {
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

    pub fn data(mut self, value: Vec<PaymentMethodResult>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentMethodsCreatedEnvelope`].
    /// This method will fail if any of the following fields are not set:
    /// - [`success`](PaymentMethodsCreatedEnvelopeBuilder::success)
    pub fn build(self) -> Result<PaymentMethodsCreatedEnvelope, BuildError> {
        Ok(PaymentMethodsCreatedEnvelope {
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            message: self.message,
            trace_id: self.trace_id,
            data: self.data,
        })
    }
}
