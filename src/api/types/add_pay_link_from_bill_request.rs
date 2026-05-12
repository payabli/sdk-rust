pub use crate::prelude::*;

/// Request for AddPayLinkFromBill (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddPayLinkFromBillRequest {
    /// Indicates whether customer can modify the payment amount. A value of `true` means the amount isn't modifiable, a value `false` means the payor can modify the amount to pay.
    #[serde(rename = "amountFixed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_fixed: Option<bool>,
    /// List of recipient email addresses. When there is more than one, separate them by a semicolon (;).
    #[serde(rename = "mail2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_2: Option<String>,
    #[serde(default)]
    pub body: PaymentPageRequestBodyOut,
}

impl AddPayLinkFromBillRequest {
    pub fn builder() -> AddPayLinkFromBillRequestBuilder {
        <AddPayLinkFromBillRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddPayLinkFromBillRequestBuilder {
    amount_fixed: Option<bool>,
    mail_2: Option<String>,
    body: Option<PaymentPageRequestBodyOut>,
}

impl AddPayLinkFromBillRequestBuilder {
    pub fn amount_fixed(mut self, value: bool) -> Self {
        self.amount_fixed = Some(value);
        self
    }

    pub fn mail_2(mut self, value: impl Into<String>) -> Self {
        self.mail_2 = Some(value.into());
        self
    }

    pub fn body(mut self, value: PaymentPageRequestBodyOut) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddPayLinkFromBillRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](AddPayLinkFromBillRequestBuilder::body)
    pub fn build(self) -> Result<AddPayLinkFromBillRequest, BuildError> {
        Ok(AddPayLinkFromBillRequest {
            amount_fixed: self.amount_fixed,
            mail_2: self.mail_2,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
