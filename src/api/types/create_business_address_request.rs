pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CreateBusinessAddressRequest {
    /// The address role.
    pub r#type: AddressType,
    /// The physical use of the address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<BusinessAddressLocation>,
    /// The first line of the street address.
    #[serde(rename = "addressLine1")]
    #[serde(default)]
    pub address_line_1: String,
    /// The second line of the street address.
    #[serde(rename = "addressLine2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line_2: Option<String>,
    /// The city or locality.
    #[serde(rename = "cityLocality")]
    #[serde(default)]
    pub city_locality: String,
    /// The state or province.
    #[serde(rename = "stateProvince")]
    #[serde(default)]
    pub state_province: String,
    /// The postal or ZIP code.
    #[serde(rename = "postalCode")]
    #[serde(default)]
    pub postal_code: String,
    /// The two-character ISO country code.
    #[serde(default)]
    pub country: String,
    /// The floor, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub floor: Option<String>,
    /// The suite or apartment number, if applicable.
    #[serde(rename = "suiteApartNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suite_apart_number: Option<String>,
}

impl CreateBusinessAddressRequest {
    pub fn builder() -> CreateBusinessAddressRequestBuilder {
        <CreateBusinessAddressRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateBusinessAddressRequestBuilder {
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

impl CreateBusinessAddressRequestBuilder {
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

    /// Consumes the builder and constructs a [`CreateBusinessAddressRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](CreateBusinessAddressRequestBuilder::r#type)
    /// - [`address_line_1`](CreateBusinessAddressRequestBuilder::address_line_1)
    /// - [`city_locality`](CreateBusinessAddressRequestBuilder::city_locality)
    /// - [`state_province`](CreateBusinessAddressRequestBuilder::state_province)
    /// - [`postal_code`](CreateBusinessAddressRequestBuilder::postal_code)
    /// - [`country`](CreateBusinessAddressRequestBuilder::country)
    pub fn build(self) -> Result<CreateBusinessAddressRequest, BuildError> {
        Ok(CreateBusinessAddressRequest {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            location: self.location,
            address_line_1: self
                .address_line_1
                .ok_or_else(|| BuildError::missing_field("address_line_1"))?,
            address_line_2: self.address_line_2,
            city_locality: self
                .city_locality
                .ok_or_else(|| BuildError::missing_field("city_locality"))?,
            state_province: self
                .state_province
                .ok_or_else(|| BuildError::missing_field("state_province"))?,
            postal_code: self
                .postal_code
                .ok_or_else(|| BuildError::missing_field("postal_code"))?,
            country: self
                .country
                .ok_or_else(|| BuildError::missing_field("country"))?,
            floor: self.floor,
            suite_apart_number: self.suite_apart_number,
        })
    }
}
