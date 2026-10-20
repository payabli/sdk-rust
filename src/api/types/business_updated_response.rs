pub use crate::prelude::*;

/// The reference and updated timestamp after a business update.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BusinessUpdatedResponse {
    /// The business reference.
    #[serde(rename = "businessUuid")]
    #[serde(default)]
    pub business_uuid: String,
    /// When the business was last updated.
    #[serde(rename = "lastUpdated")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub last_updated: DateTime<Utc>,
    /// Hypermedia links for the business resource.
    #[serde(rename = "_links")]
    #[serde(default)]
    pub links: BusinessLinks,
}

impl BusinessUpdatedResponse {
    pub fn builder() -> BusinessUpdatedResponseBuilder {
        <BusinessUpdatedResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BusinessUpdatedResponseBuilder {
    business_uuid: Option<String>,
    last_updated: Option<DateTime<Utc>>,
    links: Option<BusinessLinks>,
}

impl BusinessUpdatedResponseBuilder {
    pub fn business_uuid(mut self, value: impl Into<String>) -> Self {
        self.business_uuid = Some(value.into());
        self
    }

    pub fn last_updated(mut self, value: DateTime<Utc>) -> Self {
        self.last_updated = Some(value);
        self
    }

    pub fn links(mut self, value: BusinessLinks) -> Self {
        self.links = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BusinessUpdatedResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`business_uuid`](BusinessUpdatedResponseBuilder::business_uuid)
    /// - [`last_updated`](BusinessUpdatedResponseBuilder::last_updated)
    /// - [`links`](BusinessUpdatedResponseBuilder::links)
    pub fn build(self) -> Result<BusinessUpdatedResponse, BuildError> {
        Ok(BusinessUpdatedResponse {
            business_uuid: self
                .business_uuid
                .ok_or_else(|| BuildError::missing_field("business_uuid"))?,
            last_updated: self
                .last_updated
                .ok_or_else(|| BuildError::missing_field("last_updated"))?,
            links: self
                .links
                .ok_or_else(|| BuildError::missing_field("links"))?,
        })
    }
}
