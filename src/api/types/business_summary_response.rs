pub use crate::prelude::*;

/// A business as it appears in a list response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BusinessSummaryResponse {
    /// The business reference.
    #[serde(rename = "businessUuid")]
    #[serde(default)]
    pub business_uuid: String,
    /// The registered legal name of the business.
    #[serde(rename = "legalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub legal_name: Option<String>,
    /// The business's current status.
    #[serde(rename = "businessStatus")]
    #[serde(default)]
    pub business_status: String,
    /// The organization entrypoint the business belongs to.
    #[serde(rename = "parentReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_reference: Option<String>,
    /// When the business was created.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub created_at: DateTime<Utc>,
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

impl BusinessSummaryResponse {
    pub fn builder() -> BusinessSummaryResponseBuilder {
        <BusinessSummaryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BusinessSummaryResponseBuilder {
    business_uuid: Option<String>,
    legal_name: Option<String>,
    business_status: Option<String>,
    parent_reference: Option<String>,
    created_at: Option<DateTime<Utc>>,
    last_updated: Option<DateTime<Utc>>,
    links: Option<BusinessLinks>,
}

impl BusinessSummaryResponseBuilder {
    pub fn business_uuid(mut self, value: impl Into<String>) -> Self {
        self.business_uuid = Some(value.into());
        self
    }

    pub fn legal_name(mut self, value: impl Into<String>) -> Self {
        self.legal_name = Some(value.into());
        self
    }

    pub fn business_status(mut self, value: impl Into<String>) -> Self {
        self.business_status = Some(value.into());
        self
    }

    pub fn parent_reference(mut self, value: impl Into<String>) -> Self {
        self.parent_reference = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<Utc>) -> Self {
        self.created_at = Some(value);
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

    /// Consumes the builder and constructs a [`BusinessSummaryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`business_uuid`](BusinessSummaryResponseBuilder::business_uuid)
    /// - [`business_status`](BusinessSummaryResponseBuilder::business_status)
    /// - [`created_at`](BusinessSummaryResponseBuilder::created_at)
    /// - [`last_updated`](BusinessSummaryResponseBuilder::last_updated)
    /// - [`links`](BusinessSummaryResponseBuilder::links)
    pub fn build(self) -> Result<BusinessSummaryResponse, BuildError> {
        Ok(BusinessSummaryResponse {
            business_uuid: self
                .business_uuid
                .ok_or_else(|| BuildError::missing_field("business_uuid"))?,
            legal_name: self.legal_name,
            business_status: self
                .business_status
                .ok_or_else(|| BuildError::missing_field("business_status"))?,
            parent_reference: self.parent_reference,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            last_updated: self
                .last_updated
                .ok_or_else(|| BuildError::missing_field("last_updated"))?,
            links: self
                .links
                .ok_or_else(|| BuildError::missing_field("links"))?,
        })
    }
}
