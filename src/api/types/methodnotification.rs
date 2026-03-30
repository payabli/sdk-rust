pub use crate::prelude::*;

/// Method to use to send the notification to the target.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Methodnotification {
    Email,
    Sms,
    Web,
    ReportEmail,
    ReportWeb,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for Methodnotification {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Email => serializer.serialize_str("email"),
            Self::Sms => serializer.serialize_str("sms"),
            Self::Web => serializer.serialize_str("web"),
            Self::ReportEmail => serializer.serialize_str("report-email"),
            Self::ReportWeb => serializer.serialize_str("report-web"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for Methodnotification {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "email" => Ok(Self::Email),
            "sms" => Ok(Self::Sms),
            "web" => Ok(Self::Web),
            "report-email" => Ok(Self::ReportEmail),
            "report-web" => Ok(Self::ReportWeb),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for Methodnotification {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Email => write!(f, "email"),
            Self::Sms => write!(f, "sms"),
            Self::Web => write!(f, "web"),
            Self::ReportEmail => write!(f, "report-email"),
            Self::ReportWeb => write!(f, "report-web"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
