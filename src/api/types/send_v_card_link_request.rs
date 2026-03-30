pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendVCardLinkRequest {
    /// The transaction ID of the virtual card payout. The ID is returned as `ReferenceId` in the response when you authorize a payout with POST /MoneyOut/authorize.
    #[serde(rename = "transId")]
    #[serde(default)]
    pub trans_id: String,
}

impl SendVCardLinkRequest {
    pub fn builder() -> SendVCardLinkRequestBuilder {
        <SendVCardLinkRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendVCardLinkRequestBuilder {
    trans_id: Option<String>,
}

impl SendVCardLinkRequestBuilder {
    pub fn trans_id(mut self, value: impl Into<String>) -> Self {
        self.trans_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SendVCardLinkRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`trans_id`](SendVCardLinkRequestBuilder::trans_id)
    pub fn build(self) -> Result<SendVCardLinkRequest, BuildError> {
        Ok(SendVCardLinkRequest {
            trans_id: self
                .trans_id
                .ok_or_else(|| BuildError::missing_field("trans_id"))?,
        })
    }
}
