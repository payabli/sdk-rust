pub use crate::prelude::*;

/// The progress of setting up an approved application's services with their processors.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EnablementResult {
    /// The application reference.
    #[serde(rename = "requestsReference")]
    #[serde(default)]
    pub requests_reference: String,
    /// The overall enablement state. `null` when enablement hasn't started. One of:
    ///
    /// - `Submitted`: Enablement is in progress.
    /// - `Activated`: Every service is `Provisioned`.
    /// - `Failed`: Enablement stopped and isn't retried.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The enablement progress of each requested service. Empty when enablement hasn't started.
    #[serde(default)]
    pub services: Vec<EnablementService>,
    /// Why enablement isn't moving. `null` when nothing is blocking it, or when enablement hasn't started. One of:
    ///
    /// - `processor`: At least one service is waiting on its processor, for example during an outage or the processor's review. Payabli retries automatically.
    /// - `merchant_data`: Something about the merchant needs a correction, such as fields the processor refused or a missing tax ID. Check `missingFields` on each service.
    /// - `platform_configuration`: Waiting on configuration that Payabli maintains. The merchant doesn't need to act.
    /// - `enablement_failed`: Enablement stopped and isn't retried. `services` shows which service stopped.
    /// - `unclassified`: Enablement isn't complete, and the cause isn't determined yet.
    #[serde(rename = "blockedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_on: Option<String>,
    /// A sentence that explains `blockedOn`. From the second attempt on, it includes the number of attempts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// How many times Payabli has attempted enablement. `null` when enablement hasn't started.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attempts: Option<i64>,
    /// When Payabli first attempted enablement.
    #[serde(rename = "firstAttemptAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub first_attempt_at: Option<DateTime<Utc>>,
    /// When Payabli most recently attempted enablement.
    #[serde(rename = "lastAttemptAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub last_attempt_at: Option<DateTime<Utc>>,
}

impl EnablementResult {
    pub fn builder() -> EnablementResultBuilder {
        <EnablementResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnablementResultBuilder {
    requests_reference: Option<String>,
    state: Option<String>,
    services: Option<Vec<EnablementService>>,
    blocked_on: Option<String>,
    message: Option<String>,
    attempts: Option<i64>,
    first_attempt_at: Option<DateTime<Utc>>,
    last_attempt_at: Option<DateTime<Utc>>,
}

impl EnablementResultBuilder {
    pub fn requests_reference(mut self, value: impl Into<String>) -> Self {
        self.requests_reference = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn services(mut self, value: Vec<EnablementService>) -> Self {
        self.services = Some(value);
        self
    }

    pub fn blocked_on(mut self, value: impl Into<String>) -> Self {
        self.blocked_on = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn attempts(mut self, value: i64) -> Self {
        self.attempts = Some(value);
        self
    }

    pub fn first_attempt_at(mut self, value: DateTime<Utc>) -> Self {
        self.first_attempt_at = Some(value);
        self
    }

    pub fn last_attempt_at(mut self, value: DateTime<Utc>) -> Self {
        self.last_attempt_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EnablementResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`requests_reference`](EnablementResultBuilder::requests_reference)
    /// - [`services`](EnablementResultBuilder::services)
    pub fn build(self) -> Result<EnablementResult, BuildError> {
        Ok(EnablementResult {
            requests_reference: self
                .requests_reference
                .ok_or_else(|| BuildError::missing_field("requests_reference"))?,
            state: self.state,
            services: self
                .services
                .ok_or_else(|| BuildError::missing_field("services"))?,
            blocked_on: self.blocked_on,
            message: self.message,
            attempts: self.attempts,
            first_attempt_at: self.first_attempt_at,
            last_attempt_at: self.last_attempt_at,
        })
    }
}
