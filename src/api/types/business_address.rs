pub use crate::prelude::*;

/// An address associated with a business.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BusinessAddress {
    /// The address role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<AddressType>,
    /// The physical use of the address. Defaults to `other`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<BusinessAddressLocation>,
    /// The first line of the street address.
    #[serde(rename = "addressLine1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line_1: Option<String>,
    /// The second line of the street address.
    #[serde(rename = "addressLine2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line_2: Option<String>,
    /// The city or locality.
    #[serde(rename = "cityLocality")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city_locality: Option<String>,
    /// The state or province.
    #[serde(rename = "stateProvince")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_province: Option<String>,
    /// The postal or ZIP code.
    #[serde(rename = "postalCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// The two-character ISO country code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    /// The floor, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor: Option<String>,
    /// The suite or apartment number, if applicable.
    #[serde(rename = "suiteApartNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suite_apart_number: Option<String>,
}

impl BusinessAddress {
    pub fn builder() -> BusinessAddressBuilder {
        <BusinessAddressBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BusinessAddressBuilder {
    r#type: Option<AddressType>,
    location: Option<BusinessAddressLocation>,
    address_line_1: Option<String>,
    address_line_2: Option<String>,
    city_locality: Option<String>,
    state_province: Option<String>,
    postal_code: Option<String>,
    country: Option<String>,
    floor: Option<String>,
    suite_apart_number: Option<String>,
}

impl BusinessAddressBuilder {
    pub fn r#type(mut self, value: AddressType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn location(mut self, value: BusinessAddressLocation) -> Self {
        self.location = Some(value);
        self
    }

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

    pub fn floor(mut self, value: impl Into<String>) -> Self {
        self.floor = Some(value.into());
        self
    }

    pub fn suite_apart_number(mut self, value: impl Into<String>) -> Self {
        self.suite_apart_number = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BusinessAddress`].
    pub fn build(self) -> Result<BusinessAddress, BuildError> {
        Ok(BusinessAddress {
            r#type: self.r#type,
            location: self.location,
            address_line_1: self.address_line_1,
            address_line_2: self.address_line_2,
            city_locality: self.city_locality,
            state_province: self.state_province,
            postal_code: self.postal_code,
            country: self.country,
            floor: self.floor,
            suite_apart_number: self.suite_apart_number,
        })
    }
}
