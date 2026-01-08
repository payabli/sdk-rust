pub use crate::prelude::*;

/// Request type for API operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestCredit {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<Accountid>,
    /// Object describing the customer/payor.
    #[serde(rename = "customerData")]
    pub customer_data: PayorDataRequest,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<Entrypointfield>,
    #[serde(rename = "orderDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_description: Option<Orderdescription>,
    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderId>,
    #[serde(rename = "paymentDetails")]
    pub payment_details: PaymentDetailCredit,
    /// Object describing the ACH payment method to use for transaction.
    #[serde(rename = "paymentMethod")]
    pub payment_method: RequestCreditPaymentMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<Subdomain>,
    #[serde(rename = "forceCustomerCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_customer_creation: Option<ForceCustomerCreation>,
}
