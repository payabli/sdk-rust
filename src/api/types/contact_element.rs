pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
