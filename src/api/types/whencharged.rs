pub use crate::prelude::*;

/// Describes when customers are charged for goods or services. Accepted values:
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Whencharged {
    #[serde(rename = "When Service Provided")]
    WhenServiceProvided,
    #[serde(rename = "In Advance")]
    InAdvance,
}
impl fmt::Display for Whencharged {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::WhenServiceProvided => "When Service Provided",
            Self::InAdvance => "In Advance",
        };
        write!(f, "{}", s)
    }
}
