pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Responsedata(pub HashMap<String, serde_json::Value>);
