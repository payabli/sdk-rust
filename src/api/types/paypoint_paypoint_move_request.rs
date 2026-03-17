pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PaypointMoveRequest {
    #[serde(rename = "entryPoint")]
    pub entry_point: Entrypointfield,
    /// The ID for the paypoint's new parent organization.
    #[serde(rename = "newParentOrganizationId")]
    pub new_parent_organization_id: i64,
    /// Optional notification request object for a webhook
    #[serde(rename = "notificationRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_request: Option<NotificationRequest>,
}
