pub use crate::prelude::*;

/// A transition action in the case state machine. `Assign` is fired through the
/// dedicated assign endpoint, not the transitions endpoint.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CaseTrigger {
    Submit,
    Verify,
    RequestReview,
    Assign,
    RequestResponse,
    Escalate,
    Approve,
    AutoApprove,
    RequestCompletion,
    Complete,
    Deny,
    Error,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CaseTrigger {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Submit => serializer.serialize_str("Submit"),
            Self::Verify => serializer.serialize_str("Verify"),
            Self::RequestReview => serializer.serialize_str("RequestReview"),
            Self::Assign => serializer.serialize_str("Assign"),
            Self::RequestResponse => serializer.serialize_str("RequestResponse"),
            Self::Escalate => serializer.serialize_str("Escalate"),
            Self::Approve => serializer.serialize_str("Approve"),
            Self::AutoApprove => serializer.serialize_str("AutoApprove"),
            Self::RequestCompletion => serializer.serialize_str("RequestCompletion"),
            Self::Complete => serializer.serialize_str("Complete"),
            Self::Deny => serializer.serialize_str("Deny"),
            Self::Error => serializer.serialize_str("Error"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CaseTrigger {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Submit" => Ok(Self::Submit),
            "Verify" => Ok(Self::Verify),
            "RequestReview" => Ok(Self::RequestReview),
            "Assign" => Ok(Self::Assign),
            "RequestResponse" => Ok(Self::RequestResponse),
            "Escalate" => Ok(Self::Escalate),
            "Approve" => Ok(Self::Approve),
            "AutoApprove" => Ok(Self::AutoApprove),
            "RequestCompletion" => Ok(Self::RequestCompletion),
            "Complete" => Ok(Self::Complete),
            "Deny" => Ok(Self::Deny),
            "Error" => Ok(Self::Error),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CaseTrigger {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Submit => write!(f, "Submit"),
            Self::Verify => write!(f, "Verify"),
            Self::RequestReview => write!(f, "RequestReview"),
            Self::Assign => write!(f, "Assign"),
            Self::RequestResponse => write!(f, "RequestResponse"),
            Self::Escalate => write!(f, "Escalate"),
            Self::Approve => write!(f, "Approve"),
            Self::AutoApprove => write!(f, "AutoApprove"),
            Self::RequestCompletion => write!(f, "RequestCompletion"),
            Self::Complete => write!(f, "Complete"),
            Self::Deny => write!(f, "Deny"),
            Self::Error => write!(f, "Error"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
