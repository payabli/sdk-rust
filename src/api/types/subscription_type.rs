pub use crate::prelude::*;

/// Subscription type or category. Can't be changed after the subscription is created; if sent to the update endpoint, it's ignored.
///
/// - `Regular`: A standard recurring subscription that charges a fixed amount each cycle.
/// - `BalanceDriven`: A subscription that charges the payor's outstanding balance at run time instead of a fixed amount. Each scheduled run reads the live balance and charges that amount; a zero balance is skipped, not charged. `BalanceDriven` subscriptions only accept the `firstofmonth`, `fifteenthofmonth`, and `endofmonth` frequencies, ignore any `startDate` and `endDate` you supply (start date is calculated from `frequency`; the schedule runs until cancelled), and don't store a static `totalAmount` on the schedule.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SubscriptionType {
    Regular,
    BalanceDriven,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for SubscriptionType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Regular => serializer.serialize_str("Regular"),
            Self::BalanceDriven => serializer.serialize_str("BalanceDriven"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for SubscriptionType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Regular" => Ok(Self::Regular),
            "BalanceDriven" => Ok(Self::BalanceDriven),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for SubscriptionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Regular => write!(f, "Regular"),
            Self::BalanceDriven => write!(f, "BalanceDriven"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
