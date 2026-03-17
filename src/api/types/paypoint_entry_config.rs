pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaypointEntryConfig {
    #[serde(rename = "EntryComment")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_comment: Option<String>,
    #[serde(rename = "EntryLogo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_logo: Option<String>,
    #[serde(rename = "EntryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_name: Option<String>,
    #[serde(rename = "EntryPages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_pages: Option<Vec<PayabliPages>>,
    #[serde(rename = "EntrySubtitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_subtitle: Option<String>,
    #[serde(rename = "EntryTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_title: Option<String>,
    #[serde(rename = "IdEntry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_entry: Option<i64>,
    #[serde(rename = "Paypoint")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint: Option<PaypointData>,
}
