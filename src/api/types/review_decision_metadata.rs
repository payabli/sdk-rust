pub use crate::prelude::*;

/// Details of a reviewer's decision, when one has been made.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReviewDecisionMetadata {
    /// The decline reason, when the case was denied.
    #[serde(rename = "declineReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decline_reason: Option<BankReviewDecisionReason>,
    /// A free-text note attached to the decision.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl ReviewDecisionMetadata {
    pub fn builder() -> ReviewDecisionMetadataBuilder {
        <ReviewDecisionMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReviewDecisionMetadataBuilder {
    decline_reason: Option<BankReviewDecisionReason>,
    note: Option<String>,
}

impl ReviewDecisionMetadataBuilder {
    pub fn decline_reason(mut self, value: BankReviewDecisionReason) -> Self {
        self.decline_reason = Some(value);
        self
    }

    pub fn note(mut self, value: impl Into<String>) -> Self {
        self.note = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ReviewDecisionMetadata`].
    pub fn build(self) -> Result<ReviewDecisionMetadata, BuildError> {
        Ok(ReviewDecisionMetadata {
            decline_reason: self.decline_reason,
            note: self.note,
        })
    }
}
