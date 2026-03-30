pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RequestTokenStorage {
    /// Object describing the Customer/Payor owner of payment method. Required for POST requests. Which fields are required depends on the paypoint's custom identifier settings.
    #[serde(rename = "customerData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_data: Option<PayorDataRequest>,
    /// Entrypoint identifier. Required for POST requests.
    #[serde(rename = "entryPoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_point: Option<Entrypointfield>,
    /// When `true`, if tokenization fails, Payabli will attempt an authorization transaction to request a permanent token for the card. If the authorization is successful, the card will be tokenized and the authorization will be voided automatically.
    #[serde(rename = "fallbackAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_auth: Option<bool>,
    /// The amount for the `fallbackAuth` transaction. Defaults to one dollar (`100`).
    #[serde(rename = "fallbackAuthAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_auth_amount: Option<i64>,
    /// Custom description for stored payment method.
    #[serde(rename = "methodDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_description: Option<String>,
    /// Information about the payment method for the transaction.
    #[serde(rename = "paymentMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<RequestTokenStoragePaymentMethod>,
    #[serde(rename = "vendorData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_data: Option<VendorDataRequest>,
    /// Custom identifier to indicate the source for the request
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<Subdomain>,
}

impl RequestTokenStorage {
    pub fn builder() -> RequestTokenStorageBuilder {
        <RequestTokenStorageBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequestTokenStorageBuilder {
    customer_data: Option<PayorDataRequest>,
    entry_point: Option<Entrypointfield>,
    fallback_auth: Option<bool>,
    fallback_auth_amount: Option<i64>,
    method_description: Option<String>,
    payment_method: Option<RequestTokenStoragePaymentMethod>,
    vendor_data: Option<VendorDataRequest>,
    source: Option<Source>,
    subdomain: Option<Subdomain>,
}

impl RequestTokenStorageBuilder {
    pub fn customer_data(mut self, value: PayorDataRequest) -> Self {
        self.customer_data = Some(value);
        self
    }

    pub fn entry_point(mut self, value: Entrypointfield) -> Self {
        self.entry_point = Some(value);
        self
    }

    pub fn fallback_auth(mut self, value: bool) -> Self {
        self.fallback_auth = Some(value);
        self
    }

    pub fn fallback_auth_amount(mut self, value: i64) -> Self {
        self.fallback_auth_amount = Some(value);
        self
    }

    pub fn method_description(mut self, value: impl Into<String>) -> Self {
        self.method_description = Some(value.into());
        self
    }

    pub fn payment_method(mut self, value: RequestTokenStoragePaymentMethod) -> Self {
        self.payment_method = Some(value);
        self
    }

    pub fn vendor_data(mut self, value: VendorDataRequest) -> Self {
        self.vendor_data = Some(value);
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

    /// Consumes the builder and constructs a [`RequestTokenStorage`].
    pub fn build(self) -> Result<RequestTokenStorage, BuildError> {
        Ok(RequestTokenStorage {
            customer_data: self.customer_data,
            entry_point: self.entry_point,
            fallback_auth: self.fallback_auth,
            fallback_auth_amount: self.fallback_auth_amount,
            method_description: self.method_description,
            payment_method: self.payment_method,
            vendor_data: self.vendor_data,
            source: self.source,
            subdomain: self.subdomain,
        })
    }
}
