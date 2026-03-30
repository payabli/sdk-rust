pub use crate::prelude::*;

/// This metadata appears only when the domain verification check fails. It gives more information about why the check failed.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GooglePayMetadata {
    /// The status code return by the domain verification URL.
    #[serde(rename = "statusCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
    /// If the domain verification URL is redirected, this is the URL it's redirected to.  For example, www.partner.com could redirect to www.partners-new-home-page.com. In this case, you should add www.partners-new-home-page.com as a domain instead of www.partner.com.
    #[serde(rename = "redirectUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_url: Option<String>,
    /// The domain name if the domain verification URL returns a redirect.
    #[serde(rename = "redirectDomainName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_domain_name: Option<String>,
}

impl GooglePayMetadata {
    pub fn builder() -> GooglePayMetadataBuilder {
        <GooglePayMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GooglePayMetadataBuilder {
    status_code: Option<i64>,
    redirect_url: Option<String>,
    redirect_domain_name: Option<String>,
}

impl GooglePayMetadataBuilder {
    pub fn status_code(mut self, value: i64) -> Self {
        self.status_code = Some(value);
        self
    }

    pub fn redirect_url(mut self, value: impl Into<String>) -> Self {
        self.redirect_url = Some(value.into());
        self
    }

    pub fn redirect_domain_name(mut self, value: impl Into<String>) -> Self {
        self.redirect_domain_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`GooglePayMetadata`].
    pub fn build(self) -> Result<GooglePayMetadata, BuildError> {
        Ok(GooglePayMetadata {
            status_code: self.status_code,
            redirect_url: self.redirect_url,
            redirect_domain_name: self.redirect_domain_name,
        })
    }
}
