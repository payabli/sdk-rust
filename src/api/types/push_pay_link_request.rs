pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(tag = "channel")]
pub enum PushPayLinkRequest {
    Email {
        #[serde(flatten)]
        data: PushPayLinkRequestEmail,
    },

    Sms {
        #[serde(flatten)]
        data: PushPayLinkRequestSms,
    },
}
