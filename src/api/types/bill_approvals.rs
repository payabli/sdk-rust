pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, Default)]
pub struct BillApprovals(pub Vec<BillQueryRecord2BillApprovalsItem>);
