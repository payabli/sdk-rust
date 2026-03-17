pub use crate::prelude::*;

/// Describes the customer's SMS communications consent status.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CustomerQueryRecordsCustomerConsentSms {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OptinStatus>,
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<LastModified>,
}
