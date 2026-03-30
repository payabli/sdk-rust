pub use crate::prelude::*;

/// This metadata appears only when the domain verification check fails. It gives more information about why the check failed.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApplePayMetadata {
    /// When `true`, indicates whether the domain verification file is available at the expected path. When `false`, Payabli was unable to find the file at the expected path. If the file is missing, make sure it's hosted at the correct path: `/.well-known/apple-developer-merchantid-domain-association`
    #[serde(rename = "isFileAvailable")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_file_available: Option<bool>,
    /// Indicates whether the domain verification file content is valid. If the file is invalid, try downloading it and hosting it again.
    #[serde(rename = "isFileContentValid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_file_content_valid: Option<bool>,
    /// The domain name if the domain verification URL returns a redirect.
    #[serde(rename = "redirectDomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_domain_name: Option<String>,
    /// If the domain verification URL is redirected, this is the URL it's redirected to.
    /// For example, www.partner.com could redirect to www.partners-new-home-page.com. In this case, you should add www.partners-new-home-page.com as a domain instead of www.partner.com.
    #[serde(rename = "redirectUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// The status code return by the domain verification URL.
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
}

impl ApplePayMetadata {
    pub fn builder() -> ApplePayMetadataBuilder {
        <ApplePayMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplePayMetadataBuilder {
    is_file_available: Option<bool>,
    is_file_content_valid: Option<bool>,
    redirect_domain_name: Option<String>,
    redirect_url: Option<String>,
    status_code: Option<i64>,
}

impl ApplePayMetadataBuilder {
    pub fn is_file_available(mut self, value: bool) -> Self {
        self.is_file_available = Some(value);
        self
    }

    pub fn is_file_content_valid(mut self, value: bool) -> Self {
        self.is_file_content_valid = Some(value);
        self
    }

    pub fn redirect_domain_name(mut self, value: impl Into<String>) -> Self {
        self.redirect_domain_name = Some(value.into());
        self
    }

    pub fn redirect_url(mut self, value: impl Into<String>) -> Self {
        self.redirect_url = Some(value.into());
        self
    }

    pub fn status_code(mut self, value: i64) -> Self {
        self.status_code = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApplePayMetadata`].
    pub fn build(self) -> Result<ApplePayMetadata, BuildError> {
        Ok(ApplePayMetadata {
            is_file_available: self.is_file_available,
            is_file_content_valid: self.is_file_content_valid,
            redirect_domain_name: self.redirect_domain_name,
            redirect_url: self.redirect_url,
            status_code: self.status_code,
        })
    }
}
