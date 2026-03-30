pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransferMessageProperties {
    #[serde(rename = "originalTransferStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_transfer_status: Option<String>,
    #[serde(rename = "currentTransferStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_transfer_status: Option<String>,
}

impl TransferMessageProperties {
    pub fn builder() -> TransferMessagePropertiesBuilder {
        <TransferMessagePropertiesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferMessagePropertiesBuilder {
    original_transfer_status: Option<String>,
    current_transfer_status: Option<String>,
}

impl TransferMessagePropertiesBuilder {
    pub fn original_transfer_status(mut self, value: impl Into<String>) -> Self {
        self.original_transfer_status = Some(value.into());
        self
    }

    pub fn current_transfer_status(mut self, value: impl Into<String>) -> Self {
        self.current_transfer_status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransferMessageProperties`].
    pub fn build(self) -> Result<TransferMessageProperties, BuildError> {
        Ok(TransferMessageProperties {
            original_transfer_status: self.original_transfer_status,
            current_transfer_status: self.current_transfer_status,
        })
    }
}
