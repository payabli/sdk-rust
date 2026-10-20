pub use crate::prelude::*;

/// A business address, returned by the address endpoints.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BusinessAddressResponse {
    /// Hypermedia links for the business address resource.
    #[serde(rename = "_links")]
    #[serde(default)]
    pub links: BusinessAddressLinks,
    /// The reference of the business-to-address link, used to read and manage the address. Valid only under its own business.
    #[serde(rename = "businessAddressReference")]
    #[serde(default)]
    pub business_address_reference: String,
    /// The address role.
    pub r#type: AddressType,
    /// The physical use of the address.
    pub location: BusinessAddressLocation,
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

impl BusinessAddressResponse {
    pub fn builder() -> BusinessAddressResponseBuilder {
        <BusinessAddressResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BusinessAddressResponseBuilder {
    links: Option<BusinessAddressLinks>,
    business_address_reference: Option<String>,
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

impl BusinessAddressResponseBuilder {
    pub fn links(mut self, value: BusinessAddressLinks) -> Self {
        self.links = Some(value);
        self
    }

    pub fn business_address_reference(mut self, value: impl Into<String>) -> Self {
        self.business_address_reference = Some(value.into());
        self
    }

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

    /// Consumes the builder and constructs a [`BusinessAddressResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`links`](BusinessAddressResponseBuilder::links)
    /// - [`business_address_reference`](BusinessAddressResponseBuilder::business_address_reference)
    /// - [`r#type`](BusinessAddressResponseBuilder::r#type)
    /// - [`location`](BusinessAddressResponseBuilder::location)
    /// - [`address_line_1`](BusinessAddressResponseBuilder::address_line_1)
    /// - [`city_locality`](BusinessAddressResponseBuilder::city_locality)
    /// - [`state_province`](BusinessAddressResponseBuilder::state_province)
    /// - [`postal_code`](BusinessAddressResponseBuilder::postal_code)
    /// - [`country`](BusinessAddressResponseBuilder::country)
    pub fn build(self) -> Result<BusinessAddressResponse, BuildError> {
        Ok(BusinessAddressResponse {
            links: self
                .links
                .ok_or_else(|| BuildError::missing_field("links"))?,
            business_address_reference: self
                .business_address_reference
                .ok_or_else(|| BuildError::missing_field("business_address_reference"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            location: self
                .location
                .ok_or_else(|| BuildError::missing_field("location"))?,
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
