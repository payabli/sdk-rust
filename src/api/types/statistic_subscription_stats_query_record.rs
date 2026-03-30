pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SubscriptionStatsQueryRecord {
    /// Time interval identifier
    #[serde(default)]
    pub interval: String,
    /// Number of subscriptions
    #[serde(default)]
    pub count: i64,
    /// Subscription volume
    #[serde(default)]
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
