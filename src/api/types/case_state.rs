pub use crate::prelude::*;

/// The state of a case in the bank-account-change lifecycle. `Completed` and
/// `Denied` are terminal.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CaseState {
    Submitted,
    Verifying,
    PendingReview,
    Assigned,
    PendingResponse,
    Escalated,
    Approved,
    AutoApproved,
    PendingCompletion,
    Completed,
    Denied,
    Error,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CaseState {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Submitted => serializer.serialize_str("Submitted"),
            Self::Verifying => serializer.serialize_str("Verifying"),
            Self::PendingReview => serializer.serialize_str("PendingReview"),
            Self::Assigned => serializer.serialize_str("Assigned"),
            Self::PendingResponse => serializer.serialize_str("PendingResponse"),
            Self::Escalated => serializer.serialize_str("Escalated"),
            Self::Approved => serializer.serialize_str("Approved"),
            Self::AutoApproved => serializer.serialize_str("AutoApproved"),
            Self::PendingCompletion => serializer.serialize_str("PendingCompletion"),
            Self::Completed => serializer.serialize_str("Completed"),
            Self::Denied => serializer.serialize_str("Denied"),
            Self::Error => serializer.serialize_str("Error"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CaseState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Submitted" => Ok(Self::Submitted),
            "Verifying" => Ok(Self::Verifying),
            "PendingReview" => Ok(Self::PendingReview),
            "Assigned" => Ok(Self::Assigned),
            "PendingResponse" => Ok(Self::PendingResponse),
            "Escalated" => Ok(Self::Escalated),
            "Approved" => Ok(Self::Approved),
            "AutoApproved" => Ok(Self::AutoApproved),
            "PendingCompletion" => Ok(Self::PendingCompletion),
            "Completed" => Ok(Self::Completed),
            "Denied" => Ok(Self::Denied),
            "Error" => Ok(Self::Error),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CaseState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Submitted => write!(f, "Submitted"),
            Self::Verifying => write!(f, "Verifying"),
            Self::PendingReview => write!(f, "PendingReview"),
            Self::Assigned => write!(f, "Assigned"),
            Self::PendingResponse => write!(f, "PendingResponse"),
            Self::Escalated => write!(f, "Escalated"),
            Self::Approved => write!(f, "Approved"),
            Self::AutoApproved => write!(f, "AutoApproved"),
            Self::PendingCompletion => write!(f, "PendingCompletion"),
            Self::Completed => write!(f, "Completed"),
            Self::Denied => write!(f, "Denied"),
            Self::Error => write!(f, "Error"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
