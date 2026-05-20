pub use crate::prelude::*;

/// Request for SendToApprovalBill (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SendToApprovalBillRequest {
    /// Automatically create the target user for approval if they don't exist.
    #[serde(rename = "autocreateUser")]
    #[serde(skip_serializing)]
    pub autocreate_user: Option<bool>,
    #[serde(default)]
    pub body: Vec<String>,
}

impl SendToApprovalBillRequest {
    pub fn builder() -> SendToApprovalBillRequestBuilder {
        <SendToApprovalBillRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SendToApprovalBillRequestBuilder {
    autocreate_user: Option<bool>,
    body: Option<Vec<String>>,
}

impl SendToApprovalBillRequestBuilder {
    pub fn autocreate_user(mut self, value: bool) -> Self {
        self.autocreate_user = Some(value);
        self
    }

    pub fn body(mut self, value: Vec<String>) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SendToApprovalBillRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](SendToApprovalBillRequestBuilder::body)
    pub fn build(self) -> Result<SendToApprovalBillRequest, BuildError> {
        Ok(SendToApprovalBillRequest {
            autocreate_user: self.autocreate_user,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
