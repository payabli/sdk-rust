pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RequestSchedule {
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
    /// Object describing details of the payment. For Regular subscriptions, skip a payment by setting `totalAmount` to 0; payments pause until you update it to a non-zero value, and `serviceFee` must also be 0 when `totalAmount` is 0. For BalanceDriven subscriptions, any `totalAmount` you send is accepted but ignored at run time. Each run charges the payor's live balance, and a zero balance is skipped.
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
    /// Subscription type. Defaults to `Regular` when omitted. Can't be changed after the subscription is created. If you send it to the update endpoint, it's ignored.
    #[serde(rename = "subscriptionType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_type: Option<SubscriptionType>,
    /// When `true`, the request creates a new customer record, regardless of whether customer identifiers match an existing customer. Defaults to `false`.
    #[serde(rename = "forceCustomerCreation")]
    #[serde(skip)]
    pub force_customer_creation: Option<ForceCustomerCreation>,
}

impl RequestSchedule {
    pub fn builder() -> RequestScheduleBuilder {
        <RequestScheduleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequestScheduleBuilder {
    customer_data: Option<PayorDataRequest>,
    entry_point: Option<Entrypointfield>,
    invoice_data: Option<BillData>,
    payment_details: Option<PaymentDetail>,
    payment_method: Option<RequestSchedulePaymentMethod>,
    schedule_details: Option<ScheduleDetail>,
    set_pause: Option<SetPause>,
    source: Option<Source>,
    subdomain: Option<Subdomain>,
    subscription_type: Option<SubscriptionType>,
    force_customer_creation: Option<ForceCustomerCreation>,
}

impl RequestScheduleBuilder {
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

    pub fn subscription_type(mut self, value: SubscriptionType) -> Self {
        self.subscription_type = Some(value);
        self
    }

    pub fn force_customer_creation(mut self, value: ForceCustomerCreation) -> Self {
        self.force_customer_creation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RequestSchedule`].
    pub fn build(self) -> Result<RequestSchedule, BuildError> {
        Ok(RequestSchedule {
            customer_data: self.customer_data,
            entry_point: self.entry_point,
            invoice_data: self.invoice_data,
            payment_details: self.payment_details,
            payment_method: self.payment_method,
            schedule_details: self.schedule_details,
            set_pause: self.set_pause,
            source: self.source,
            subdomain: self.subdomain,
            subscription_type: self.subscription_type,
            force_customer_creation: self.force_customer_creation,
        })
    }
}
