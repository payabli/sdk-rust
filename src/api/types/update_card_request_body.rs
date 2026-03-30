pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UpdateCardRequestBody {
    /// Token that uniquely identifies the card. This is the `ReferenceId` returned when the card was created.
    #[serde(rename = "cardToken")]
    #[serde(default)]
    pub card_token: String,
    /// The new status to set on the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<CardStatus>,
}

impl UpdateCardRequestBody {
    pub fn builder() -> UpdateCardRequestBodyBuilder {
        <UpdateCardRequestBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateCardRequestBodyBuilder {
    card_token: Option<String>,
    status: Option<CardStatus>,
}

impl UpdateCardRequestBodyBuilder {
    pub fn card_token(mut self, value: impl Into<String>) -> Self {
        self.card_token = Some(value.into());
        self
    }

    pub fn status(mut self, value: CardStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdateCardRequestBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`card_token`](UpdateCardRequestBodyBuilder::card_token)
    pub fn build(self) -> Result<UpdateCardRequestBody, BuildError> {
        Ok(UpdateCardRequestBody {
            card_token: self
                .card_token
                .ok_or_else(|| BuildError::missing_field("card_token"))?,
            status: self.status,
        })
    }
}
