pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct WithdrawApplicationRequest {
    /// An optional reason for the withdrawal, recorded in the status history.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

impl WithdrawApplicationRequest {
    pub fn builder() -> WithdrawApplicationRequestBuilder {
        <WithdrawApplicationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct WithdrawApplicationRequestBuilder {
    reason: Option<String>,
}

impl WithdrawApplicationRequestBuilder {
    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`WithdrawApplicationRequest`].
    pub fn build(self) -> Result<WithdrawApplicationRequest, BuildError> {
        Ok(WithdrawApplicationRequest {
            reason: self.reason,
        })
    }
}
