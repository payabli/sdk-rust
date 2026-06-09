pub use crate::prelude::*;

/// Configuration for the Pay Out payment link page. Controls branding, messaging, vendor fields, and which payout methods are offered to the vendor.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaymentPageRequestBodyOut {
    /// ContactUs section of payment link page.
    #[serde(rename = "contactUs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_us: Option<ContactElement>,
    /// Logo section of payment link page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<Element>,
    /// Message section of payment link page.
    #[serde(rename = "messageBeforePaying")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_before_paying: Option<LabelElement>,
    /// Notes section of payment link page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<NoteElement>,
    /// Page header section of payment link page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<PageElement>,
    /// Payment button section of payment link page.
    #[serde(rename = "paymentButton")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_button: Option<LabelElement>,
    /// Payment methods section of payment link page. Use this to configure which payout methods (ACH, vCard, check) are offered to the vendor.
    #[serde(rename = "paymentMethods")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_methods: Option<MethodElementOut>,
    /// Review section of payment link page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review: Option<HeaderElement>,
    /// Bills section of payment link page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bills: Option<Element>,
    /// Settings section of payment link page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<PagelinkSetting>,
}

impl PaymentPageRequestBodyOut {
    pub fn builder() -> PaymentPageRequestBodyOutBuilder {
        <PaymentPageRequestBodyOutBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentPageRequestBodyOutBuilder {
    contact_us: Option<ContactElement>,
    logo: Option<Element>,
    message_before_paying: Option<LabelElement>,
    notes: Option<NoteElement>,
    page: Option<PageElement>,
    payment_button: Option<LabelElement>,
    payment_methods: Option<MethodElementOut>,
    review: Option<HeaderElement>,
    bills: Option<Element>,
    settings: Option<PagelinkSetting>,
}

impl PaymentPageRequestBodyOutBuilder {
    pub fn contact_us(mut self, value: ContactElement) -> Self {
        self.contact_us = Some(value);
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

    pub fn payment_methods(mut self, value: MethodElementOut) -> Self {
        self.payment_methods = Some(value);
        self
    }

    pub fn review(mut self, value: HeaderElement) -> Self {
        self.review = Some(value);
        self
    }

    pub fn bills(mut self, value: Element) -> Self {
        self.bills = Some(value);
        self
    }

    pub fn settings(mut self, value: PagelinkSetting) -> Self {
        self.settings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentPageRequestBodyOut`].
    pub fn build(self) -> Result<PaymentPageRequestBodyOut, BuildError> {
        Ok(PaymentPageRequestBodyOut {
            contact_us: self.contact_us,
            logo: self.logo,
            message_before_paying: self.message_before_paying,
            notes: self.notes,
            page: self.page,
            payment_button: self.payment_button,
            payment_methods: self.payment_methods,
            review: self.review,
            bills: self.bills,
            settings: self.settings,
        })
    }
}
