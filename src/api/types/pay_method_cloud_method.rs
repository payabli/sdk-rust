pub use crate::prelude::*;

/// Method to use for the transaction. For cloud device transactions, the method is `cloud`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PayMethodCloudMethod {
    #[serde(rename = "cloud")]
    Cloud,
}
impl fmt::Display for PayMethodCloudMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Cloud => "cloud",
        };
        write!(f, "{}", s)
    }
}
