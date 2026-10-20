pub use crate::prelude::*;

/// The reference and updated timestamp after a paypoint update.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaypointUpdatedResponse {
    /// The paypoint reference.
    #[serde(rename = "paypointReference")]
    #[serde(default)]
    pub paypoint_reference: String,
    /// When the paypoint was last updated.
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub updated_at: DateTime<Utc>,
    /// Hypermedia links for the paypoint resource.
    #[serde(rename = "_links")]
    #[serde(default)]
    pub links: PaypointLinks,
}

impl PaypointUpdatedResponse {
    pub fn builder() -> PaypointUpdatedResponseBuilder {
        <PaypointUpdatedResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaypointUpdatedResponseBuilder {
    paypoint_reference: Option<String>,
    updated_at: Option<DateTime<Utc>>,
    links: Option<PaypointLinks>,
}

impl PaypointUpdatedResponseBuilder {
    pub fn paypoint_reference(mut self, value: impl Into<String>) -> Self {
        self.paypoint_reference = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: DateTime<Utc>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn links(mut self, value: PaypointLinks) -> Self {
        self.links = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaypointUpdatedResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`paypoint_reference`](PaypointUpdatedResponseBuilder::paypoint_reference)
    /// - [`updated_at`](PaypointUpdatedResponseBuilder::updated_at)
    /// - [`links`](PaypointUpdatedResponseBuilder::links)
    pub fn build(self) -> Result<PaypointUpdatedResponse, BuildError> {
        Ok(PaypointUpdatedResponse {
            paypoint_reference: self
                .paypoint_reference
                .ok_or_else(|| BuildError::missing_field("paypoint_reference"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            links: self
                .links
                .ok_or_else(|| BuildError::missing_field("links"))?,
        })
    }
}
