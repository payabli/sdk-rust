pub use crate::prelude::*;

/// Request type for API operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RequestPaymentValidate {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<Accountid>,
    #[serde(rename = "entryPoint")]
    pub entry_point: Entrypointfield,
    #[serde(rename = "orderDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_description: Option<Orderdescription>,
    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderId>,
    /// Object describing payment method to use for transaction.
    #[serde(rename = "paymentMethod")]
    pub payment_method: RequestPaymentValidatePaymentMethod,
}
