pub use crate::prelude::*;

/// Request for AddPayLinkFromBillLotNumber (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddPayLinkFromBillLotNumberRequest {
    /// The entity's entrypoint identifier. [Learn more](/developers/api-reference/api-overview#entrypoint-vs-entry)
    #[serde(rename = "entryPoint")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub entry_point: Entry,
    /// The vendor number for the vendor being paid with this payment link.
    #[serde(rename = "vendorNumber")]
    #[serde(skip_serializing)]
    #[serde(default)]
    pub vendor_number: String,
    /// List of recipient email addresses. When there is more than one, separate them by a semicolon (;).
    #[serde(rename = "mail2")]
    #[serde(skip_serializing)]
    pub mail_2: Option<String>,
    /// Indicates whether customer can modify the payment amount. A value of `true` means the amount isn't modifiable, a value `false` means the payor can modify the amount to pay.
    #[serde(rename = "amountFixed")]
    #[serde(skip_serializing)]
    pub amount_fixed: Option<String>,
    #[serde(default)]
    pub body: PaymentPageRequestBodyOut,
}

impl AddPayLinkFromBillLotNumberRequest {
    pub fn builder() -> AddPayLinkFromBillLotNumberRequestBuilder {
        <AddPayLinkFromBillLotNumberRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddPayLinkFromBillLotNumberRequestBuilder {
    entry_point: Option<Entry>,
    vendor_number: Option<String>,
    mail_2: Option<String>,
    amount_fixed: Option<String>,
    body: Option<PaymentPageRequestBodyOut>,
}

impl AddPayLinkFromBillLotNumberRequestBuilder {
    pub fn entry_point(mut self, value: Entry) -> Self {
        self.entry_point = Some(value);
        self
    }

    pub fn vendor_number(mut self, value: impl Into<String>) -> Self {
        self.vendor_number = Some(value.into());
        self
    }

    pub fn mail_2(mut self, value: impl Into<String>) -> Self {
        self.mail_2 = Some(value.into());
        self
    }

    pub fn amount_fixed(mut self, value: impl Into<String>) -> Self {
        self.amount_fixed = Some(value.into());
        self
    }

    pub fn body(mut self, value: PaymentPageRequestBodyOut) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddPayLinkFromBillLotNumberRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entry_point`](AddPayLinkFromBillLotNumberRequestBuilder::entry_point)
    /// - [`vendor_number`](AddPayLinkFromBillLotNumberRequestBuilder::vendor_number)
    /// - [`body`](AddPayLinkFromBillLotNumberRequestBuilder::body)
    pub fn build(self) -> Result<AddPayLinkFromBillLotNumberRequest, BuildError> {
        Ok(AddPayLinkFromBillLotNumberRequest {
            entry_point: self
                .entry_point
                .ok_or_else(|| BuildError::missing_field("entry_point"))?,
            vendor_number: self
                .vendor_number
                .ok_or_else(|| BuildError::missing_field("vendor_number"))?,
            mail_2: self.mail_2,
            amount_fixed: self.amount_fixed,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
