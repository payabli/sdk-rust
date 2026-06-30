pub use crate::prelude::*;

/// Details of an outreach call that didn't complete successfully.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VendorCallStatusFailed {
    /// ISO-8601 timestamp of the most recent call attempt.
    #[serde(rename = "lastAttemptAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_attempt_at: Option<String>,
    /// Reason the call didn't complete, as reported by the calling system (for example, `No answer`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Number of call attempts left before retries are exhausted.
    #[serde(rename = "attemptsRemaining")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts_remaining: Option<i64>,
    /// Maximum number of call attempts configured for this schedule.
    #[serde(rename = "maxAttempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<i64>,
    /// ISO-8601 timestamp of the next scheduled retry, or `null` when no further retries are scheduled.
    #[serde(rename = "nextRetryScheduledFor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_retry_scheduled_for: Option<String>,
}

impl VendorCallStatusFailed {
    pub fn builder() -> VendorCallStatusFailedBuilder {
        <VendorCallStatusFailedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VendorCallStatusFailedBuilder {
    last_attempt_at: Option<String>,
    reason: Option<String>,
    attempts_remaining: Option<i64>,
    max_attempts: Option<i64>,
    next_retry_scheduled_for: Option<String>,
}

impl VendorCallStatusFailedBuilder {
    pub fn last_attempt_at(mut self, value: impl Into<String>) -> Self {
        self.last_attempt_at = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn attempts_remaining(mut self, value: i64) -> Self {
        self.attempts_remaining = Some(value);
        self
    }

    pub fn max_attempts(mut self, value: i64) -> Self {
        self.max_attempts = Some(value);
        self
    }

    pub fn next_retry_scheduled_for(mut self, value: impl Into<String>) -> Self {
        self.next_retry_scheduled_for = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VendorCallStatusFailed`].
    pub fn build(self) -> Result<VendorCallStatusFailed, BuildError> {
        Ok(VendorCallStatusFailed {
            last_attempt_at: self.last_attempt_at,
            reason: self.reason,
            attempts_remaining: self.attempts_remaining,
            max_attempts: self.max_attempts,
            next_retry_scheduled_for: self.next_retry_scheduled_for,
        })
    }
}
