pub use crate::prelude::*;

/// A person's link to a business, with the relationship's role, ownership, and relationship-scoped contact details.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BusinessPersonLink {
    /// The reference of the linked business.
    #[serde(rename = "businessReference")]
    #[serde(default)]
    pub business_reference: String,
    /// The reference of the linked person.
    #[serde(rename = "personReference")]
    #[serde(default)]
    pub person_reference: String,
    /// The person's role on the business, returned in lowercase (for example `owner`).
    #[serde(rename = "personType")]
    #[serde(default)]
    pub person_type: String,
    /// The person's ownership percentage (0 to 100). Present for owners.
    #[serde(rename = "ownershipPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_percentage: Option<f64>,
    /// Whether the person qualifies as a beneficial owner. Derived by Payabli.
    #[serde(rename = "beneficialOwner")]
    #[serde(default)]
    pub beneficial_owner: bool,
    /// Whether the person is the business's primary controller.
    #[serde(rename = "isPrimaryController")]
    #[serde(default)]
    pub is_primary_controller: bool,
    /// Whether the person signs the business's boarding applications.
    #[serde(rename = "isSigner")]
    #[serde(default)]
    pub is_signer: bool,
    /// The person's job title at this business.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// The relationship's primary email address.
    #[serde(rename = "primaryEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_email: Option<String>,
    /// The relationship's secondary email address.
    #[serde(rename = "secondaryEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_email: Option<String>,
    /// The relationship's primary phone number.
    #[serde(rename = "primaryPhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_phone_number: Option<String>,
    /// The relationship's secondary phone number.
    #[serde(rename = "secondaryPhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_phone_number: Option<String>,
    /// Reference to the stored billing address for the relationship. Present when a billing address was set.
    #[serde(rename = "billingAddressUuid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_uuid: Option<String>,
    /// Reference to the stored mailing address for the relationship. Present when a mailing address was set.
    #[serde(rename = "mailingAddressUuid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailing_address_uuid: Option<String>,
    /// When the link was created.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub created_at: DateTime<Utc>,
    /// When the link was last updated.
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub updated_at: DateTime<Utc>,
    /// Hypermedia links for the link resource.
    #[serde(rename = "_links")]
    #[serde(default)]
    pub links: BusinessPersonLinks,
}

impl BusinessPersonLink {
    pub fn builder() -> BusinessPersonLinkBuilder {
        <BusinessPersonLinkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BusinessPersonLinkBuilder {
    business_reference: Option<String>,
    person_reference: Option<String>,
    person_type: Option<String>,
    ownership_percentage: Option<f64>,
    beneficial_owner: Option<bool>,
    is_primary_controller: Option<bool>,
    is_signer: Option<bool>,
    title: Option<String>,
    primary_email: Option<String>,
    secondary_email: Option<String>,
    primary_phone_number: Option<String>,
    secondary_phone_number: Option<String>,
    billing_address_uuid: Option<String>,
    mailing_address_uuid: Option<String>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
    links: Option<BusinessPersonLinks>,
}

impl BusinessPersonLinkBuilder {
    pub fn business_reference(mut self, value: impl Into<String>) -> Self {
        self.business_reference = Some(value.into());
        self
    }

    pub fn person_reference(mut self, value: impl Into<String>) -> Self {
        self.person_reference = Some(value.into());
        self
    }

    pub fn person_type(mut self, value: impl Into<String>) -> Self {
        self.person_type = Some(value.into());
        self
    }

    pub fn ownership_percentage(mut self, value: f64) -> Self {
        self.ownership_percentage = Some(value);
        self
    }

    pub fn beneficial_owner(mut self, value: bool) -> Self {
        self.beneficial_owner = Some(value);
        self
    }

    pub fn is_primary_controller(mut self, value: bool) -> Self {
        self.is_primary_controller = Some(value);
        self
    }

    pub fn is_signer(mut self, value: bool) -> Self {
        self.is_signer = Some(value);
        self
    }

    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn primary_email(mut self, value: impl Into<String>) -> Self {
        self.primary_email = Some(value.into());
        self
    }

    pub fn secondary_email(mut self, value: impl Into<String>) -> Self {
        self.secondary_email = Some(value.into());
        self
    }

    pub fn primary_phone_number(mut self, value: impl Into<String>) -> Self {
        self.primary_phone_number = Some(value.into());
        self
    }

    pub fn secondary_phone_number(mut self, value: impl Into<String>) -> Self {
        self.secondary_phone_number = Some(value.into());
        self
    }

    pub fn billing_address_uuid(mut self, value: impl Into<String>) -> Self {
        self.billing_address_uuid = Some(value.into());
        self
    }

    pub fn mailing_address_uuid(mut self, value: impl Into<String>) -> Self {
        self.mailing_address_uuid = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<Utc>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn links(mut self, value: BusinessPersonLinks) -> Self {
        self.links = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BusinessPersonLink`].
    /// This method will fail if any of the following fields are not set:
    /// - [`business_reference`](BusinessPersonLinkBuilder::business_reference)
    /// - [`person_reference`](BusinessPersonLinkBuilder::person_reference)
    /// - [`person_type`](BusinessPersonLinkBuilder::person_type)
    /// - [`beneficial_owner`](BusinessPersonLinkBuilder::beneficial_owner)
    /// - [`is_primary_controller`](BusinessPersonLinkBuilder::is_primary_controller)
    /// - [`is_signer`](BusinessPersonLinkBuilder::is_signer)
    /// - [`created_at`](BusinessPersonLinkBuilder::created_at)
    /// - [`updated_at`](BusinessPersonLinkBuilder::updated_at)
    /// - [`links`](BusinessPersonLinkBuilder::links)
    pub fn build(self) -> Result<BusinessPersonLink, BuildError> {
        Ok(BusinessPersonLink {
            business_reference: self
                .business_reference
                .ok_or_else(|| BuildError::missing_field("business_reference"))?,
            person_reference: self
                .person_reference
                .ok_or_else(|| BuildError::missing_field("person_reference"))?,
            person_type: self
                .person_type
                .ok_or_else(|| BuildError::missing_field("person_type"))?,
            ownership_percentage: self.ownership_percentage,
            beneficial_owner: self
                .beneficial_owner
                .ok_or_else(|| BuildError::missing_field("beneficial_owner"))?,
            is_primary_controller: self
                .is_primary_controller
                .ok_or_else(|| BuildError::missing_field("is_primary_controller"))?,
            is_signer: self
                .is_signer
                .ok_or_else(|| BuildError::missing_field("is_signer"))?,
            title: self.title,
            primary_email: self.primary_email,
            secondary_email: self.secondary_email,
            primary_phone_number: self.primary_phone_number,
            secondary_phone_number: self.secondary_phone_number,
            billing_address_uuid: self.billing_address_uuid,
            mailing_address_uuid: self.mailing_address_uuid,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            links: self
                .links
                .ok_or_else(|| BuildError::missing_field("links"))?,
        })
    }
}
