pub use crate::prelude::*;

/// Object containing receipt body configuration
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReceiptContent {
    /// Section amount of payment receipt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<Element>,
    /// Section contactUs of payment receipt
    #[serde(rename = "contactUs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_us: Option<Element>,
    /// Section payment details of payment receipt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<Element>,
    /// Section logo of payment receipt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<Element>,
    /// Section message of payment receipt
    #[serde(rename = "messageBeforeButton")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message_before_button: Option<LabelElement>,
    /// Section page of payment receipt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<PageElement>,
    /// Section payment button of payment receipt
    #[serde(rename = "paymentButton")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_button: Option<LabelElement>,
    /// Section payment information of payment receipt
    #[serde(rename = "paymentInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_information: Option<Element>,
    /// The receipt's settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<SettingElement>,
}

impl ReceiptContent {
    pub fn builder() -> ReceiptContentBuilder {
        <ReceiptContentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReceiptContentBuilder {
    amount: Option<Element>,
    contact_us: Option<Element>,
    details: Option<Element>,
    logo: Option<Element>,
    message_before_button: Option<LabelElement>,
    page: Option<PageElement>,
    payment_button: Option<LabelElement>,
    payment_information: Option<Element>,
    settings: Option<SettingElement>,
}

impl ReceiptContentBuilder {
    pub fn amount(mut self, value: Element) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn contact_us(mut self, value: Element) -> Self {
        self.contact_us = Some(value);
        self
    }

    pub fn details(mut self, value: Element) -> Self {
        self.details = Some(value);
        self
    }

    pub fn logo(mut self, value: Element) -> Self {
        self.logo = Some(value);
        self
    }

    pub fn message_before_button(mut self, value: LabelElement) -> Self {
        self.message_before_button = Some(value);
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

    pub fn payment_information(mut self, value: Element) -> Self {
        self.payment_information = Some(value);
        self
    }

    pub fn settings(mut self, value: SettingElement) -> Self {
        self.settings = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReceiptContent`].
    pub fn build(self) -> Result<ReceiptContent, BuildError> {
        Ok(ReceiptContent {
            amount: self.amount,
            contact_us: self.contact_us,
            details: self.details,
            logo: self.logo,
            message_before_button: self.message_before_button,
            page: self.page,
            payment_button: self.payment_button,
            payment_information: self.payment_information,
            settings: self.settings,
        })
    }
}
