pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PayMethodCloud {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
    /// Method to use for the transaction. For cloud device transactions, the method is `cloud`.
    pub method: String,
    #[serde(rename = "saveIfSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_if_success: Option<SaveIfSuccess>,
}
