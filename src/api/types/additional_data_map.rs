pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct AdditionalDataMap(pub HashMap<String, String>);
