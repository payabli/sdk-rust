pub use crate::prelude::*;

/// Request type for API operation
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RequestUpdateSchedule {
    /// Object describing details of the payment. To skip the payment, set the `totalAmount` to 0. Payments will be paused until the amount is updated to a non-zero value. When `totalAmount` is set to 0, the `serviceFee` must also be set to 0.
    #[serde(rename = "paymentDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_details: Option<PaymentDetail>,
    /// Object describing the schedule for subscription
    #[serde(rename = "scheduleDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_details: Option<ScheduleDetail>,
    #[serde(rename = "setPause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_pause: Option<SetPause>,
}
