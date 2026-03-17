pub use crate::prelude::*;

/// Object containing receipt body configuration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
