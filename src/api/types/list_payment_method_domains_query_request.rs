pub use crate::prelude::*;

/// Query parameters for ListPaymentMethodDomains
///
/// Request type for the ListPaymentMethodDomainsQueryRequest operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPaymentMethodDomainsQueryRequest {
    /// Identifier for the organization or paypoint.
    /// - For organization, provide the organization ID - For paypoint, provide the paypoint ID
    #[serde(rename = "entityId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<i64>,
    /// The type of entity. Valid values:
    /// - organization
    /// - paypoint
    /// - psp
    #[serde(rename = "entityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    /// Number of records to skip. Defaults to `0`.
    #[serde(rename = "fromRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_record: Option<i64>,
    /// Max number of records for query response. Defaults to `20`.
    #[serde(rename = "limitRecord")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_record: Option<i64>,
}
