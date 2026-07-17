pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "channel")]
#[non_exhaustive]
pub enum PushPayLinkRequest {
    #[serde(rename = "email")]
    #[non_exhaustive]
    Email {
        #[serde(rename = "additionalEmails")]
        #[serde(skip_serializing_if = "Option::is_none")]
        additional_emails: Option<Vec<String>>,
        #[serde(rename = "attachFile")]
        #[serde(skip_serializing_if = "Option::is_none")]
        attach_file: Option<bool>,
    },

    #[serde(rename = "sms")]
    #[non_exhaustive]
    Sms {},

    /// Catch-all variant for unrecognized discriminant values.
    /// If the server sends a discriminant not recognized by the current SDK
    /// version, the raw payload is captured here so callers can still inspect it.
    #[serde(untagged)]
    __Unknown(serde_json::Value),
}

impl PushPayLinkRequest {
    pub fn email() -> Self {
        Self::Email {
            additional_emails: None,
            attach_file: None,
        }
    }

    pub fn sms() -> Self {
        Self::Sms {}
    }

    pub fn email_with_additional_emails(
        additional_emails: Vec<String>,
        attach_file: Option<bool>,
    ) -> Self {
        Self::Email {
            additional_emails: Some(additional_emails),
            attach_file,
        }
    }

    pub fn email_with_attach_file(
        additional_emails: Option<Vec<String>>,
        attach_file: bool,
    ) -> Self {
        Self::Email {
            additional_emails,
            attach_file: Some(attach_file),
        }
    }

    pub fn unknown(value: serde_json::Value) -> Self {
        Self::__Unknown(value)
    }
}
