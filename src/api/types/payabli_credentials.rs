pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PayabliCredentials {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(rename = "cfeeFix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfee_fix: Option<f64>,
    #[serde(rename = "cfeeFloat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfee_float: Option<f64>,
    #[serde(rename = "cfeeMax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfee_max: Option<f64>,
    #[serde(rename = "cfeeMin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfee_min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maxticket: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minticket: Option<f64>,
    /// The payment mode supported by this service. `0` for one-time payments, `1` for recurring payments, `2` for both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i64>,
    #[serde(rename = "referenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<i64>,
    /// The payment service that this credential applies to. A paypoint can support multiple services, each represented by its own credential object in the array. Possible values are `card` (credit/debit card), `ach` (ACH bank transfer), `check` (paper check), `vcard` (virtual card), `cloud` (card-present), `cash`, `managed` (managed payment service), and `wallet`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
}