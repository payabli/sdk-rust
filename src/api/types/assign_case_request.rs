pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AssignCaseRequest {
    /// The numeric id of the reviewer to assign the case to.
    #[serde(rename = "assigneeId")]
    #[serde(default)]
    pub assignee_id: i64,
    /// An optional reason for the assignment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl AssignCaseRequest {
    pub fn builder() -> AssignCaseRequestBuilder {
        <AssignCaseRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AssignCaseRequestBuilder {
    assignee_id: Option<i64>,
    reason: Option<String>,
}

impl AssignCaseRequestBuilder {
    pub fn assignee_id(mut self, value: i64) -> Self {
        self.assignee_id = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AssignCaseRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`assignee_id`](AssignCaseRequestBuilder::assignee_id)
    pub fn build(self) -> Result<AssignCaseRequest, BuildError> {
        Ok(AssignCaseRequest {
            assignee_id: self
                .assignee_id
                .ok_or_else(|| BuildError::missing_field("assignee_id"))?,
            reason: self.reason,
        })
    }
}
