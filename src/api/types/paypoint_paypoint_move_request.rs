pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaypointMoveRequest {
    #[serde(rename = "entryPoint")]
    #[serde(default)]
    pub entry_point: Entrypointfield,
    /// The ID for the paypoint's new parent organization.
    #[serde(rename = "newParentOrganizationId")]
    #[serde(default)]
    pub new_parent_organization_id: i64,
    /// Optional notification request object for a webhook
    #[serde(rename = "notificationRequest")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_request: Option<NotificationRequest>,
}

impl PaypointMoveRequest {
    pub fn builder() -> PaypointMoveRequestBuilder {
        <PaypointMoveRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaypointMoveRequestBuilder {
    entry_point: Option<Entrypointfield>,
    new_parent_organization_id: Option<i64>,
    notification_request: Option<NotificationRequest>,
}

impl PaypointMoveRequestBuilder {
    pub fn entry_point(mut self, value: Entrypointfield) -> Self {
        self.entry_point = Some(value);
        self
    }

    pub fn new_parent_organization_id(mut self, value: i64) -> Self {
        self.new_parent_organization_id = Some(value);
        self
    }

    pub fn notification_request(mut self, value: NotificationRequest) -> Self {
        self.notification_request = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaypointMoveRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entry_point`](PaypointMoveRequestBuilder::entry_point)
    /// - [`new_parent_organization_id`](PaypointMoveRequestBuilder::new_parent_organization_id)
    pub fn build(self) -> Result<PaypointMoveRequest, BuildError> {
        Ok(PaypointMoveRequest {
            entry_point: self
                .entry_point
                .ok_or_else(|| BuildError::missing_field("entry_point"))?,
            new_parent_organization_id: self
                .new_parent_organization_id
                .ok_or_else(|| BuildError::missing_field("new_parent_organization_id"))?,
            notification_request: self.notification_request,
        })
    }
}
