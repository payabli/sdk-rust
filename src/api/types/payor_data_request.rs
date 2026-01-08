pub use crate::prelude::*;

/// Customer information. May be required, depending on the paypoint's settings. Required for subscriptions.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PayorDataRequest {
    #[serde(rename = "additionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalData>,
    #[serde(rename = "billingAddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_1: Option<BillingAddressNullable>,
    #[serde(rename = "billingAddress2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_2: Option<BillingAddressAddtlNullable>,
    #[serde(rename = "billingCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_city: Option<BillingCityNullable>,
    #[serde(rename = "billingCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_country: Option<BillingCountryNullable>,
    #[serde(rename = "billingEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_email: Option<Email>,
    #[serde(rename = "billingPhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_phone: Option<PhoneNumber>,
    #[serde(rename = "billingState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_state: Option<BillingStateNullable>,
    /// Customer's billing ZIP code. For Pay In functions, this field supports 5-digit and 9-digit ZIP codes and alphanumeric Canadian postal codes. For example: "37615-1234" or "37615".
    #[serde(rename = "billingZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_zip: Option<BillingZip>,
    /// Customer's company name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<CustomerId>,
    #[serde(rename = "customerNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_number: Option<CustomerNumberNullable>,
    /// Customer/Payor first name.
    #[serde(rename = "firstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "identifierFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier_fields: Option<Identifierfields>,
    /// Customer/Payor last name.
    #[serde(rename = "lastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(rename = "shippingAddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_1: Option<Shippingaddress>,
    #[serde(rename = "shippingAddress2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_2: Option<Shippingaddressadditional>,
    #[serde(rename = "shippingCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_city: Option<Shippingcity>,
    #[serde(rename = "shippingCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_country: Option<Shippingcountry>,
    #[serde(rename = "shippingState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_state: Option<Shippingstate>,
    #[serde(rename = "shippingZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_zip: Option<Shippingzip>,
}
