pub use crate::prelude::*;

/// The result of deactivating a business.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BusinessDeactivatedResponse {
    /// The business reference.
    #[serde(rename = "businessUuid")]
    #[serde(default)]
    pub business_uuid: String,
    /// The business status after deactivation (Deactivated).
    #[serde(rename = "businessStatus")]
    #[serde(default)]
    pub business_status: String,
    /// When the business was deactivated.
    #[serde(rename = "deactivatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub deactivated_at: DateTime<Utc>,
    /// The reason recorded for the deactivation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Hypermedia links for the business resource.
    #[serde(rename = "_links")]
    #[serde(default)]
    pub links: BusinessLinks,
}

impl BusinessDeactivatedResponse {
    pub fn builder() -> BusinessDeactivatedResponseBuilder {
        <BusinessDeactivatedResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BusinessDeactivatedResponseBuilder {
    business_uuid: Option<String>,
    business_status: Option<String>,
    deactivated_at: Option<DateTime<Utc>>,
    reason: Option<String>,
    links: Option<BusinessLinks>,
}

impl BusinessDeactivatedResponseBuilder {
    pub fn business_uuid(mut self, value: impl Into<String>) -> Self {
        self.business_uuid = Some(value.into());
        self
    }

    pub fn business_status(mut self, value: impl Into<String>) -> Self {
        self.business_status = Some(value.into());
        self
    }

    pub fn deactivated_at(mut self, value: DateTime<Utc>) -> Self {
        self.deactivated_at = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn links(mut self, value: BusinessLinks) -> Self {
        self.links = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BusinessDeactivatedResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`business_uuid`](BusinessDeactivatedResponseBuilder::business_uuid)
    /// - [`business_status`](BusinessDeactivatedResponseBuilder::business_status)
    /// - [`deactivated_at`](BusinessDeactivatedResponseBuilder::deactivated_at)
    /// - [`links`](BusinessDeactivatedResponseBuilder::links)
    pub fn build(self) -> Result<BusinessDeactivatedResponse, BuildError> {
        Ok(BusinessDeactivatedResponse {
            business_uuid: self
                .business_uuid
                .ok_or_else(|| BuildError::missing_field("business_uuid"))?,
            business_status: self
                .business_status
                .ok_or_else(|| BuildError::missing_field("business_status"))?,
            deactivated_at: self
                .deactivated_at
                .ok_or_else(|| BuildError::missing_field("deactivated_at"))?,
            reason: self.reason,
            links: self
                .links
                .ok_or_else(|| BuildError::missing_field("links"))?,
        })
    }
}
