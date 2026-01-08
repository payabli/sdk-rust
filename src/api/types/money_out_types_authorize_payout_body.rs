pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuthorizePayoutBody {
    #[serde(rename = "entryPoint")]
    pub entry_point: Entrypointfield,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderId>,
    #[serde(rename = "orderDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_description: Option<Orderdescription>,
    #[serde(rename = "paymentMethod")]
    pub payment_method: AuthorizePaymentMethod,
    /// Object containing payment details.
    #[serde(rename = "paymentDetails")]
    pub payment_details: RequestOutAuthorizePaymentDetails,
    /// Object containing vendor data.
    #[serde(rename = "vendorData")]
    pub vendor_data: RequestOutAuthorizeVendorData,
    /// Array of bills associated to the transaction
    #[serde(rename = "invoiceData")]
    pub invoice_data: Vec<RequestOutAuthorizeInvoiceData>,
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<Accountid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<Subdomain>,
    #[serde(rename = "subscriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<Subscriptionid>,
}
