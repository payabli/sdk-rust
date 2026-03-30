pub use crate::prelude::*;

/// Query parameters for sendPayLinkFromId
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendPayLinkFromIdQueryRequest {
    /// When `true`, attaches a PDF version of invoice to the email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachfile: Option<bool>,
    /// List of recipient email addresses. When there is more than one, separate them by a semicolon (;).
    #[serde(rename = "mail2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mail_2: Option<String>,
}

impl SendPayLinkFromIdQueryRequest {
    pub fn builder() -> SendPayLinkFromIdQueryRequestBuilder {
        <SendPayLinkFromIdQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendPayLinkFromIdQueryRequestBuilder {
    attachfile: Option<bool>,
    mail_2: Option<String>,
}

impl SendPayLinkFromIdQueryRequestBuilder {
    pub fn attachfile(mut self, value: bool) -> Self {
        self.attachfile = Some(value);
        self
    }

    pub fn mail_2(mut self, value: impl Into<String>) -> Self {
        self.mail_2 = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SendPayLinkFromIdQueryRequest`].
    pub fn build(self) -> Result<SendPayLinkFromIdQueryRequest, BuildError> {
        Ok(SendPayLinkFromIdQueryRequest {
            attachfile: self.attachfile,
            mail_2: self.mail_2,
        })
    }
}
