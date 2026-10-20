pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdateBusinessPersonRequest {
    /// The person's role on the business.
    #[serde(rename = "personType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_type: Option<PersonType>,
    /// The person's ownership percentage (0 to 100).
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
    /// `Only an owner can be the signer.` Changing the signer's `personType`
    /// away from `Owner` returns the same error. Setting it to `true` removes
    /// it from every other person linked to the business. Setting it to
    /// `false` leaves the business with no signer until you set another one.
    #[serde(rename = "isSigner")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_signer: Option<bool>,
    /// The person's job title at this business, from 1 to 100 characters. Updating it changes the title
    /// for this business only. Omitting it, or sending `null` or a blank value,
    /// keeps the stored title. You can replace a title, but you can't clear it.
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

impl UpdateBusinessPersonRequest {
    pub fn builder() -> UpdateBusinessPersonRequestBuilder {
        <UpdateBusinessPersonRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdateBusinessPersonRequestBuilder {
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

impl UpdateBusinessPersonRequestBuilder {
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

    /// Consumes the builder and constructs a [`UpdateBusinessPersonRequest`].
    pub fn build(self) -> Result<UpdateBusinessPersonRequest, BuildError> {
        Ok(UpdateBusinessPersonRequest {
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
