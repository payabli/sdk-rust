pub use crate::prelude::*;

/// Query parameters for SetApprovedBill
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SetApprovedBillQueryRequest {
    /// Email or username of user modifying approval status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl SetApprovedBillQueryRequest {
    pub fn builder() -> SetApprovedBillQueryRequestBuilder {
        <SetApprovedBillQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SetApprovedBillQueryRequestBuilder {
    email: Option<String>,
}

impl SetApprovedBillQueryRequestBuilder {
    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`SetApprovedBillQueryRequest`].
    pub fn build(self) -> Result<SetApprovedBillQueryRequest, BuildError> {
        Ok(SetApprovedBillQueryRequest { email: self.email })
    }
}
