pub use crate::prelude::*;

/// The full detail of a person. Sensitive fields are masked unless read from the unmasked endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PersonDetailResponse {
    /// The person reference.
    #[serde(rename = "personReference")]
    #[serde(default)]
    pub person_reference: String,
    /// Your own identifier for the person.
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// The person's first name.
    #[serde(rename = "firstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// The person's middle name.
    #[serde(rename = "middleName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    /// The person's last name.
    #[serde(rename = "lastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
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
    /// The person's date of birth. Masked on standard reads (for example `****-**-14`); read the unmasked endpoint for the raw value.
    #[serde(rename = "dateOfBirth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// The person's US Social Security Number. Populated when `nationality` is `US`, otherwise null. Masked on standard reads (for example `***-**-9021`); read the unmasked endpoint for the raw value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssn: Option<String>,
    /// The person's nationality, as a two-letter ISO 3166-1 alpha-2 country code. Not masked. When `US`, `ssn` is populated and the national ID fields are null; for any other value, `nationalIdNumber` and `nationalIdIssuer` are populated and `ssn` is null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    /// The person's national identification number. Populated when `nationality` is not `US`, otherwise null. Masked on standard reads; read the unmasked endpoint for the raw value.
    #[serde(rename = "nationalIdNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_id_number: Option<String>,
    /// The authority that issued the national ID, for example the two-letter country code. Populated when `nationality` is not `US`, otherwise null.
    #[serde(rename = "nationalIdIssuer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub national_id_issuer: Option<String>,
    /// The person's driver's license number. Masked on standard reads (for example `D1***567`); read the unmasked endpoint for the raw value.
    #[serde(rename = "driversLicenseNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drivers_license_number: Option<String>,
    /// The two-letter state code for the driver's license.
    #[serde(rename = "driversLicenseState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub drivers_license_state: Option<String>,
    /// Arbitrary key-value data attached to the person. When present,
    /// `templateAnswers` holds this owner's answers to a boarding
    /// template's person-scoped custom questions, meaning questions
    /// whose field path starts with `person.`, such as
    /// `person.backgroundCheck`. Payabli saves them on each owner. It's
    /// an object keyed by field path, and each answer is a string. A
    /// multi-select answer is one comma-separated string.
    ///
    /// Payabli saves the answers when an application is submitted
    /// through the hosted boarding application. Submitting an
    /// application with the API
    /// (`POST /v2/requests/{requestsReference}/submit`) doesn't save them.
    /// Only answers to custom questions are saved, and blank answers,
    /// standard-field answers, and document uploads aren't. A later
    /// hosted submit replaces the answer to the same question and keeps
    /// every other answer, including answers from other applications.
    /// Leaving a question blank on a later submit doesn't remove the
    /// stored answer.
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<HashMap<String, serde_json::Value>>,
    /// The person's identity verification status, for example `not_started`.
    #[serde(rename = "verificationStatus")]
    #[serde(default)]
    pub verification_status: String,
    /// Postal addresses associated with the person.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub addresses: Option<Vec<PersonAddressResponse>>,
    /// Government-issued identification documents for the person.
    #[serde(rename = "identificationDocuments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identification_documents: Option<Vec<IdentificationDocumentResponse>>,
    /// The business link created with the person, present when the person was
    /// created with a `businessRelationship`. Null on a standalone person read,
    /// since the relationship is scoped to the business rather than the shared
    /// person.
    #[serde(rename = "businessRelationship")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_relationship: Option<BusinessRelationshipResponse>,
    /// When the person was created.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub created_at: DateTime<Utc>,
    /// When the person was last updated.
    #[serde(rename = "lastUpdated")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub last_updated: DateTime<Utc>,
    /// Hypermedia links for the person resource.
    #[serde(rename = "_links")]
    #[serde(default)]
    pub links: PersonLinks,
}

impl PersonDetailResponse {
    pub fn builder() -> PersonDetailResponseBuilder {
        <PersonDetailResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PersonDetailResponseBuilder {
    person_reference: Option<String>,
    external_id: Option<String>,
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
    custom_data: Option<HashMap<String, serde_json::Value>>,
    verification_status: Option<String>,
    addresses: Option<Vec<PersonAddressResponse>>,
    identification_documents: Option<Vec<IdentificationDocumentResponse>>,
    business_relationship: Option<BusinessRelationshipResponse>,
    created_at: Option<DateTime<Utc>>,
    last_updated: Option<DateTime<Utc>>,
    links: Option<PersonLinks>,
}

impl PersonDetailResponseBuilder {
    pub fn person_reference(mut self, value: impl Into<String>) -> Self {
        self.person_reference = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

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

    pub fn custom_data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.custom_data = Some(value);
        self
    }

    pub fn verification_status(mut self, value: impl Into<String>) -> Self {
        self.verification_status = Some(value.into());
        self
    }

    pub fn addresses(mut self, value: Vec<PersonAddressResponse>) -> Self {
        self.addresses = Some(value);
        self
    }

    pub fn identification_documents(mut self, value: Vec<IdentificationDocumentResponse>) -> Self {
        self.identification_documents = Some(value);
        self
    }

    pub fn business_relationship(mut self, value: BusinessRelationshipResponse) -> Self {
        self.business_relationship = Some(value);
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

    pub fn links(mut self, value: PersonLinks) -> Self {
        self.links = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PersonDetailResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`person_reference`](PersonDetailResponseBuilder::person_reference)
    /// - [`verification_status`](PersonDetailResponseBuilder::verification_status)
    /// - [`created_at`](PersonDetailResponseBuilder::created_at)
    /// - [`last_updated`](PersonDetailResponseBuilder::last_updated)
    /// - [`links`](PersonDetailResponseBuilder::links)
    pub fn build(self) -> Result<PersonDetailResponse, BuildError> {
        Ok(PersonDetailResponse {
            person_reference: self
                .person_reference
                .ok_or_else(|| BuildError::missing_field("person_reference"))?,
            external_id: self.external_id,
            first_name: self.first_name,
            middle_name: self.middle_name,
            last_name: self.last_name,
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
            custom_data: self.custom_data,
            verification_status: self
                .verification_status
                .ok_or_else(|| BuildError::missing_field("verification_status"))?,
            addresses: self.addresses,
            identification_documents: self.identification_documents,
            business_relationship: self.business_relationship,
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
