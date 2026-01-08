pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BillEvents(pub Option<Vec<GeneralEvents>>);
