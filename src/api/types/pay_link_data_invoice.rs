pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayLinkDataInvoice {
    /// Contact us section of payment link page. If omitted, this block is enabled at display order 11.
    #[serde(rename = "contactUs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_us: Option<ContactElement>,
    /// Invoices section of payment link page. Required. Omitting it returns a `400` error with code `7045`.
    #[serde(default)]
    pub invoices: InvoiceElement,
    /// Logo section of payment link page. If omitted, this block is enabled at display order 1, and the logo image is resolved from the paypoint's entry logo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<Element>,
    /// Message section of payment link page. If omitted, this block is enabled at display order 5.
    #[serde(rename = "messageBeforePaying")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_before_paying: Option<LabelElement>,
    /// Notes section of payment link page. If omitted, this block is enabled at display order 10.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<NoteElement>,
    /// Page header section of payment link page. If omitted, this block is enabled at display order 2.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<PageElement>,
    /// Payment button section of payment link page. If omitted, this block is enabled at display order 6, with the label "Pay Now".
    #[serde(rename = "paymentButton")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_button: Option<LabelElement>,
    /// Payment methods section of payment link page. If omitted, this block is enabled at display order 3, with all payment methods enabled except RDC.
    #[serde(rename = "paymentMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<MethodElement>,
    /// Customer/Payor section of payment link page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payor: Option<PayorElement>,
    /// Review section of payment link page. If omitted, this block is enabled at display order 4.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review: Option<HeaderElement>,
    /// Settings section of payment link page. If omitted, defaults are applied, including page color `#10a0e3` and language `en`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<PagelinkSetting>,
    /// Indicates whether customer can modify the payment amount. A value of `true` means the amount isn't modifiable, a value `false` means the payor can modify the amount to pay.
    #[serde(rename = "amountFixed")]
    #[serde(skip_serializing)]
    pub amount_fixed: Option<bool>,
    /// List of recipient email addresses. When there is more than one, separate them by a semicolon (;).
    #[serde(rename = "mail2")]
    #[serde(skip_serializing)]
    pub mail_2: Option<String>,
}

impl PayLinkDataInvoice {
    pub fn builder() -> PayLinkDataInvoiceBuilder {
        <PayLinkDataInvoiceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayLinkDataInvoiceBuilder {
    contact_us: Option<ContactElement>,
    invoices: Option<InvoiceElement>,
    logo: Option<Element>,
    message_before_paying: Option<LabelElement>,
    notes: Option<NoteElement>,
    page: Option<PageElement>,
    payment_button: Option<LabelElement>,
    payment_methods: Option<MethodElement>,
    payor: Option<PayorElement>,
    review: Option<HeaderElement>,
    settings: Option<PagelinkSetting>,
    amount_fixed: Option<bool>,
    mail_2: Option<String>,
}

impl PayLinkDataInvoiceBuilder {
    pub fn contact_us(mut self, value: ContactElement) -> Self {
        self.contact_us = Some(value);
        self
    }

    pub fn invoices(mut self, value: InvoiceElement) -> Self {
        self.invoices = Some(value);
        self
    }

    pub fn logo(mut self, value: Element) -> Self {
        self.logo = Some(value);
        self
    }

    pub fn message_before_paying(mut self, value: LabelElement) -> Self {
        self.message_before_paying = Some(value);
        self
    }

    pub fn notes(mut self, value: NoteElement) -> Self {
        self.notes = Some(value);
        self
    }

    pub fn page(mut self, value: PageElement) -> Self {
        self.page = Some(value);
        self
    }

    pub fn payment_button(mut self, value: LabelElement) -> Self {
        self.payment_button = Some(value);
        self
    }

    pub fn payment_methods(mut self, value: MethodElement) -> Self {
        self.payment_methods = Some(value);
        self
    }

    pub fn payor(mut self, value: PayorElement) -> Self {
        self.payor = Some(value);
        self
    }

    pub fn review(mut self, value: HeaderElement) -> Self {
        self.review = Some(value);
        self
    }

    pub fn settings(mut self, value: PagelinkSetting) -> Self {
        self.settings = Some(value);
        self
    }

    pub fn amount_fixed(mut self, value: bool) -> Self {
        self.amount_fixed = Some(value);
        self
    }

    pub fn mail_2(mut self, value: impl Into<String>) -> Self {
        self.mail_2 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PayLinkDataInvoice`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoices`](PayLinkDataInvoiceBuilder::invoices)
    pub fn build(self) -> Result<PayLinkDataInvoice, BuildError> {
        Ok(PayLinkDataInvoice {
            contact_us: self.contact_us,
            invoices: self
                .invoices
                .ok_or_else(|| BuildError::missing_field("invoices"))?,
            logo: self.logo,
            message_before_paying: self.message_before_paying,
            notes: self.notes,
            page: self.page,
            payment_button: self.payment_button,
            payment_methods: self.payment_methods,
            payor: self.payor,
            review: self.review,
            settings: self.settings,
            amount_fixed: self.amount_fixed,
            mail_2: self.mail_2,
        })
    }
}
