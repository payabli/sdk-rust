pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AdditionalData(pub HashMap<String, HashMap<String, serde_json::Value>>);
