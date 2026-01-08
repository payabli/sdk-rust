pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BillQueryRecord2BillApprovalsItem {
    /// Indicates whether the bill has been approved. `0` is false, and `1` is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved: Option<i64>,
    /// Timestamp of when the approval was made, in UTC.
    #[serde(rename = "approvedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_time: Option<DatetimeNullable>,
    /// Additional comments on the approval.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// The approving user's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
    /// The approving user's ID.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
}
