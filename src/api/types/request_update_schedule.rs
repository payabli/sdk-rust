pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RequestUpdateSchedule {
    /// Object describing details of the payment. For Regular subscriptions, skip a payment by setting `totalAmount` to 0; payments pause until you update it to a non-zero value, and `serviceFee` must also be 0 when `totalAmount` is 0. For BalanceDriven subscriptions, any `totalAmount` you send is accepted but ignored at run time. Each run charges the payor's live balance, and a zero balance is skipped.
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
