pub use crate::prelude::*;

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

impl RequestUpdateSchedule {
    pub fn builder() -> RequestUpdateScheduleBuilder {
        <RequestUpdateScheduleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequestUpdateScheduleBuilder {
    payment_details: Option<PaymentDetail>,
    schedule_details: Option<ScheduleDetail>,
    set_pause: Option<SetPause>,
}

impl RequestUpdateScheduleBuilder {
    pub fn payment_details(mut self, value: PaymentDetail) -> Self {
        self.payment_details = Some(value);
        self
    }

    pub fn schedule_details(mut self, value: ScheduleDetail) -> Self {
        self.schedule_details = Some(value);
        self
    }

    pub fn set_pause(mut self, value: SetPause) -> Self {
        self.set_pause = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RequestUpdateSchedule`].
    pub fn build(self) -> Result<RequestUpdateSchedule, BuildError> {
        Ok(RequestUpdateSchedule {
            payment_details: self.payment_details,
            schedule_details: self.schedule_details,
            set_pause: self.set_pause,
        })
    }
}
