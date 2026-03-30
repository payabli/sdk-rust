pub use crate::prelude::*;

/// Query parameters for SendReceipt2Trans
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendReceipt2TransQueryRequest {
    /// Email address where the payment receipt should be sent.
    ///
    /// If not provided, the email address on file for the user owner of the transaction is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl SendReceipt2TransQueryRequest {
    pub fn builder() -> SendReceipt2TransQueryRequestBuilder {
        <SendReceipt2TransQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendReceipt2TransQueryRequestBuilder {
    email: Option<String>,
}

impl SendReceipt2TransQueryRequestBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SendReceipt2TransQueryRequest`].
    pub fn build(self) -> Result<SendReceipt2TransQueryRequest, BuildError> {
        Ok(SendReceipt2TransQueryRequest { email: self.email })
    }
}
