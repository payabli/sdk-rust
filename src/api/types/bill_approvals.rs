pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BillApprovals(pub Vec<Option<BillQueryRecord2BillApprovalsItem>>);