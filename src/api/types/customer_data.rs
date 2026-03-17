pub use crate::prelude::*;

/// Data about a single customer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    pub balance: Option<f64>,
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<Timezone>,
    /// Additional Custom fields in format "key":"value".
    #[serde(rename = "additionalFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_fields: Option<HashMap<String, Option<String>>>,
    #[serde(rename = "identifierFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier_fields: Option<Identifierfields>,
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
}
