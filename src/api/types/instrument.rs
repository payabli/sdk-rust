pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Instrument {
    #[serde(rename = "achAccount")]
    pub ach_account: Achaccount,
    #[serde(rename = "achRouting")]
    pub ach_routing: Achrouting,
    #[serde(rename = "billingAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<BillingAddressNullable>,
    #[serde(rename = "billingCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_city: Option<BillingCityNullable>,
    #[serde(rename = "billingCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_country: Option<BillingCountryNullable>,
    #[serde(rename = "billingState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_state: Option<BillingStateNullable>,
    #[serde(rename = "billingZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_zip: Option<BillingZip>,
}
