pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
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

impl PagelinkSetting {
    pub fn builder() -> PagelinkSettingBuilder {
        <PagelinkSettingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PagelinkSettingBuilder {
    color: Option<String>,
    custom_css_url: Option<String>,
    language: Option<String>,
    page_logo: Option<FileContent>,
    redirect_after_approve: Option<bool>,
    redirect_after_approve_url: Option<String>,
}

impl PagelinkSettingBuilder {
    pub fn color(mut self, value: impl Into<String>) -> Self {
        self.color = Some(value.into());
        self
    }

    pub fn custom_css_url(mut self, value: impl Into<String>) -> Self {
        self.custom_css_url = Some(value.into());
        self
    }

    pub fn language(mut self, value: impl Into<String>) -> Self {
        self.language = Some(value.into());
        self
    }

    pub fn page_logo(mut self, value: FileContent) -> Self {
        self.page_logo = Some(value);
        self
    }

    pub fn redirect_after_approve(mut self, value: bool) -> Self {
        self.redirect_after_approve = Some(value);
        self
    }

    pub fn redirect_after_approve_url(mut self, value: impl Into<String>) -> Self {
        self.redirect_after_approve_url = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PagelinkSetting`].
    pub fn build(self) -> Result<PagelinkSetting, BuildError> {
        Ok(PagelinkSetting {
            color: self.color,
            custom_css_url: self.custom_css_url,
            language: self.language,
            page_logo: self.page_logo,
            redirect_after_approve: self.redirect_after_approve,
            redirect_after_approve_url: self.redirect_after_approve_url,
        })
    }
}
