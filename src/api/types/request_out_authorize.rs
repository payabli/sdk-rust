pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RequestOutAuthorize {
    #[serde(rename = "entryPoint")]
    #[serde(default)]
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
    #[serde(default)]
    pub payment_method: AuthorizePaymentMethod,
    /// Object containing payment details.
    #[serde(rename = "paymentDetails")]
    #[serde(default)]
    pub payment_details: RequestOutAuthorizePaymentDetails,
    /// Object containing vendor data.
    #[serde(rename = "vendorData")]
    #[serde(default)]
    pub vendor_data: RequestOutAuthorizeVendorData,
    /// Array of bills associated to the transaction
    #[serde(rename = "invoiceData")]
    #[serde(default)]
    pub invoice_data: Vec<RequestOutAuthorizeInvoiceData>,
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<Subdomain>,
    #[serde(rename = "subscriptionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_id: Option<Subscriptionid>,
    #[serde(rename = "autoCapture")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_capture: Option<AutoCapture>,
    /// When `true`, the authorization bypasses the requirement for unique bills, identified by vendor invoice number. This allows you to make more than one payout authorization for a bill, like a split payment.
    #[serde(rename = "allowDuplicatedBills")]
    #[serde(skip)]
    pub allow_duplicated_bills: Option<bool>,
    /// When `true`, Payabli won't automatically create a bill for this payout transaction.
    #[serde(rename = "doNotCreateBills")]
    #[serde(skip)]
    pub do_not_create_bills: Option<bool>,
    /// When `true`, the request creates a new vendor record, regardless of whether the vendor already exists.
    #[serde(rename = "forceVendorCreation")]
    #[serde(skip)]
    pub force_vendor_creation: Option<bool>,
    /// When `true`, Payabli authorizes the payout for same-day ACH processing instead of standard ACH. Same-day ACH must be enabled for the paypoint, otherwise the authorization fails with a `400` response and `responseCode` `3492`. Only ACH payouts honor this flag. Wire and RTP payouts ignore it.
    ///
    /// Same-day ACH has a daily cutoff. Capture the transaction before the cutoff, or pass `autoConvertSameDayAch` with a value of `true` when you capture it.
    #[serde(rename = "sameDayACH")]
    #[serde(skip)]
    pub same_day_ach: Option<bool>,
}

impl RequestOutAuthorize {
    pub fn builder() -> RequestOutAuthorizeBuilder {
        <RequestOutAuthorizeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequestOutAuthorizeBuilder {
    entry_point: Option<Entrypointfield>,
    source: Option<Source>,
    order_id: Option<OrderId>,
    order_description: Option<Orderdescription>,
    payment_method: Option<AuthorizePaymentMethod>,
    payment_details: Option<RequestOutAuthorizePaymentDetails>,
    vendor_data: Option<RequestOutAuthorizeVendorData>,
    invoice_data: Option<Vec<RequestOutAuthorizeInvoiceData>>,
    account_id: Option<AccountId>,
    subdomain: Option<Subdomain>,
    subscription_id: Option<Subscriptionid>,
    auto_capture: Option<AutoCapture>,
    allow_duplicated_bills: Option<bool>,
    do_not_create_bills: Option<bool>,
    force_vendor_creation: Option<bool>,
    same_day_ach: Option<bool>,
}

impl RequestOutAuthorizeBuilder {
    pub fn entry_point(mut self, value: Entrypointfield) -> Self {
        self.entry_point = Some(value);
        self
    }

    pub fn source(mut self, value: Source) -> Self {
        self.source = Some(value);
        self
    }

    pub fn order_id(mut self, value: OrderId) -> Self {
        self.order_id = Some(value);
        self
    }

    pub fn order_description(mut self, value: Orderdescription) -> Self {
        self.order_description = Some(value);
        self
    }

    pub fn payment_method(mut self, value: AuthorizePaymentMethod) -> Self {
        self.payment_method = Some(value);
        self
    }

    pub fn payment_details(mut self, value: RequestOutAuthorizePaymentDetails) -> Self {
        self.payment_details = Some(value);
        self
    }

    pub fn vendor_data(mut self, value: RequestOutAuthorizeVendorData) -> Self {
        self.vendor_data = Some(value);
        self
    }

    pub fn invoice_data(mut self, value: Vec<RequestOutAuthorizeInvoiceData>) -> Self {
        self.invoice_data = Some(value);
        self
    }

    pub fn account_id(mut self, value: AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn subdomain(mut self, value: Subdomain) -> Self {
        self.subdomain = Some(value);
        self
    }

    pub fn subscription_id(mut self, value: Subscriptionid) -> Self {
        self.subscription_id = Some(value);
        self
    }

    pub fn auto_capture(mut self, value: AutoCapture) -> Self {
        self.auto_capture = Some(value);
        self
    }

    pub fn allow_duplicated_bills(mut self, value: bool) -> Self {
        self.allow_duplicated_bills = Some(value);
        self
    }

    pub fn do_not_create_bills(mut self, value: bool) -> Self {
        self.do_not_create_bills = Some(value);
        self
    }

    pub fn force_vendor_creation(mut self, value: bool) -> Self {
        self.force_vendor_creation = Some(value);
        self
    }

    pub fn same_day_ach(mut self, value: bool) -> Self {
        self.same_day_ach = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RequestOutAuthorize`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entry_point`](RequestOutAuthorizeBuilder::entry_point)
    /// - [`payment_method`](RequestOutAuthorizeBuilder::payment_method)
    /// - [`payment_details`](RequestOutAuthorizeBuilder::payment_details)
    /// - [`vendor_data`](RequestOutAuthorizeBuilder::vendor_data)
    /// - [`invoice_data`](RequestOutAuthorizeBuilder::invoice_data)
    pub fn build(self) -> Result<RequestOutAuthorize, BuildError> {
        Ok(RequestOutAuthorize {
            entry_point: self
                .entry_point
                .ok_or_else(|| BuildError::missing_field("entry_point"))?,
            source: self.source,
            order_id: self.order_id,
            order_description: self.order_description,
            payment_method: self
                .payment_method
                .ok_or_else(|| BuildError::missing_field("payment_method"))?,
            payment_details: self
                .payment_details
                .ok_or_else(|| BuildError::missing_field("payment_details"))?,
            vendor_data: self
                .vendor_data
                .ok_or_else(|| BuildError::missing_field("vendor_data"))?,
            invoice_data: self
                .invoice_data
                .ok_or_else(|| BuildError::missing_field("invoice_data"))?,
            account_id: self.account_id,
            subdomain: self.subdomain,
            subscription_id: self.subscription_id,
            auto_capture: self.auto_capture,
            allow_duplicated_bills: self.allow_duplicated_bills,
            do_not_create_bills: self.do_not_create_bills,
            force_vendor_creation: self.force_vendor_creation,
            same_day_ach: self.same_day_ach,
        })
    }
}
