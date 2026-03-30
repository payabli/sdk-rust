pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NotificationStandardRequestContent {
    /// The notification's event name.
    #[serde(rename = "eventType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_type: Option<NotificationStandardRequestContentEventType>,
    /// Array of pairs key:value to insert in request body to target in **method** = *web*.
    #[serde(rename = "internalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub internal_data: Option<Vec<KeyValueDuo>>,
    /// Used internally to reference the entity or object generating the event.
    #[serde(rename = "transactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// Array of pairs key:value to insert in header of request to target in **method** = *web*.
    #[serde(rename = "webHeaderParameters")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_header_parameters: Option<Vec<KeyValueDuo>>,
}

impl NotificationStandardRequestContent {
    pub fn builder() -> NotificationStandardRequestContentBuilder {
        <NotificationStandardRequestContentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotificationStandardRequestContentBuilder {
    event_type: Option<NotificationStandardRequestContentEventType>,
    internal_data: Option<Vec<KeyValueDuo>>,
    transaction_id: Option<String>,
    web_header_parameters: Option<Vec<KeyValueDuo>>,
}

impl NotificationStandardRequestContentBuilder {
    pub fn event_type(mut self, value: NotificationStandardRequestContentEventType) -> Self {
        self.event_type = Some(value);
        self
    }

    pub fn internal_data(mut self, value: Vec<KeyValueDuo>) -> Self {
        self.internal_data = Some(value);
        self
    }

    pub fn transaction_id(mut self, value: impl Into<String>) -> Self {
        self.transaction_id = Some(value.into());
        self
    }

    pub fn web_header_parameters(mut self, value: Vec<KeyValueDuo>) -> Self {
        self.web_header_parameters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NotificationStandardRequestContent`].
    pub fn build(self) -> Result<NotificationStandardRequestContent, BuildError> {
        Ok(NotificationStandardRequestContent {
            event_type: self.event_type,
            internal_data: self.internal_data,
            transaction_id: self.transaction_id,
            web_header_parameters: self.web_header_parameters,
        })
    }
}
