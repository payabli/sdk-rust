pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteGhostCardResponseData {
    /// The type of delete performed. Always `soft`, because Payabli keeps the card record.
    #[serde(rename = "deleteType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete_type: Option<String>,
    /// Token of the canceled ghost card.
    #[serde(rename = "cardToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_token: Option<String>,
}

impl DeleteGhostCardResponseData {
    pub fn builder() -> DeleteGhostCardResponseDataBuilder {
        <DeleteGhostCardResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteGhostCardResponseDataBuilder {
    delete_type: Option<String>,
    card_token: Option<String>,
}

impl DeleteGhostCardResponseDataBuilder {
    pub fn delete_type(mut self, value: impl Into<String>) -> Self {
        self.delete_type = Some(value.into());
        self
    }

    pub fn card_token(mut self, value: impl Into<String>) -> Self {
        self.card_token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeleteGhostCardResponseData`].
    pub fn build(self) -> Result<DeleteGhostCardResponseData, BuildError> {
        Ok(DeleteGhostCardResponseData {
            delete_type: self.delete_type,
            card_token: self.card_token,
        })
    }
}
