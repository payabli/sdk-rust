pub use crate::prelude::*;

/// Case metadata, populated as the case progresses. Null until verification completes.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CaseMetadata {
    /// The verification outcome. Null until verification finishes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification: Option<BankVerificationMetadata>,
    /// The reviewer's decision, when one has been made.
    #[serde(rename = "reviewDecision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_decision: Option<ReviewDecisionMetadata>,
}

impl CaseMetadata {
    pub fn builder() -> CaseMetadataBuilder {
        <CaseMetadataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CaseMetadataBuilder {
    verification: Option<BankVerificationMetadata>,
    review_decision: Option<ReviewDecisionMetadata>,
}

impl CaseMetadataBuilder {
    pub fn verification(mut self, value: BankVerificationMetadata) -> Self {
        self.verification = Some(value);
        self
    }

    pub fn review_decision(mut self, value: ReviewDecisionMetadata) -> Self {
        self.review_decision = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CaseMetadata`].
    pub fn build(self) -> Result<CaseMetadata, BuildError> {
        Ok(CaseMetadata {
            verification: self.verification,
            review_decision: self.review_decision,
        })
    }
}
