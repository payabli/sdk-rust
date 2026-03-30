pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PageContent {
    /// Amount section of payment page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<AmountElement>,
    /// Autopay section of payment page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub autopay: Option<AutoElement>,
    /// ContactUs section of payment page
    #[serde(rename = "contactUs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_us: Option<ContactElement>,
    /// Identifier of entry point owner of page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<String>,
    /// Invoices section of payment page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoices: Option<InvoiceElement>,
    /// Logo section of payment page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<Element>,
    /// Message section of payment page
    #[serde(rename = "messageBeforePaying")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_before_paying: Option<LabelElement>,
    /// Descriptor of page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Notes section of payment page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<NoteElement>,
    /// Page header section of payment page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<PageElement>,
    /// Payment button section of payment page
    #[serde(rename = "paymentButton")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_button: Option<LabelElement>,
    /// Payment methods section of payment page
    #[serde(rename = "paymentMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<MethodElement>,
    /// Customer/Payor section of payment page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payor: Option<PayorElement>,
    /// Review section of payment page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review: Option<HeaderElement>,
    /// Unique identifier assigned to the page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<Subdomain>,
}

impl PageContent {
    pub fn builder() -> PageContentBuilder {
        <PageContentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PageContentBuilder {
    amount: Option<AmountElement>,
    autopay: Option<AutoElement>,
    contact_us: Option<ContactElement>,
    entry: Option<String>,
    invoices: Option<InvoiceElement>,
    logo: Option<Element>,
    message_before_paying: Option<LabelElement>,
    name: Option<String>,
    notes: Option<NoteElement>,
    page: Option<PageElement>,
    payment_button: Option<LabelElement>,
    payment_methods: Option<MethodElement>,
    payor: Option<PayorElement>,
    review: Option<HeaderElement>,
    subdomain: Option<Subdomain>,
}

impl PageContentBuilder {
    pub fn amount(mut self, value: AmountElement) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn autopay(mut self, value: AutoElement) -> Self {
        self.autopay = Some(value);
        self
    }

    pub fn contact_us(mut self, value: ContactElement) -> Self {
        self.contact_us = Some(value);
        self
    }

    pub fn entry(mut self, value: impl Into<String>) -> Self {
        self.entry = Some(value.into());
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

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
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

    pub fn subdomain(mut self, value: Subdomain) -> Self {
        self.subdomain = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PageContent`].
    pub fn build(self) -> Result<PageContent, BuildError> {
        Ok(PageContent {
            amount: self.amount,
            autopay: self.autopay,
            contact_us: self.contact_us,
            entry: self.entry,
            invoices: self.invoices,
            logo: self.logo,
            message_before_paying: self.message_before_paying,
            name: self.name,
            notes: self.notes,
            page: self.page,
            payment_button: self.payment_button,
            payment_methods: self.payment_methods,
            payor: self.payor,
            review: self.review,
            subdomain: self.subdomain,
        })
    }
}
