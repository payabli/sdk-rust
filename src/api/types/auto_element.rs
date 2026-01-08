pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AutoElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Enabled>,
    /// Type of end date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish: Option<Finishtype>,
    /// accepted frequencies for autopay
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<FrequencyList>,
    /// Value of pre-selected frequency
    #[serde(rename = "frequencySelected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_selected: Option<String>,
    /// Header text for section
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    /// Range of days enabled in calendar. Leave empty to enable all days.
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
}
