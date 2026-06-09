pub use crate::prelude::*;

/// Information about the standard notification configuration (email, SMS, web).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NotificationStandardRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<NotificationStandardRequestContent>,
    pub frequency: NotificationStandardRequestFrequency,
    /// Get near-instant notifications via email, SMS, or webhooks for important events like new payment disputes, merchant activations, fraud alerts, approved transactions, settlement history, vendor payouts, and more. Use webhooks with notifications to get real-time updates and automate operations based on those key events. See [Notifications](/developers/developer-guides/notifications-and-webhooks-overview#notifications) for more.
    pub method: NotificationStandardRequestMethod,
    #[serde(rename = "ownerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<Ownerid>,
    #[serde(rename = "ownerType")]
    #[serde(default)]
    pub owner_type: Ownertype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Statusnotification>,
    /// Specify the notification target.
    /// - For method=email the expected value is a list of email addresses separated by semicolon.
    /// - For method=sms the expected value is a list of phone numbers separated by semicolon.
    /// - For method=web the expected value is a valid and complete URL. Webhooks support only standard HTTP ports: 80, 443, 8080, or 4443.
    #[serde(default)]
    pub target: String,
}

impl NotificationStandardRequest {
    pub fn builder() -> NotificationStandardRequestBuilder {
        <NotificationStandardRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotificationStandardRequestBuilder {
    content: Option<NotificationStandardRequestContent>,
    frequency: Option<NotificationStandardRequestFrequency>,
    method: Option<NotificationStandardRequestMethod>,
    owner_id: Option<Ownerid>,
    owner_type: Option<Ownertype>,
    status: Option<Statusnotification>,
    target: Option<String>,
}

impl NotificationStandardRequestBuilder {
    pub fn content(mut self, value: NotificationStandardRequestContent) -> Self {
        self.content = Some(value);
        self
    }

    pub fn frequency(mut self, value: NotificationStandardRequestFrequency) -> Self {
        self.frequency = Some(value);
        self
    }

    pub fn method(mut self, value: NotificationStandardRequestMethod) -> Self {
        self.method = Some(value);
        self
    }

    pub fn owner_id(mut self, value: Ownerid) -> Self {
        self.owner_id = Some(value);
        self
    }

    pub fn owner_type(mut self, value: Ownertype) -> Self {
        self.owner_type = Some(value);
        self
    }

    pub fn status(mut self, value: Statusnotification) -> Self {
        self.status = Some(value);
        self
    }

    pub fn target(mut self, value: impl Into<String>) -> Self {
        self.target = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`NotificationStandardRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`frequency`](NotificationStandardRequestBuilder::frequency)
    /// - [`method`](NotificationStandardRequestBuilder::method)
    /// - [`owner_type`](NotificationStandardRequestBuilder::owner_type)
    /// - [`target`](NotificationStandardRequestBuilder::target)
    pub fn build(self) -> Result<NotificationStandardRequest, BuildError> {
        Ok(NotificationStandardRequest {
            content: self.content,
            frequency: self
                .frequency
                .ok_or_else(|| BuildError::missing_field("frequency"))?,
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
            owner_id: self.owner_id,
            owner_type: self
                .owner_type
                .ok_or_else(|| BuildError::missing_field("owner_type"))?,
            status: self.status,
            target: self
                .target
                .ok_or_else(|| BuildError::missing_field("target"))?,
        })
    }
}
