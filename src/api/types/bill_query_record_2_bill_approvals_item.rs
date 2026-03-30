pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BillQueryRecord2BillApprovalsItem {
    /// Indicates whether the bill has been approved. `0` is false, and `1` is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved: Option<i64>,
    /// Timestamp of when the approval was made, in UTC.
    #[serde(rename = "approvedTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub approved_time: Option<DateTime<Utc>>,
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

impl BillQueryRecord2BillApprovalsItem {
    pub fn builder() -> BillQueryRecord2BillApprovalsItemBuilder {
        <BillQueryRecord2BillApprovalsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillQueryRecord2BillApprovalsItemBuilder {
    approved: Option<i64>,
    approved_time: Option<DateTime<Utc>>,
    comments: Option<String>,
    email: Option<Email>,
    id: Option<i64>,
}

impl BillQueryRecord2BillApprovalsItemBuilder {
    pub fn approved(mut self, value: i64) -> Self {
        self.approved = Some(value);
        self
    }

    pub fn approved_time(mut self, value: DateTime<Utc>) -> Self {
        self.approved_time = Some(value);
        self
    }

    pub fn comments(mut self, value: impl Into<String>) -> Self {
        self.comments = Some(value.into());
        self
    }

    pub fn email(mut self, value: Email) -> Self {
        self.email = Some(value);
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BillQueryRecord2BillApprovalsItem`].
    pub fn build(self) -> Result<BillQueryRecord2BillApprovalsItem, BuildError> {
        Ok(BillQueryRecord2BillApprovalsItem {
            approved: self.approved,
            approved_time: self.approved_time,
            comments: self.comments,
            email: self.email,
            id: self.id,
        })
    }
}
