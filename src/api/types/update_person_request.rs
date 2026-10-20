pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdatePersonRequest {
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
    /// The person's nationality, as a two-letter ISO 3166-1 alpha-2 country code. Nationality selects which identity number applies — `US` uses `ssn`, and any other value uses `nationalIdNumber` and `nationalIdIssuer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nationality: Option<String>,
    /// The authority that issued the national ID, used when `nationality` is not `US`.
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
    /// Key-value data to merge into the person's stored `customData`.
    /// Payabli stores an owner's answers to a boarding template's
    /// custom questions under the `templateAnswers` key.
    ///
    /// When you update a person, Payabli merges `customData` into the
    /// stored value, following JSON Merge Patch (RFC 7396). Keys you
    /// don't send are kept at every depth, and nested objects merge key
    /// by key. Strings, numbers, Boolean values, and arrays replace the
    /// stored value, and arrays aren't merged. Send a key as `null` to
    /// delete it, such as `{"crmId": null}`. A `customData` that isn't
    /// an object, such as an array, replaces the stored value whole.
    /// Leave out `customData` to keep it as is.
    ///
    /// The `customData` you send can be up to 65,536 bytes serialized.
    /// The limit applies to the request, not to the merged result.
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
}

impl UpdatePersonRequest {
    pub fn builder() -> UpdatePersonRequestBuilder {
        <UpdatePersonRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePersonRequestBuilder {
    first_name: Option<String>,
    middle_name: Option<String>,
    last_name: Option<String>,
    primary_email: Option<String>,
    secondary_email: Option<String>,
    primary_phone_number: Option<String>,
    secondary_phone_number: Option<String>,
    nationality: Option<String>,
    national_id_issuer: Option<String>,
    drivers_license_number: Option<String>,
    drivers_license_state: Option<String>,
    external_id: Option<String>,
    custom_data: Option<HashMap<String, serde_json::Value>>,
    addresses: Option<Vec<PersonAddressRequest>>,
    identification_documents: Option<Vec<IdentificationDocumentRequest>>,
}

impl UpdatePersonRequestBuilder {
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

    pub fn nationality(mut self, value: impl Into<String>) -> Self {
        self.nationality = Some(value.into());
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

    /// Consumes the builder and constructs a [`UpdatePersonRequest`].
    pub fn build(self) -> Result<UpdatePersonRequest, BuildError> {
        Ok(UpdatePersonRequest {
            first_name: self.first_name,
            middle_name: self.middle_name,
            last_name: self.last_name,
            primary_email: self.primary_email,
            secondary_email: self.secondary_email,
            primary_phone_number: self.primary_phone_number,
            secondary_phone_number: self.secondary_phone_number,
            nationality: self.nationality,
            national_id_issuer: self.national_id_issuer,
            drivers_license_number: self.drivers_license_number,
            drivers_license_state: self.drivers_license_state,
            external_id: self.external_id,
            custom_data: self.custom_data,
            addresses: self.addresses,
            identification_documents: self.identification_documents,
        })
    }
}
