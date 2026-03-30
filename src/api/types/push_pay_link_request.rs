pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "channel")]
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
}
