pub use crate::prelude::*;

/// Data about a single customer.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CustomerData {
    #[serde(rename = "customerNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_number: Option<CustomerNumberNullable>,
    /// Customer username for customer portal
    #[serde(rename = "customerUsername")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_username: Option<String>,
    /// Customer password for customer portal
    #[serde(rename = "customerPsw")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_psw: Option<String>,
    #[serde(rename = "customerStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_status: Option<CustomerStatus>,
    /// Company name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    /// Customer first name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub firstname: Option<String>,
    /// Customer last name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lastname: Option<String>,
    /// Customer phone number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// Customer email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
    /// Customer address
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    /// Additional customer address
    #[serde(rename = "address1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_1: Option<String>,
    /// Customer city
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// Customer State
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Customer postal code
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
    /// Customer country in ISO-3166-1 alpha 2 format. See https://en.wikipedia.org/wiki/ISO_3166-1 for reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(rename = "shippingAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<Shippingaddress>,
    #[serde(rename = "shippingAddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_1: Option<Shippingaddressadditional>,
    #[serde(rename = "shippingCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_city: Option<Shippingcity>,
    #[serde(rename = "shippingState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_state: Option<Shippingstate>,
    #[serde(rename = "shippingZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_zip: Option<Shippingzip>,
    #[serde(rename = "shippingCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_country: Option<Shippingcountry>,
    /// Customer balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub balance: Option<f64>,
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<Timezone>,
    /// Additional Custom fields in format "key":"value".
    #[serde(rename = "additionalFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_fields: Option<HashMap<String, String>>,
    #[serde(rename = "identifierFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier_fields: Option<Identifierfields>,
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
}

impl CustomerData {
    pub fn builder() -> CustomerDataBuilder {
        <CustomerDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomerDataBuilder {
    customer_number: Option<CustomerNumberNullable>,
    customer_username: Option<String>,
    customer_psw: Option<String>,
    customer_status: Option<CustomerStatus>,
    company: Option<String>,
    firstname: Option<String>,
    lastname: Option<String>,
    phone: Option<String>,
    email: Option<Email>,
    address: Option<String>,
    address_1: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zip: Option<String>,
    country: Option<String>,
    shipping_address: Option<Shippingaddress>,
    shipping_address_1: Option<Shippingaddressadditional>,
    shipping_city: Option<Shippingcity>,
    shipping_state: Option<Shippingstate>,
    shipping_zip: Option<Shippingzip>,
    shipping_country: Option<Shippingcountry>,
    balance: Option<f64>,
    time_zone: Option<Timezone>,
    additional_fields: Option<HashMap<String, String>>,
    identifier_fields: Option<Identifierfields>,
    created_at: Option<CreatedAt>,
}

impl CustomerDataBuilder {
    pub fn customer_number(mut self, value: CustomerNumberNullable) -> Self {
        self.customer_number = Some(value);
        self
    }

    pub fn customer_username(mut self, value: impl Into<String>) -> Self {
        self.customer_username = Some(value.into());
        self
    }

    pub fn customer_psw(mut self, value: impl Into<String>) -> Self {
        self.customer_psw = Some(value.into());
        self
    }

    pub fn customer_status(mut self, value: CustomerStatus) -> Self {
        self.customer_status = Some(value);
        self
    }

    pub fn company(mut self, value: impl Into<String>) -> Self {
        self.company = Some(value.into());
        self
    }

    pub fn firstname(mut self, value: impl Into<String>) -> Self {
        self.firstname = Some(value.into());
        self
    }

    pub fn lastname(mut self, value: impl Into<String>) -> Self {
        self.lastname = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    pub fn email(mut self, value: Email) -> Self {
        self.email = Some(value);
        self
    }

    pub fn address(mut self, value: impl Into<String>) -> Self {
        self.address = Some(value.into());
        self
    }

    pub fn address_1(mut self, value: impl Into<String>) -> Self {
        self.address_1 = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn zip(mut self, value: impl Into<String>) -> Self {
        self.zip = Some(value.into());
        self
    }

    pub fn country(mut self, value: impl Into<String>) -> Self {
        self.country = Some(value.into());
        self
    }

    pub fn shipping_address(mut self, value: Shippingaddress) -> Self {
        self.shipping_address = Some(value);
        self
    }

    pub fn shipping_address_1(mut self, value: Shippingaddressadditional) -> Self {
        self.shipping_address_1 = Some(value);
        self
    }

    pub fn shipping_city(mut self, value: Shippingcity) -> Self {
        self.shipping_city = Some(value);
        self
    }

    pub fn shipping_state(mut self, value: Shippingstate) -> Self {
        self.shipping_state = Some(value);
        self
    }

    pub fn shipping_zip(mut self, value: Shippingzip) -> Self {
        self.shipping_zip = Some(value);
        self
    }

    pub fn shipping_country(mut self, value: Shippingcountry) -> Self {
        self.shipping_country = Some(value);
        self
    }

    pub fn balance(mut self, value: f64) -> Self {
        self.balance = Some(value);
        self
    }

    pub fn time_zone(mut self, value: Timezone) -> Self {
        self.time_zone = Some(value);
        self
    }

    pub fn additional_fields(mut self, value: HashMap<String, String>) -> Self {
        self.additional_fields = Some(value);
        self
    }

    pub fn identifier_fields(mut self, value: Identifierfields) -> Self {
        self.identifier_fields = Some(value);
        self
    }

    pub fn created_at(mut self, value: CreatedAt) -> Self {
        self.created_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CustomerData`].
    pub fn build(self) -> Result<CustomerData, BuildError> {
        Ok(CustomerData {
            customer_number: self.customer_number,
            customer_username: self.customer_username,
            customer_psw: self.customer_psw,
            customer_status: self.customer_status,
            company: self.company,
            firstname: self.firstname,
            lastname: self.lastname,
            phone: self.phone,
            email: self.email,
            address: self.address,
            address_1: self.address_1,
            city: self.city,
            state: self.state,
            zip: self.zip,
            country: self.country,
            shipping_address: self.shipping_address,
            shipping_address_1: self.shipping_address_1,
            shipping_city: self.shipping_city,
            shipping_state: self.shipping_state,
            shipping_zip: self.shipping_zip,
            shipping_country: self.shipping_country,
            balance: self.balance,
            time_zone: self.time_zone,
            additional_fields: self.additional_fields,
            identifier_fields: self.identifier_fields,
            created_at: self.created_at,
        })
    }
}
