pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentPageRequestBody {
    /// ContactUs section of payment link page
    #[serde(rename = "contactUs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_us: Option<ContactElement>,
    /// Invoices section of payment link page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoices: Option<InvoiceElement>,
    /// Logo section of payment link page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<Element>,
    /// Message section of payment link page
    #[serde(rename = "messageBeforePaying")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_before_paying: Option<LabelElement>,
    /// Notes section of payment link page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<NoteElement>,
    /// Page header section of payment link page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<PageElement>,
    /// Payment button section of payment link page
    #[serde(rename = "paymentButton")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_button: Option<LabelElement>,
    /// Payment methods section of payment link page
    #[serde(rename = "paymentMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<MethodElement>,
    /// Customer/Payor section of payment link page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payor: Option<PayorElement>,
    /// Review section of payment link page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review: Option<HeaderElement>,
    /// Settings section of payment link page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<PagelinkSetting>,
}

impl PaymentPageRequestBody {
    pub fn builder() -> PaymentPageRequestBodyBuilder {
        <PaymentPageRequestBodyBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentPageRequestBodyBuilder {
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
}

impl PaymentPageRequestBodyBuilder {
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

    /// Consumes the builder and constructs a [`PaymentPageRequestBody`].
    pub fn build(self) -> Result<PaymentPageRequestBody, BuildError> {
        Ok(PaymentPageRequestBody {
            contact_us: self.contact_us,
            invoices: self.invoices,
            logo: self.logo,
            message_before_paying: self.message_before_paying,
            notes: self.notes,
            page: self.page,
            payment_button: self.payment_button,
            payment_methods: self.payment_methods,
            payor: self.payor,
            review: self.review,
            settings: self.settings,
        })
    }
}
