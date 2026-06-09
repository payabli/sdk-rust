pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UpdatePayoutSubscriptionBody {
    #[serde(rename = "setPause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_pause: Option<PayoutSetPause>,
    /// Object describing details of the payout.
    #[serde(rename = "paymentDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_details: Option<PayoutPaymentDetail>,
    #[serde(rename = "paymentMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<AuthorizePaymentMethod>,
    /// Object describing the schedule for the payout subscription.
    #[serde(rename = "scheduleDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_details: Option<PayoutScheduleDetail>,
}

impl UpdatePayoutSubscriptionBody {
    pub fn builder() -> UpdatePayoutSubscriptionBodyBuilder {
        <UpdatePayoutSubscriptionBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UpdatePayoutSubscriptionBodyBuilder {
    set_pause: Option<PayoutSetPause>,
    payment_details: Option<PayoutPaymentDetail>,
    payment_method: Option<AuthorizePaymentMethod>,
    schedule_details: Option<PayoutScheduleDetail>,
}

impl UpdatePayoutSubscriptionBodyBuilder {
    pub fn set_pause(mut self, value: PayoutSetPause) -> Self {
        self.set_pause = Some(value);
        self
    }

    pub fn payment_details(mut self, value: PayoutPaymentDetail) -> Self {
        self.payment_details = Some(value);
        self
    }

    pub fn payment_method(mut self, value: AuthorizePaymentMethod) -> Self {
        self.payment_method = Some(value);
        self
    }

    pub fn schedule_details(mut self, value: PayoutScheduleDetail) -> Self {
        self.schedule_details = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UpdatePayoutSubscriptionBody`].
    pub fn build(self) -> Result<UpdatePayoutSubscriptionBody, BuildError> {
        Ok(UpdatePayoutSubscriptionBody {
            set_pause: self.set_pause,
            payment_details: self.payment_details,
            payment_method: self.payment_method,
            schedule_details: self.schedule_details,
        })
    }
}
