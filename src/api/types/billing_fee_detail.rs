pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BillingFeeDetail {
    #[serde(rename = "billableEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billable_event: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(rename = "eventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// Description of the billing fee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Category of the billing fee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Fixed price component of the fee
    #[serde(rename = "fixPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix_price: Option<f64>,
    /// Percentage component of the fee
    #[serde(rename = "floatPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float_price: Option<f64>,
    /// Amount eligible for the fee
    #[serde(rename = "billableAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billable_amount: Option<f64>,
    /// Total fee amount charged
    #[serde(rename = "billAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    #[serde(rename = "serviceGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_group: Option<String>,
}
