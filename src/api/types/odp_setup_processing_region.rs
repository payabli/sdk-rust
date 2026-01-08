pub use crate::prelude::*;

/// Region where payment processing occurs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OdpSetupProcessingRegion {
    #[serde(rename = "US")]
    Us,
    #[serde(rename = "CA")]
    Ca,
}
impl fmt::Display for OdpSetupProcessingRegion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Us => "US",
            Self::Ca => "CA",
        };
        write!(f, "{}", s)
    }
}
