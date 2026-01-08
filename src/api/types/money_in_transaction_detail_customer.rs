pub use crate::prelude::*;

/// Customer information associated with the transaction
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TransactionDetailCustomer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<Identifierfields>,
    #[serde(rename = "firstName")]
    pub first_name: String,
    #[serde(rename = "lastName")]
    pub last_name: String,
    #[serde(rename = "companyName")]
    pub company_name: String,
    #[serde(rename = "billingAddress1")]
    pub billing_address_1: BillingAddressNullable,
    #[serde(rename = "billingAddress2")]
    pub billing_address_2: BillingAddressAddtlNullable,
    #[serde(rename = "billingCity")]
    pub billing_city: BillingCityNullable,
    #[serde(rename = "billingState")]
    pub billing_state: BillingStateNullable,
    #[serde(rename = "billingZip")]
    pub billing_zip: BillingZip,
    #[serde(rename = "billingCountry")]
    pub billing_country: BillingCountryNullable,
    #[serde(rename = "billingPhone")]
    pub billing_phone: PhoneNumber,
    #[serde(rename = "billingEmail")]
    pub billing_email: Email,
    #[serde(rename = "customerNumber")]
    pub customer_number: CustomerNumberNullable,
    #[serde(rename = "shippingAddress1")]
    pub shipping_address_1: Shippingaddress,
    #[serde(rename = "shippingAddress2")]
    pub shipping_address_2: Shippingaddressadditional,
    #[serde(rename = "shippingCity")]
    pub shipping_city: Shippingcity,
    #[serde(rename = "shippingState")]
    pub shipping_state: Shippingstate,
    #[serde(rename = "shippingZip")]
    pub shipping_zip: Shippingzip,
    #[serde(rename = "shippingCountry")]
    pub shipping_country: Shippingcountry,
    #[serde(rename = "customerId")]
    pub customer_id: CustomerId,
    #[serde(rename = "customerStatus")]
    pub customer_status: CustomerStatus,
    #[serde(rename = "additionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalDataString>,
}
