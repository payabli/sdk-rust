pub use crate::prelude::*;

/// The result of removing a business address.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BusinessAddressDeletedResponse {
    /// Hypermedia links for the business resource.
    #[serde(rename = "_links")]
    #[serde(default)]
    pub links: BusinessLinks,
    /// The business reference.
    #[serde(rename = "businessUuid")]
    #[serde(default)]
    pub business_uuid: String,
    /// The reference of the removed business address.
    #[serde(rename = "businessAddressReference")]
    #[serde(default)]
    pub business_address_reference: String,
    /// When the address was removed.
    #[serde(rename = "deletedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub deleted_at: DateTime<Utc>,
}

impl BusinessAddressDeletedResponse {
    pub fn builder() -> BusinessAddressDeletedResponseBuilder {
        <BusinessAddressDeletedResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BusinessAddressDeletedResponseBuilder {
    links: Option<BusinessLinks>,
    business_uuid: Option<String>,
    business_address_reference: Option<String>,
    deleted_at: Option<DateTime<Utc>>,
}

impl BusinessAddressDeletedResponseBuilder {
    pub fn links(mut self, value: BusinessLinks) -> Self {
        self.links = Some(value);
        self
    }

    pub fn business_uuid(mut self, value: impl Into<String>) -> Self {
        self.business_uuid = Some(value.into());
        self
    }

    pub fn business_address_reference(mut self, value: impl Into<String>) -> Self {
        self.business_address_reference = Some(value.into());
        self
    }

    pub fn deleted_at(mut self, value: DateTime<Utc>) -> Self {
        self.deleted_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BusinessAddressDeletedResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`links`](BusinessAddressDeletedResponseBuilder::links)
    /// - [`business_uuid`](BusinessAddressDeletedResponseBuilder::business_uuid)
    /// - [`business_address_reference`](BusinessAddressDeletedResponseBuilder::business_address_reference)
    /// - [`deleted_at`](BusinessAddressDeletedResponseBuilder::deleted_at)
    pub fn build(self) -> Result<BusinessAddressDeletedResponse, BuildError> {
        Ok(BusinessAddressDeletedResponse {
            links: self
                .links
                .ok_or_else(|| BuildError::missing_field("links"))?,
            business_uuid: self
                .business_uuid
                .ok_or_else(|| BuildError::missing_field("business_uuid"))?,
            business_address_reference: self
                .business_address_reference
                .ok_or_else(|| BuildError::missing_field("business_address_reference"))?,
            deleted_at: self
                .deleted_at
                .ok_or_else(|| BuildError::missing_field("deleted_at"))?,
        })
    }
}
