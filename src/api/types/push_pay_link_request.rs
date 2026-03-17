pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "channel")]
pub enum PushPayLinkRequest {
    #[serde(rename = "email")]
    Email {
        #[serde(flatten)]
        data: PushPayLinkRequestEmail,
    },

    #[serde(rename = "sms")]
    Sms {
        #[serde(flatten)]
        data: PushPayLinkRequestSms,
    },
}
