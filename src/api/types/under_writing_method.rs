pub use crate::prelude::*;

/// This field controls which method is used to handle risk orchestration.
/// - `automatic`: Sends the application through the automatic underwriting workflow using the provided `policyId`.
/// - `manual`: Puts the application into the pending review status. An analyst must manually change it's final status to approved or declined.
/// - `bypass`: The application won't go through Payabli's review, and proceeds directly to boarding products and services.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UnderWritingMethod {
    #[serde(rename = "automatic")]
    Automatic,
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "bypass")]
    Bypass,
}
impl fmt::Display for UnderWritingMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Automatic => "automatic",
            Self::Manual => "manual",
            Self::Bypass => "bypass",
        };
        write!(f, "{}", s)
    }
}
