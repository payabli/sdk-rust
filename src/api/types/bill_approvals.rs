pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BillApprovals(pub Option<Vec<Option<BillQueryRecord2BillApprovalsItem>>>);
