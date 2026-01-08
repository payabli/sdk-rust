pub use crate::prelude::*;

/// Request type for API operation
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
