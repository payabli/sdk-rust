pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PagelinkSetting {
    /// An HTML color code in format #RRGGBB
    #[serde(skip_serializing_if = "Option::is_none")]
    pub color: Option<String>,
    /// Complete URL to a custom CSS file to be loaded with the page
    #[serde(rename = "customCssUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_css_url: Option<String>,
    /// Two-letter code following ISO 639-1
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    /// Object containing logo file to upload/ use in page
    #[serde(rename = "pageLogo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_logo: Option<FileContent>,
    /// Flag indicating if the capability for redirection in the page will be activated
    #[serde(rename = "redirectAfterApprove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_after_approve: Option<bool>,
    /// Complete URL where the page will be redirected after completion
    #[serde(rename = "redirectAfterApproveUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_after_approve_url: Option<String>,
}
