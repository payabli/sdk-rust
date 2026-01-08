pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubscriptionRequestBody {
    /// Object describing the customer/payor.
    #[serde(rename = "customerData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_data: Option<PayorDataRequest>,
    #[serde(rename = "entryPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<Entrypointfield>,
    /// Object describing an Invoice linked to the subscription.
    #[serde(rename = "invoiceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_data: Option<BillData>,
    /// Object describing details of the payment. To skip the payment, set the `totalAmount` to 0. Payments will be paused until the amount is updated to a non-zero value. When `totalAmount` is set to 0, the `serviceFee` must also be set to 0.
    #[serde(rename = "paymentDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_details: Option<PaymentDetail>,
    /// Information about the payment method for the transaction. Required and recommended fields for each payment method type are described in each schema below.
    #[serde(rename = "paymentMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<RequestSchedulePaymentMethod>,
    /// Object describing the schedule for subscription.
    #[serde(rename = "scheduleDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_details: Option<ScheduleDetail>,
    #[serde(rename = "setPause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_pause: Option<SetPause>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<Subdomain>,
}
