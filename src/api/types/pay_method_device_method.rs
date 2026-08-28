pub use crate::prelude::*;

/// Method to use for the transaction. For semi-integrated device transactions, the method is `device`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PayMethodDeviceMethod {
    #[serde(rename = "device")]
    Device,
}
impl fmt::Display for PayMethodDeviceMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Device => "device",
        };
        write!(f, "{}", s)
    }
}
