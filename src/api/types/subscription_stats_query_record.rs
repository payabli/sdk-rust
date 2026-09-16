pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SubscriptionStatsQueryRecord {
    /// The renewal window this row represents: `30` (due within 30 days), `60` (31 to 60 days), `90` (61 to 90 days), or `+90` (more than 90 days out). Note the response label `+90` differs from its request path value `plus`. Requesting `all` returns one row per window.
    #[serde(default)]
    pub interval: String,
    /// Number of active subscriptions scheduled to renew within this window. This is a forecast of upcoming renewals, not charges already taken, so it doesn't reconcile with `inSubscriptionsPaid` on `/Statistic/basic`.
    #[serde(default)]
    pub count: i64,
    /// Total value of the upcoming renewals in this window, net of fees.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub volume: f64,
}

impl SubscriptionStatsQueryRecord {
    pub fn builder() -> SubscriptionStatsQueryRecordBuilder {
        <SubscriptionStatsQueryRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionStatsQueryRecordBuilder {
    interval: Option<String>,
    count: Option<i64>,
    volume: Option<f64>,
}

impl SubscriptionStatsQueryRecordBuilder {
    pub fn interval(mut self, value: impl Into<String>) -> Self {
        self.interval = Some(value.into());
        self
    }

    pub fn count(mut self, value: i64) -> Self {
        self.count = Some(value);
        self
    }

    pub fn volume(mut self, value: f64) -> Self {
        self.volume = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionStatsQueryRecord`].
    /// This method will fail if any of the following fields are not set:
    /// - [`interval`](SubscriptionStatsQueryRecordBuilder::interval)
    /// - [`count`](SubscriptionStatsQueryRecordBuilder::count)
    /// - [`volume`](SubscriptionStatsQueryRecordBuilder::volume)
    pub fn build(self) -> Result<SubscriptionStatsQueryRecord, BuildError> {
        Ok(SubscriptionStatsQueryRecord {
            interval: self
                .interval
                .ok_or_else(|| BuildError::missing_field("interval"))?,
            count: self
                .count
                .ok_or_else(|| BuildError::missing_field("count"))?,
            volume: self
                .volume
                .ok_or_else(|| BuildError::missing_field("volume"))?,
        })
    }
}
