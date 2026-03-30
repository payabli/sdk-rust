pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PayoutSubscriptionRequestBody {
    #[serde(rename = "entryPoint")]
    #[serde(default)]
    pub entry_point: Entrypointfield,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<Subdomain>,
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(rename = "setPause")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_pause: Option<PayoutSetPause>,
    /// Payment method for the payout subscription. Supports `ach`, `vcard`, and `check`. The `managed` method isn't supported for payout subscriptions.
    #[serde(rename = "paymentMethod")]
    #[serde(default)]
    pub payment_method: AuthorizePaymentMethod,
    /// Object describing details of the payout.
    #[serde(rename = "paymentDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_details: Option<PayoutPaymentDetail>,
    /// Object identifying the vendor for this subscription. Only a `vendorId` or `vendorNumber` is needed to link to an existing vendor.
    #[serde(rename = "vendorData")]
    #[serde(default)]
    pub vendor_data: RequestOutAuthorizeVendorData,
    /// Array of bills associated with the payout subscription. If omitted and `doNotCreateBills` isn't set to `true`, the system creates a bill automatically.
    #[serde(rename = "billData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_data: Option<Vec<BillPayOutDataRequest>>,
    /// Object describing the schedule for the payout subscription.
    #[serde(rename = "scheduleDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_details: Option<PayoutScheduleDetail>,
}

impl PayoutSubscriptionRequestBody {
    pub fn builder() -> PayoutSubscriptionRequestBodyBuilder {
        <PayoutSubscriptionRequestBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayoutSubscriptionRequestBodyBuilder {
    entry_point: Option<Entrypointfield>,
    subdomain: Option<Subdomain>,
    account_id: Option<AccountId>,
    source: Option<Source>,
    set_pause: Option<PayoutSetPause>,
    payment_method: Option<AuthorizePaymentMethod>,
    payment_details: Option<PayoutPaymentDetail>,
    vendor_data: Option<RequestOutAuthorizeVendorData>,
    bill_data: Option<Vec<BillPayOutDataRequest>>,
    schedule_details: Option<PayoutScheduleDetail>,
}

impl PayoutSubscriptionRequestBodyBuilder {
    pub fn entry_point(mut self, value: Entrypointfield) -> Self {
        self.entry_point = Some(value);
        self
    }

    pub fn subdomain(mut self, value: Subdomain) -> Self {
        self.subdomain = Some(value);
        self
    }

    pub fn account_id(mut self, value: AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn source(mut self, value: Source) -> Self {
        self.source = Some(value);
        self
    }

    pub fn set_pause(mut self, value: PayoutSetPause) -> Self {
        self.set_pause = Some(value);
        self
    }

    pub fn payment_method(mut self, value: AuthorizePaymentMethod) -> Self {
        self.payment_method = Some(value);
        self
    }

    pub fn payment_details(mut self, value: PayoutPaymentDetail) -> Self {
        self.payment_details = Some(value);
        self
    }

    pub fn vendor_data(mut self, value: RequestOutAuthorizeVendorData) -> Self {
        self.vendor_data = Some(value);
        self
    }

    pub fn bill_data(mut self, value: Vec<BillPayOutDataRequest>) -> Self {
        self.bill_data = Some(value);
        self
    }

    pub fn schedule_details(mut self, value: PayoutScheduleDetail) -> Self {
        self.schedule_details = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayoutSubscriptionRequestBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entry_point`](PayoutSubscriptionRequestBodyBuilder::entry_point)
    /// - [`payment_method`](PayoutSubscriptionRequestBodyBuilder::payment_method)
    /// - [`vendor_data`](PayoutSubscriptionRequestBodyBuilder::vendor_data)
    pub fn build(self) -> Result<PayoutSubscriptionRequestBody, BuildError> {
        Ok(PayoutSubscriptionRequestBody {
            entry_point: self
                .entry_point
                .ok_or_else(|| BuildError::missing_field("entry_point"))?,
            subdomain: self.subdomain,
            account_id: self.account_id,
            source: self.source,
            set_pause: self.set_pause,
            payment_method: self
                .payment_method
                .ok_or_else(|| BuildError::missing_field("payment_method"))?,
            payment_details: self.payment_details,
            vendor_data: self
                .vendor_data
                .ok_or_else(|| BuildError::missing_field("vendor_data"))?,
            bill_data: self.bill_data,
            schedule_details: self.schedule_details,
        })
    }
}
