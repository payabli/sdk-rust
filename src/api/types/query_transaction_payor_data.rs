pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryTransactionPayorData {
    /// Array of field names to be used as identifiers.
    #[serde(rename = "Identifiers")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<Vec<serde_json::Value>>,
    /// Customer/Payor first name.
    #[serde(rename = "FirstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// Customer/Payor last name.
    #[serde(rename = "LastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Customer's company name.
    #[serde(rename = "CompanyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company_name: Option<String>,
    /// Customer's billing address.
    #[serde(rename = "BillingAddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_1: Option<String>,
    /// Additional line for Customer's billing address.
    #[serde(rename = "BillingAddress2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_2: Option<String>,
    /// Customer's billing city.
    #[serde(rename = "BillingCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_city: Option<String>,
    /// Customer's billing state. Must be 2-letter state code for address in US.
    #[serde(rename = "BillingState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_state: Option<String>,
    /// Customer's billing ZIP code.
    #[serde(rename = "BillingZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_zip: Option<BillingZip>,
    /// Customer's billing country.
    #[serde(rename = "BillingCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_country: Option<String>,
    /// Customer's phone number.
    #[serde(rename = "BillingPhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_phone: Option<String>,
    /// Customer's email address.
    #[serde(rename = "BillingEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_email: Option<Email>,
    #[serde(rename = "CustomerNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_number: Option<CustomerNumberNullable>,
    #[serde(rename = "ShippingAddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_1: Option<Shippingaddress>,
    #[serde(rename = "ShippingAddress2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_2: Option<Shippingaddressadditional>,
    #[serde(rename = "ShippingCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_city: Option<Shippingcity>,
    #[serde(rename = "ShippingState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_state: Option<Shippingstate>,
    #[serde(rename = "ShippingZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_zip: Option<Shippingzip>,
    #[serde(rename = "ShippingCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_country: Option<Shippingcountry>,
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<CustomerId>,
    #[serde(rename = "customerStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_status: Option<CustomerStatus>,
    #[serde(rename = "AdditionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalDataString>,
}
