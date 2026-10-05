pub use crate::prelude::*;

/// A notification log entry returned by the detail and retry endpoints, including the request and response captured for the delivery.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NotificationLogDetail {
    #[serde(flatten)]
    pub notification_log_base_fields: NotificationLogBase,
    /// The custom headers Payabli sent with the notification, if any.
    #[serde(rename = "webHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_headers: Option<Vec<StringStringKeyValuePair>>,
    /// The headers the target returned. Null when the target sent no response.
    #[serde(rename = "responseHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_headers: Option<Vec<KeyValueArray>>,
    /// The body the target returned. Empty when the target sent no response.
    #[serde(rename = "responseContent")]
    #[serde(default)]
    pub response_content: String,
}

impl NotificationLogDetail {
    pub fn builder() -> NotificationLogDetailBuilder {
        <NotificationLogDetailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotificationLogDetailBuilder {
    notification_log_base_fields: Option<NotificationLogBase>,
    web_headers: Option<Vec<StringStringKeyValuePair>>,
    response_headers: Option<Vec<KeyValueArray>>,
    response_content: Option<String>,
}

impl NotificationLogDetailBuilder {
    pub fn notification_log_base_fields(mut self, value: NotificationLogBase) -> Self {
        self.notification_log_base_fields = Some(value);
        self
    }

    pub fn web_headers(mut self, value: Vec<StringStringKeyValuePair>) -> Self {
        self.web_headers = Some(value);
        self
    }

    pub fn response_headers(mut self, value: Vec<KeyValueArray>) -> Self {
        self.response_headers = Some(value);
        self
    }

    pub fn response_content(mut self, value: impl Into<String>) -> Self {
        self.response_content = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`NotificationLogDetail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`notification_log_base_fields`](NotificationLogDetailBuilder::notification_log_base_fields)
    /// - [`response_content`](NotificationLogDetailBuilder::response_content)
    pub fn build(self) -> Result<NotificationLogDetail, BuildError> {
        Ok(NotificationLogDetail {
            notification_log_base_fields: self
                .notification_log_base_fields
                .ok_or_else(|| BuildError::missing_field("notification_log_base_fields"))?,
            web_headers: self.web_headers,
            response_headers: self.response_headers,
            response_content: self
                .response_content
                .ok_or_else(|| BuildError::missing_field("response_content"))?,
        })
    }
}
