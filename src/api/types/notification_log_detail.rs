pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NotificationLogDetail {
    #[serde(flatten)]
    pub notification_log_fields: NotificationLog,
    #[serde(rename = "webHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_headers: Option<Vec<StringStringKeyValuePair>>,
    #[serde(rename = "responseHeaders")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_headers: Option<Vec<KeyValueArray>>,
    #[serde(rename = "responseContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_content: Option<String>,
}

impl NotificationLogDetail {
    pub fn builder() -> NotificationLogDetailBuilder {
        <NotificationLogDetailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotificationLogDetailBuilder {
    notification_log_fields: Option<NotificationLog>,
    web_headers: Option<Vec<StringStringKeyValuePair>>,
    response_headers: Option<Vec<KeyValueArray>>,
    response_content: Option<String>,
}

impl NotificationLogDetailBuilder {
    pub fn notification_log_fields(mut self, value: NotificationLog) -> Self {
        self.notification_log_fields = Some(value);
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
    /// - [`notification_log_fields`](NotificationLogDetailBuilder::notification_log_fields)
    pub fn build(self) -> Result<NotificationLogDetail, BuildError> {
        Ok(NotificationLogDetail {
            notification_log_fields: self
                .notification_log_fields
                .ok_or_else(|| BuildError::missing_field("notification_log_fields"))?,
            web_headers: self.web_headers,
            response_headers: self.response_headers,
            response_content: self.response_content,
        })
    }
}
