pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AdditionalData(pub HashMap<String, Option<HashMap<String, serde_json::Value>>>);
