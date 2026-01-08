pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransRequestBody {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<Accountid>,
    /// Object describing the Customer/Payor. Which fields are required depends on the paypoint's custom identifier settings.
    #[serde(rename = "customerData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_data: Option<PayorDataRequest>,
    #[serde(rename = "entryPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<Entrypointfield>,
    /// Object describing an Invoice linked to the transaction.
    #[serde(rename = "invoiceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_data: Option<BillData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ipaddress: Option<IpAddress>,
    #[serde(rename = "orderDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_description: Option<Orderdescription>,
    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderId>,
    /// Object describing details of the payment. Required.
    #[serde(rename = "paymentDetails")]
    pub payment_details: PaymentDetail,
    /// Information about the payment method for the transaction. Required and recommended fields for each payment method type are described in each schema below.
    #[serde(rename = "paymentMethod")]
    pub payment_method: PaymentMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<Subdomain>,
    #[serde(rename = "subscriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<Subscriptionid>,
}
