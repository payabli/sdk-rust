pub use crate::prelude::*;

/// The business-person relationship returned when a person is created with a
/// `businessRelationship`. Its contact fields are scoped to the relationship and
/// are distinct from the person's own top-level contact fields.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BusinessRelationshipResponse {
    /// The reference of the linked business.
    #[serde(rename = "businessReference")]
    #[serde(default)]
    pub business_reference: String,
    /// The person's role on the business, returned in lowercase (for example `owner`).
    #[serde(rename = "personType")]
    #[serde(default)]
    pub person_type: String,
    /// The person's ownership percentage (0 to 100).
    #[serde(rename = "ownershipPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub ownership_percentage: Option<f64>,
    /// Whether the person qualifies as a beneficial owner. Derived by Payabli, not sent on create.
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
    /// Reference to the stored billing address for the relationship. The address
    /// is sent as an object on create, then stored and returned by ID.
    #[serde(rename = "billingAddressUuid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_uuid: Option<String>,
    /// Reference to the stored mailing address for the relationship. The address
    /// is sent as an object on create, then stored and returned by ID.
    #[serde(rename = "mailingAddressUuid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailing_address_uuid: Option<String>,
}

impl BusinessRelationshipResponse {
    pub fn builder() -> BusinessRelationshipResponseBuilder {
        <BusinessRelationshipResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BusinessRelationshipResponseBuilder {
    business_reference: Option<String>,
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
}

impl BusinessRelationshipResponseBuilder {
    pub fn business_reference(mut self, value: impl Into<String>) -> Self {
        self.business_reference = Some(value.into());
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

    /// Consumes the builder and constructs a [`BusinessRelationshipResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`business_reference`](BusinessRelationshipResponseBuilder::business_reference)
    /// - [`person_type`](BusinessRelationshipResponseBuilder::person_type)
    /// - [`beneficial_owner`](BusinessRelationshipResponseBuilder::beneficial_owner)
    /// - [`is_primary_controller`](BusinessRelationshipResponseBuilder::is_primary_controller)
    /// - [`is_signer`](BusinessRelationshipResponseBuilder::is_signer)
    pub fn build(self) -> Result<BusinessRelationshipResponse, BuildError> {
        Ok(BusinessRelationshipResponse {
            business_reference: self
                .business_reference
                .ok_or_else(|| BuildError::missing_field("business_reference"))?,
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
        })
    }
}
