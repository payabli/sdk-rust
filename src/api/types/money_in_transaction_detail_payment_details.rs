pub use crate::prelude::*;

/// Detailed breakdown of payment amounts and identifiers
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionDetailPaymentDetails {
    #[serde(rename = "totalAmount")]
    pub total_amount: f64,
    #[serde(rename = "serviceFee")]
    pub service_fee: f64,
    #[serde(rename = "checkNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_number: Option<String>,
    #[serde(rename = "checkImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_image: Option<serde_json::Value>,
    #[serde(rename = "checkUniqueId")]
    pub check_unique_id: String,
    pub currency: String,
    #[serde(rename = "orderDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_description: Option<Orderdescription>,
    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderId>,
    #[serde(rename = "orderIdAlternative")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id_alternative: Option<String>,
    #[serde(rename = "paymentDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_description: Option<String>,
    #[serde(rename = "groupNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(rename = "payabliTransId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payabli_trans_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unbundled: Option<serde_json::Value>,
    pub categories: Vec<serde_json::Value>,
    #[serde(rename = "splitFunding")]
    pub split_funding: Vec<serde_json::Value>,
}
