pub use crate::prelude::*;

/// Get near-instant notifications via email, SMS, or webhooks for important
/// events like new payment disputes, merchant activations, fraud alerts,
/// approved transactions, settlement history, vendor payouts, and more. Use
/// webhooks with notifications to get real-time updates and automate
/// operations based on those key events. See
/// [Notifications](/developers/developer-guides/notifications-and-webhooks-overview#notifications)
/// for more.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NotificationStandardRequestMethod {
    Email,
    Sms,
    Web,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for NotificationStandardRequestMethod {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Email => serializer.serialize_str("email"),
            Self::Sms => serializer.serialize_str("sms"),
            Self::Web => serializer.serialize_str("web"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for NotificationStandardRequestMethod {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "email" => Ok(Self::Email),
            "sms" => Ok(Self::Sms),
            "web" => Ok(Self::Web),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for NotificationStandardRequestMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Email => write!(f, "email"),
            Self::Sms => write!(f, "sms"),
            Self::Web => write!(f, "web"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
