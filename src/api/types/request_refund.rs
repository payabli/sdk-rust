pub use crate::prelude::*;

/// Request type for API operation
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RequestRefund {
    /// Amount to refund from original transaction, minus any service fees charged on the original transaction.
    ///
    /// The amount provided can't be greater than the original total amount of the transaction, minus service fees. For example, if a transaction was $90 plus a $10 service fee, you can refund up to $90.
    ///
    /// An amount equal to zero will refund the total amount authorized minus any service fee.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipaddress: Option<IpAddress>,
    #[serde(rename = "orderDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_description: Option<Orderdescription>,
    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderId>,
    #[serde(rename = "refundDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_details: Option<RefundDetail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
}
