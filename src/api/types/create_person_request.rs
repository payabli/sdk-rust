pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreatePersonRequest {
    /// The person's first name.
    #[serde(rename = "firstName")]
    #[serde(default)]
    pub first_name: String,
    /// The person's middle name.
    #[serde(rename = "middleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    /// The person's last name.
    #[serde(rename = "lastName")]
    #[serde(default)]
    pub last_name: String,
    /// The person's primary email address.
    #[serde(rename = "primaryEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_email: Option<String>,
    /// The person's secondary email address.
    #[serde(rename = "secondaryEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_email: Option<String>,
    /// The person's primary phone number.
    #[serde(rename = "primaryPhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_phone_number: Option<String>,
    /// The person's secondary phone number.
    #[serde(rename = "secondaryPhoneNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_phone_number: Option<String>,
    /// The person's date of birth (`YYYY-MM-DD`).
    #[serde(rename = "dateOfBirth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// The person's US Social Security Number, in `XXX-XX-XXXX` or `XXXXXXXXX` format. Applies when `nationality` is `US`, and needed to validate a US owner. Create doesn't enforce it. For any other nationality, send `nationalIdNumber` and `nationalIdIssuer` instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn: Option<String>,
    /// The person's nationality, as a two-letter ISO 3166-1 alpha-2 country code. Needed to validate an owner, but create doesn't enforce it. Nationality selects which identity number applies. `US` uses `ssn`, and any other value uses `nationalIdNumber` and `nationalIdIssuer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    /// The person's national identification number, for an individual whose `nationality` is not `US`. Send it with `nationalIdIssuer` when `nationality` is non-US. Create doesn't enforce it.
    #[serde(rename = "nationalIdNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_id_number: Option<String>,
    /// The authority that issued the national ID, for example the two-letter country code. Send it with `nationalIdNumber` when `nationality` is non-US. Create doesn't enforce it.
    #[serde(rename = "nationalIdIssuer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_id_issuer: Option<String>,
    /// The person's driver's license number.
    #[serde(rename = "driversLicenseNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drivers_license_number: Option<String>,
    /// The two-letter state code for the driver's license.
    #[serde(rename = "driversLicenseState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drivers_license_state: Option<String>,
    /// Your own identifier for the person.
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Arbitrary key-value data you attach to the person. Payabli
    /// stores an owner's answers to a boarding template's custom
    /// questions under the `templateAnswers` key.
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<HashMap<String, serde_json::Value>>,
    /// Postal addresses associated with the person.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<PersonAddressRequest>>,
    /// Government-issued identification documents for the person.
    #[serde(rename = "identificationDocuments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification_documents: Option<Vec<IdentificationDocumentRequest>>,
    /// Links the person to a business as you create them. The link's role,
    /// ownership, primary-controller flag, contact fields, and addresses go here.
    /// These relationship contact fields are separate from the person's own
    /// top-level contact fields above.
    #[serde(rename = "businessRelationship")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_relationship: Option<BusinessRelationshipRequest>,
}

impl CreatePersonRequest {
    pub fn builder() -> CreatePersonRequestBuilder {
        <CreatePersonRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePersonRequestBuilder {
    first_name: Option<String>,
    middle_name: Option<String>,
    last_name: Option<String>,
    primary_email: Option<String>,
    secondary_email: Option<String>,
    primary_phone_number: Option<String>,
    secondary_phone_number: Option<String>,
    date_of_birth: Option<String>,
    ssn: Option<String>,
    nationality: Option<String>,
    national_id_number: Option<String>,
    national_id_issuer: Option<String>,
    drivers_license_number: Option<String>,
    drivers_license_state: Option<String>,
    external_id: Option<String>,
    custom_data: Option<HashMap<String, serde_json::Value>>,
    addresses: Option<Vec<PersonAddressRequest>>,
    identification_documents: Option<Vec<IdentificationDocumentRequest>>,
    business_relationship: Option<BusinessRelationshipRequest>,
}

impl CreatePersonRequestBuilder {
    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn middle_name(mut self, value: impl Into<String>) -> Self {
        self.middle_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
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

    pub fn date_of_birth(mut self, value: impl Into<String>) -> Self {
        self.date_of_birth = Some(value.into());
        self
    }

    pub fn ssn(mut self, value: impl Into<String>) -> Self {
        self.ssn = Some(value.into());
        self
    }

    pub fn nationality(mut self, value: impl Into<String>) -> Self {
        self.nationality = Some(value.into());
        self
    }

    pub fn national_id_number(mut self, value: impl Into<String>) -> Self {
        self.national_id_number = Some(value.into());
        self
    }

    pub fn national_id_issuer(mut self, value: impl Into<String>) -> Self {
        self.national_id_issuer = Some(value.into());
        self
    }

    pub fn drivers_license_number(mut self, value: impl Into<String>) -> Self {
        self.drivers_license_number = Some(value.into());
        self
    }

    pub fn drivers_license_state(mut self, value: impl Into<String>) -> Self {
        self.drivers_license_state = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn custom_data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.custom_data = Some(value);
        self
    }

    pub fn addresses(mut self, value: Vec<PersonAddressRequest>) -> Self {
        self.addresses = Some(value);
        self
    }

    pub fn identification_documents(mut self, value: Vec<IdentificationDocumentRequest>) -> Self {
        self.identification_documents = Some(value);
        self
    }

    pub fn business_relationship(mut self, value: BusinessRelationshipRequest) -> Self {
        self.business_relationship = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePersonRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`first_name`](CreatePersonRequestBuilder::first_name)
    /// - [`last_name`](CreatePersonRequestBuilder::last_name)
    pub fn build(self) -> Result<CreatePersonRequest, BuildError> {
        Ok(CreatePersonRequest {
            first_name: self
                .first_name
                .ok_or_else(|| BuildError::missing_field("first_name"))?,
            middle_name: self.middle_name,
            last_name: self
                .last_name
                .ok_or_else(|| BuildError::missing_field("last_name"))?,
            primary_email: self.primary_email,
            secondary_email: self.secondary_email,
            primary_phone_number: self.primary_phone_number,
            secondary_phone_number: self.secondary_phone_number,
            date_of_birth: self.date_of_birth,
            ssn: self.ssn,
            nationality: self.nationality,
            national_id_number: self.national_id_number,
            national_id_issuer: self.national_id_issuer,
            drivers_license_number: self.drivers_license_number,
            drivers_license_state: self.drivers_license_state,
            external_id: self.external_id,
            custom_data: self.custom_data,
            addresses: self.addresses,
            identification_documents: self.identification_documents,
            business_relationship: self.business_relationship,
        })
    }
}
