pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayLinkUpdateData {
    /// ContactUs section of payment link page
    #[serde(rename = "contactUs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_us: Option<ContactElement>,
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
    /// Review section of payment link page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review: Option<HeaderElement>,
    /// Settings section of payment link page
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<PagelinkSetting>,
}

impl PayLinkUpdateData {
    pub fn builder() -> PayLinkUpdateDataBuilder {
        <PayLinkUpdateDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayLinkUpdateDataBuilder {
    contact_us: Option<ContactElement>,
    logo: Option<Element>,
    message_before_paying: Option<LabelElement>,
    notes: Option<NoteElement>,
    page: Option<PageElement>,
    payment_button: Option<LabelElement>,
    payment_methods: Option<MethodElement>,
    review: Option<HeaderElement>,
    settings: Option<PagelinkSetting>,
}

impl PayLinkUpdateDataBuilder {
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

    pub fn payment_methods(mut self, value: MethodElement) -> Self {
        self.payment_methods = Some(value);
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

    /// Consumes the builder and constructs a [`PayLinkUpdateData`].
    pub fn build(self) -> Result<PayLinkUpdateData, BuildError> {
        Ok(PayLinkUpdateData {
            contact_us: self.contact_us,
            logo: self.logo,
            message_before_paying: self.message_before_paying,
            notes: self.notes,
            page: self.page,
            payment_button: self.payment_button,
            payment_methods: self.payment_methods,
            review: self.review,
            settings: self.settings,
        })
    }
}
