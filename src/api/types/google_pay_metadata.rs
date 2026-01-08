pub use crate::prelude::*;

/// This metadata appears only when the domain verification check fails. It gives more information about why the check failed.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
