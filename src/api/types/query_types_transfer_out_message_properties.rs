pub use crate::prelude::*;

/// Properties associated with a transfer message.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransferOutMessageProperties {
    /// The original status of the transfer before the message.
    #[serde(rename = "originalTransferStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_transfer_status: Option<String>,
    /// The current status of the transfer after the message.
    #[serde(rename = "currentTransferStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_transfer_status: Option<String>,
}

impl TransferOutMessageProperties {
    pub fn builder() -> TransferOutMessagePropertiesBuilder {
        <TransferOutMessagePropertiesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferOutMessagePropertiesBuilder {
    original_transfer_status: Option<String>,
    current_transfer_status: Option<String>,
}

impl TransferOutMessagePropertiesBuilder {
    pub fn original_transfer_status(mut self, value: impl Into<String>) -> Self {
        self.original_transfer_status = Some(value.into());
        self
    }

    pub fn current_transfer_status(mut self, value: impl Into<String>) -> Self {
        self.current_transfer_status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransferOutMessageProperties`].
    pub fn build(self) -> Result<TransferOutMessageProperties, BuildError> {
        Ok(TransferOutMessageProperties {
            original_transfer_status: self.original_transfer_status,
            current_transfer_status: self.current_transfer_status,
        })
    }
}
