pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubscriptionStatsQueryRecord {
    /// Time interval identifier
    pub interval: String,
    /// Number of subscriptions
    pub count: i64,
    /// Subscription volume
    pub volume: f64,
}
