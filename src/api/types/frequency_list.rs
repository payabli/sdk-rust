pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct FrequencyList {
    /// Enable or disable frequency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annually: Option<bool>,
    /// Enable or disable frequency
    #[serde(rename = "every2Weeks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub every_2_weeks: Option<bool>,
    /// Enable or disable frequency
    #[serde(rename = "every3Months")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub every_3_months: Option<bool>,
    /// Enable or disable frequency
    #[serde(rename = "every6Months")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub every_6_months: Option<bool>,
    /// Enable or disable frequency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly: Option<bool>,
    /// Enable or disable frequency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onetime: Option<bool>,
    /// Enable or disable frequency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly: Option<bool>,
}
