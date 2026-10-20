pub use crate::prelude::*;

/// An address included in a person request.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PersonAddressRequest {
    /// The first line of the street address.
    #[serde(rename = "addressLine1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line_1: Option<String>,
    /// The second line of the street address (suite, unit, and so on).
    #[serde(rename = "addressLine2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line_2: Option<String>,
    /// The city or locality.
    #[serde(rename = "cityLocality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city_locality: Option<String>,
    /// The two-letter state or province code.
    #[serde(rename = "stateProvince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_province: Option<String>,
    /// The postal or ZIP code.
    #[serde(rename = "postalCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// The two-letter ISO 3166-1 alpha-2 country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The role the address plays for the person.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<PersonAddressType>,
}

impl PersonAddressRequest {
    pub fn builder() -> PersonAddressRequestBuilder {
        <PersonAddressRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PersonAddressRequestBuilder {
    address_line_1: Option<String>,
    address_line_2: Option<String>,
    city_locality: Option<String>,
    state_province: Option<String>,
    postal_code: Option<String>,
    country: Option<String>,
    r#type: Option<PersonAddressType>,
}

impl PersonAddressRequestBuilder {
    pub fn address_line_1(mut self, value: impl Into<String>) -> Self {
        self.address_line_1 = Some(value.into());
        self
    }

    pub fn address_line_2(mut self, value: impl Into<String>) -> Self {
        self.address_line_2 = Some(value.into());
        self
    }

    pub fn city_locality(mut self, value: impl Into<String>) -> Self {
        self.city_locality = Some(value.into());
        self
    }

    pub fn state_province(mut self, value: impl Into<String>) -> Self {
        self.state_province = Some(value.into());
        self
    }

    pub fn postal_code(mut self, value: impl Into<String>) -> Self {
        self.postal_code = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PersonAddressType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PersonAddressRequest`].
    pub fn build(self) -> Result<PersonAddressRequest, BuildError> {
        Ok(PersonAddressRequest {
            address_line_1: self.address_line_1,
            address_line_2: self.address_line_2,
            city_locality: self.city_locality,
            state_province: self.state_province,
            postal_code: self.postal_code,
            country: self.country,
            r#type: self.r#type,
        })
    }
}
