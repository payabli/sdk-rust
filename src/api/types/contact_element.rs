pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ContactElement {
    /// Custom content for email
    #[serde(rename = "emailLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Enabled>,
    /// Header text for section
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    /// Flag indicating if icons for accepted card brands will be shown
    #[serde(rename = "paymentIcons")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_icons: Option<bool>,
    /// Custom content for phone number
    #[serde(rename = "phoneLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_label: Option<String>,
}

impl ContactElement {
    pub fn builder() -> ContactElementBuilder {
        <ContactElementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContactElementBuilder {
    email_label: Option<String>,
    enabled: Option<Enabled>,
    header: Option<String>,
    order: Option<Order>,
    payment_icons: Option<bool>,
    phone_label: Option<String>,
}

impl ContactElementBuilder {
    pub fn email_label(mut self, value: impl Into<String>) -> Self {
        self.email_label = Some(value.into());
        self
    }

    pub fn enabled(mut self, value: Enabled) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn header(mut self, value: impl Into<String>) -> Self {
        self.header = Some(value.into());
        self
    }

    pub fn order(mut self, value: Order) -> Self {
        self.order = Some(value);
        self
    }

    pub fn payment_icons(mut self, value: bool) -> Self {
        self.payment_icons = Some(value);
        self
    }

    pub fn phone_label(mut self, value: impl Into<String>) -> Self {
        self.phone_label = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ContactElement`].
    pub fn build(self) -> Result<ContactElement, BuildError> {
        Ok(ContactElement {
            email_label: self.email_label,
            enabled: self.enabled,
            header: self.header,
            order: self.order,
            payment_icons: self.payment_icons,
            phone_label: self.phone_label,
        })
    }
}
