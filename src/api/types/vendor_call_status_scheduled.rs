pub use crate::prelude::*;

/// Details of a queued or in-progress outreach call.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VendorCallStatusScheduled {
    /// ISO-8601 timestamp of the next scheduled call attempt.
    #[serde(rename = "scheduledFor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_for: Option<String>,
    /// Number of call attempts left before retries are exhausted.
    #[serde(rename = "attemptsRemaining")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts_remaining: Option<i64>,
    /// Maximum number of call attempts configured for this schedule.
    #[serde(rename = "maxAttempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_attempts: Option<i64>,
}

impl VendorCallStatusScheduled {
    pub fn builder() -> VendorCallStatusScheduledBuilder {
        <VendorCallStatusScheduledBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VendorCallStatusScheduledBuilder {
    scheduled_for: Option<String>,
    attempts_remaining: Option<i64>,
    max_attempts: Option<i64>,
}

impl VendorCallStatusScheduledBuilder {
    pub fn scheduled_for(mut self, value: impl Into<String>) -> Self {
        self.scheduled_for = Some(value.into());
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

    /// Consumes the builder and constructs a [`VendorCallStatusScheduled`].
    pub fn build(self) -> Result<VendorCallStatusScheduled, BuildError> {
        Ok(VendorCallStatusScheduled {
            scheduled_for: self.scheduled_for,
            attempts_remaining: self.attempts_remaining,
            max_attempts: self.max_attempts,
        })
    }
}
