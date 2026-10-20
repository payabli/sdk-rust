pub use crate::prelude::*;

/// The reference and initial status of a newly created business.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BusinessCreatedResponse {
    /// The business reference, used to read and manage the business.
    #[serde(rename = "businessUuid")]
    #[serde(default)]
    pub business_uuid: String,
    /// The business status. New businesses start in Draft.
    #[serde(rename = "businessStatus")]
    #[serde(default)]
    pub business_status: String,
    /// When the business was created.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub created_at: DateTime<Utc>,
    /// Hypermedia links for the business resource.
    #[serde(rename = "_links")]
    #[serde(default)]
    pub links: BusinessLinks,
}

impl BusinessCreatedResponse {
    pub fn builder() -> BusinessCreatedResponseBuilder {
        <BusinessCreatedResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BusinessCreatedResponseBuilder {
    business_uuid: Option<String>,
    business_status: Option<String>,
    created_at: Option<DateTime<Utc>>,
    links: Option<BusinessLinks>,
}

impl BusinessCreatedResponseBuilder {
    pub fn business_uuid(mut self, value: impl Into<String>) -> Self {
        self.business_uuid = Some(value.into());
        self
    }

    pub fn business_status(mut self, value: impl Into<String>) -> Self {
        self.business_status = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn links(mut self, value: BusinessLinks) -> Self {
        self.links = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BusinessCreatedResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`business_uuid`](BusinessCreatedResponseBuilder::business_uuid)
    /// - [`business_status`](BusinessCreatedResponseBuilder::business_status)
    /// - [`created_at`](BusinessCreatedResponseBuilder::created_at)
    /// - [`links`](BusinessCreatedResponseBuilder::links)
    pub fn build(self) -> Result<BusinessCreatedResponse, BuildError> {
        Ok(BusinessCreatedResponse {
            business_uuid: self
                .business_uuid
                .ok_or_else(|| BuildError::missing_field("business_uuid"))?,
            business_status: self
                .business_status
                .ok_or_else(|| BuildError::missing_field("business_status"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            links: self
                .links
                .ok_or_else(|| BuildError::missing_field("links"))?,
        })
    }
}
