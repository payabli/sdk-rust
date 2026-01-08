pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeneralEvents {
    /// Event description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Event timestamp, in UTC.
    #[serde(rename = "eventTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub event_time: Option<DateTime<FixedOffset>>,
    /// Extra data.
    #[serde(rename = "extraData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_data: Option<HashMap<String, serde_json::Value>>,
    /// Reference data.
    #[serde(rename = "refData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_data: Option<String>,
    /// The event source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
}
