pub use crate::prelude::*;

/// Event associated with transaction processing
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransactionDetailEvent {
    #[serde(rename = "transEvent")]
    #[serde(default)]
    pub trans_event: String,
    #[serde(rename = "eventData")]
    #[serde(default)]
    pub event_data: String,
    #[serde(rename = "eventTime")]
    #[serde(default)]
    pub event_time: String,
}

impl TransactionDetailEvent {
    pub fn builder() -> TransactionDetailEventBuilder {
        <TransactionDetailEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionDetailEventBuilder {
    trans_event: Option<String>,
    event_data: Option<String>,
    event_time: Option<String>,
}

impl TransactionDetailEventBuilder {
    pub fn trans_event(mut self, value: impl Into<String>) -> Self {
        self.trans_event = Some(value.into());
        self
    }

    pub fn event_data(mut self, value: impl Into<String>) -> Self {
        self.event_data = Some(value.into());
        self
    }

    pub fn event_time(mut self, value: impl Into<String>) -> Self {
        self.event_time = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransactionDetailEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`trans_event`](TransactionDetailEventBuilder::trans_event)
    /// - [`event_data`](TransactionDetailEventBuilder::event_data)
    /// - [`event_time`](TransactionDetailEventBuilder::event_time)
    pub fn build(self) -> Result<TransactionDetailEvent, BuildError> {
        Ok(TransactionDetailEvent {
            trans_event: self
                .trans_event
                .ok_or_else(|| BuildError::missing_field("trans_event"))?,
            event_data: self
                .event_data
                .ok_or_else(|| BuildError::missing_field("event_data"))?,
            event_time: self
                .event_time
                .ok_or_else(|| BuildError::missing_field("event_time"))?,
        })
    }
}
