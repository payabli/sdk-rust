pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PatchOutPaymentLinkRequest {
    /// Updated payment link page configuration.
    #[serde(rename = "billPageData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_page_data: Option<PaymentPageRequestBodyOut>,
    /// New expiration date for the payment link. Must be a future date. If null and the link is expired, uses the default expiration from settings. Updating the expiration date reactivates an expired payment link to Active status.
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    /// Updated status for the payment link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PaymentLinkStatus>,
}

impl PatchOutPaymentLinkRequest {
    pub fn builder() -> PatchOutPaymentLinkRequestBuilder {
        <PatchOutPaymentLinkRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PatchOutPaymentLinkRequestBuilder {
    bill_page_data: Option<PaymentPageRequestBodyOut>,
    expiration_date: Option<String>,
    status: Option<PaymentLinkStatus>,
}

impl PatchOutPaymentLinkRequestBuilder {
    pub fn bill_page_data(mut self, value: PaymentPageRequestBodyOut) -> Self {
        self.bill_page_data = Some(value);
        self
    }

    pub fn expiration_date(mut self, value: impl Into<String>) -> Self {
        self.expiration_date = Some(value.into());
        self
    }

    pub fn status(mut self, value: PaymentLinkStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PatchOutPaymentLinkRequest`].
    pub fn build(self) -> Result<PatchOutPaymentLinkRequest, BuildError> {
        Ok(PatchOutPaymentLinkRequest {
            bill_page_data: self.bill_page_data,
            expiration_date: self.expiration_date,
            status: self.status,
        })
    }
}
