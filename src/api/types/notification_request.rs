pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NotificationRequest {
    /// Complete HTTP URL receiving the notification
    #[serde(rename = "notificationUrl")]
    #[serde(default)]
    pub notification_url: String,
    /// List of key-value header parameters to include in the notification request
    #[serde(rename = "webHeaderParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_header_parameters: Option<Vec<WebHeaderParameter>>,
}

impl NotificationRequest {
    pub fn builder() -> NotificationRequestBuilder {
        <NotificationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotificationRequestBuilder {
    notification_url: Option<String>,
    web_header_parameters: Option<Vec<WebHeaderParameter>>,
}

impl NotificationRequestBuilder {
    pub fn notification_url(mut self, value: impl Into<String>) -> Self {
        self.notification_url = Some(value.into());
        self
    }

    pub fn web_header_parameters(mut self, value: Vec<WebHeaderParameter>) -> Self {
        self.web_header_parameters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NotificationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`notification_url`](NotificationRequestBuilder::notification_url)
    pub fn build(self) -> Result<NotificationRequest, BuildError> {
        Ok(NotificationRequest {
            notification_url: self
                .notification_url
                .ok_or_else(|| BuildError::missing_field("notification_url"))?,
            web_header_parameters: self.web_header_parameters,
        })
    }
}
