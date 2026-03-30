pub use crate::prelude::*;

/// Query parameters for SendInvoice
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendInvoiceQueryRequest {
    /// When `true`, attaches a PDF version of invoice to the email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachfile: Option<bool>,
    /// Email address where the invoice will be sent to. If this parameter isn't included, Payabli uses the email address on file for the customer owner of the invoice.
    #[serde(rename = "mail2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_2: Option<String>,
}

impl SendInvoiceQueryRequest {
    pub fn builder() -> SendInvoiceQueryRequestBuilder {
        <SendInvoiceQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendInvoiceQueryRequestBuilder {
    attachfile: Option<bool>,
    mail_2: Option<String>,
}

impl SendInvoiceQueryRequestBuilder {
    pub fn attachfile(mut self, value: bool) -> Self {
        self.attachfile = Some(value);
        self
    }

    pub fn mail_2(mut self, value: impl Into<String>) -> Self {
        self.mail_2 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SendInvoiceQueryRequest`].
    pub fn build(self) -> Result<SendInvoiceQueryRequest, BuildError> {
        Ok(SendInvoiceQueryRequest {
            attachfile: self.attachfile,
            mail_2: self.mail_2,
        })
    }
}
