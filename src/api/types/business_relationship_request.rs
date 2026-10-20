pub use crate::prelude::*;

/// Links the person to a business as part of a person-create request. The
/// contact fields here are scoped to the business-person relationship and are
/// distinct from the person's own top-level `primaryEmail`, `primaryPhoneNumber`,
/// and so on.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BusinessRelationshipRequest {
    /// The reference of the business to link the person to.
    #[serde(rename = "businessReference")]
    #[serde(default)]
    pub business_reference: String,
    /// The person's role on the business.
    #[serde(rename = "personType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_type: Option<PersonType>,
    /// The person's ownership percentage (0 to 100). Required when `personType`
    /// is `Owner`, and must be omitted for every other role. If linking the
    /// owner would push the business over 100% total ownership, the person is
    /// still created but the link doesn't complete. The response has `success`
    /// set to `false`, omits `businessRelationship` from the person, and
    /// returns a `LINK_OWNERSHIP_EXCEEDED` entry in `warnings`.
    #[serde(rename = "ownershipPercentage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub ownership_percentage: Option<f64>,
    /// Whether the person is the business's primary controller.
    #[serde(rename = "isPrimaryController")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_primary_controller: Option<bool>,
    /// Whether the person signs the business's boarding applications. Only an
    /// owner can be the signer. Setting `isSigner` to `true` on a person with
    /// another `personType` returns `400` with the message
    /// `Only an owner can be the signer.` A business has one signer. Setting
    /// `isSigner` to `true` on one person removes it from every other person
    /// linked to the business.
    #[serde(rename = "isSigner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_signer: Option<bool>,
    /// The person's job title at this business, such as `CEO` or
    /// `Managing Member`, from 1 to 100 characters. A person linked to more than
    /// one business can have a different title at each one. A blank value is
    /// treated as not sent.
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
    /// The billing address for the relationship.
    #[serde(rename = "billingAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<PersonAddressRequest>,
    /// The mailing address for the relationship.
    #[serde(rename = "mailingAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mailing_address: Option<PersonAddressRequest>,
}

impl BusinessRelationshipRequest {
    pub fn builder() -> BusinessRelationshipRequestBuilder {
        <BusinessRelationshipRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BusinessRelationshipRequestBuilder {
    business_reference: Option<String>,
    person_type: Option<PersonType>,
    ownership_percentage: Option<f64>,
    is_primary_controller: Option<bool>,
    is_signer: Option<bool>,
    title: Option<String>,
    primary_email: Option<String>,
    secondary_email: Option<String>,
    primary_phone_number: Option<String>,
    secondary_phone_number: Option<String>,
    billing_address: Option<PersonAddressRequest>,
    mailing_address: Option<PersonAddressRequest>,
}

impl BusinessRelationshipRequestBuilder {
    pub fn business_reference(mut self, value: impl Into<String>) -> Self {
        self.business_reference = Some(value.into());
        self
    }

    pub fn person_type(mut self, value: PersonType) -> Self {
        self.person_type = Some(value);
        self
    }

    pub fn ownership_percentage(mut self, value: f64) -> Self {
        self.ownership_percentage = Some(value);
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

    pub fn billing_address(mut self, value: PersonAddressRequest) -> Self {
        self.billing_address = Some(value);
        self
    }

    pub fn mailing_address(mut self, value: PersonAddressRequest) -> Self {
        self.mailing_address = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BusinessRelationshipRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`business_reference`](BusinessRelationshipRequestBuilder::business_reference)
    pub fn build(self) -> Result<BusinessRelationshipRequest, BuildError> {
        Ok(BusinessRelationshipRequest {
            business_reference: self
                .business_reference
                .ok_or_else(|| BuildError::missing_field("business_reference"))?,
            person_type: self.person_type,
            ownership_percentage: self.ownership_percentage,
            is_primary_controller: self.is_primary_controller,
            is_signer: self.is_signer,
            title: self.title,
            primary_email: self.primary_email,
            secondary_email: self.secondary_email,
            primary_phone_number: self.primary_phone_number,
            secondary_phone_number: self.secondary_phone_number,
            billing_address: self.billing_address,
            mailing_address: self.mailing_address,
        })
    }
}
