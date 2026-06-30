pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RenewVCardRequest {
    /// The new expiration date for the virtual card, in `MM-YYYY` or `MM/YYYY` format. The card expires on the last day of the month you specify. The date can't be more than 2 years and 363 days in the future.
    #[serde(rename = "expirationDate")]
    #[serde(default)]
    pub expiration_date: String,
}

impl RenewVCardRequest {
    pub fn builder() -> RenewVCardRequestBuilder {
        <RenewVCardRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RenewVCardRequestBuilder {
    expiration_date: Option<String>,
}

impl RenewVCardRequestBuilder {
    pub fn expiration_date(mut self, value: impl Into<String>) -> Self {
        self.expiration_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RenewVCardRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`expiration_date`](RenewVCardRequestBuilder::expiration_date)
    pub fn build(self) -> Result<RenewVCardRequest, BuildError> {
        Ok(RenewVCardRequest {
            expiration_date: self
                .expiration_date
                .ok_or_else(|| BuildError::missing_field("expiration_date"))?,
        })
    }
}
