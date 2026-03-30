pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AuthorizePayoutBody {
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
}

impl AuthorizePayoutBody {
    pub fn builder() -> AuthorizePayoutBodyBuilder {
        <AuthorizePayoutBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthorizePayoutBodyBuilder {
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
}

impl AuthorizePayoutBodyBuilder {
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

    /// Consumes the builder and constructs a [`AuthorizePayoutBody`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entry_point`](AuthorizePayoutBodyBuilder::entry_point)
    /// - [`payment_method`](AuthorizePayoutBodyBuilder::payment_method)
    /// - [`payment_details`](AuthorizePayoutBodyBuilder::payment_details)
    /// - [`vendor_data`](AuthorizePayoutBodyBuilder::vendor_data)
    /// - [`invoice_data`](AuthorizePayoutBodyBuilder::invoice_data)
    pub fn build(self) -> Result<AuthorizePayoutBody, BuildError> {
        Ok(AuthorizePayoutBody {
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
        })
    }
}
