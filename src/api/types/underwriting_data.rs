pub use crate::prelude::*;

/// Underwriting data is used to manage risk orchestration in the boarding application lifecycle.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UnderwritingData {
    pub method: UnderWritingMethod,
    #[serde(rename = "policyId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy_id: Option<PolicyId>,
}

impl UnderwritingData {
    pub fn builder() -> UnderwritingDataBuilder {
        <UnderwritingDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UnderwritingDataBuilder {
    method: Option<UnderWritingMethod>,
    policy_id: Option<PolicyId>,
}

impl UnderwritingDataBuilder {
    pub fn method(mut self, value: UnderWritingMethod) -> Self {
        self.method = Some(value);
        self
    }

    pub fn policy_id(mut self, value: PolicyId) -> Self {
        self.policy_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UnderwritingData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`method`](UnderwritingDataBuilder::method)
    pub fn build(self) -> Result<UnderwritingData, BuildError> {
        Ok(UnderwritingData {
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
            policy_id: self.policy_id,
        })
    }
}
