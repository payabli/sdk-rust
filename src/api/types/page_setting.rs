pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PageSetting {
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
    #[serde(rename = "paymentButton")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_button: Option<ButtonElement>,
    /// Flag indicating if the capability for redirection in the page will be activated
    #[serde(rename = "redirectAfterApprove")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_after_approve: Option<bool>,
    /// Complete URL where the page will be redirected after completion
    #[serde(rename = "redirectAfterApproveUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_after_approve_url: Option<String>,
}

impl PageSetting {
    pub fn builder() -> PageSettingBuilder {
        <PageSettingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PageSettingBuilder {
    color: Option<String>,
    custom_css_url: Option<String>,
    language: Option<String>,
    page_logo: Option<FileContent>,
    payment_button: Option<ButtonElement>,
    redirect_after_approve: Option<bool>,
    redirect_after_approve_url: Option<String>,
}

impl PageSettingBuilder {
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

    pub fn payment_button(mut self, value: ButtonElement) -> Self {
        self.payment_button = Some(value);
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

    /// Consumes the builder and constructs a [`PageSetting`].
    pub fn build(self) -> Result<PageSetting, BuildError> {
        Ok(PageSetting {
            color: self.color,
            custom_css_url: self.custom_css_url,
            language: self.language,
            page_logo: self.page_logo,
            payment_button: self.payment_button,
            redirect_after_approve: self.redirect_after_approve,
            redirect_after_approve_url: self.redirect_after_approve_url,
        })
    }
}
