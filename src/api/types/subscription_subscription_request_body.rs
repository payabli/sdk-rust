pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
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

impl SubscriptionRequestBody {
    pub fn builder() -> SubscriptionRequestBodyBuilder {
        <SubscriptionRequestBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionRequestBodyBuilder {
    customer_data: Option<PayorDataRequest>,
    entry_point: Option<Entrypointfield>,
    invoice_data: Option<BillData>,
    payment_details: Option<PaymentDetail>,
    payment_method: Option<RequestSchedulePaymentMethod>,
    schedule_details: Option<ScheduleDetail>,
    set_pause: Option<SetPause>,
    source: Option<Source>,
    subdomain: Option<Subdomain>,
}

impl SubscriptionRequestBodyBuilder {
    pub fn customer_data(mut self, value: PayorDataRequest) -> Self {
        self.customer_data = Some(value);
        self
    }

    pub fn entry_point(mut self, value: Entrypointfield) -> Self {
        self.entry_point = Some(value);
        self
    }

    pub fn invoice_data(mut self, value: BillData) -> Self {
        self.invoice_data = Some(value);
        self
    }

    pub fn payment_details(mut self, value: PaymentDetail) -> Self {
        self.payment_details = Some(value);
        self
    }

    pub fn payment_method(mut self, value: RequestSchedulePaymentMethod) -> Self {
        self.payment_method = Some(value);
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

    pub fn source(mut self, value: Source) -> Self {
        self.source = Some(value);
        self
    }

    pub fn subdomain(mut self, value: Subdomain) -> Self {
        self.subdomain = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionRequestBody`].
    pub fn build(self) -> Result<SubscriptionRequestBody, BuildError> {
        Ok(SubscriptionRequestBody {
            customer_data: self.customer_data,
            entry_point: self.entry_point,
            invoice_data: self.invoice_data,
            payment_details: self.payment_details,
            payment_method: self.payment_method,
            schedule_details: self.schedule_details,
            set_pause: self.set_pause,
            source: self.source,
            subdomain: self.subdomain,
        })
    }
}
