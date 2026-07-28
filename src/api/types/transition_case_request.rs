pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TransitionCaseRequest {
    pub trigger: CaseTrigger,
    /// The reason for the action.
    #[serde(default)]
    pub reason: String,
    /// The decline reason. Required when the trigger is `Deny`, and must be omitted otherwise.
    #[serde(rename = "declineReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_reason: Option<BankReviewDecisionReason>,
}

impl TransitionCaseRequest {
    pub fn builder() -> TransitionCaseRequestBuilder {
        <TransitionCaseRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransitionCaseRequestBuilder {
    trigger: Option<CaseTrigger>,
    reason: Option<String>,
    decline_reason: Option<BankReviewDecisionReason>,
}

impl TransitionCaseRequestBuilder {
    pub fn trigger(mut self, value: CaseTrigger) -> Self {
        self.trigger = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn decline_reason(mut self, value: BankReviewDecisionReason) -> Self {
        self.decline_reason = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransitionCaseRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`trigger`](TransitionCaseRequestBuilder::trigger)
    /// - [`reason`](TransitionCaseRequestBuilder::reason)
    pub fn build(self) -> Result<TransitionCaseRequest, BuildError> {
        Ok(TransitionCaseRequest {
            trigger: self
                .trigger
                .ok_or_else(|| BuildError::missing_field("trigger"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
            decline_reason: self.decline_reason,
        })
    }
}
