pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SplitFunding(pub Option<Vec<SplitFundingContent>>);
