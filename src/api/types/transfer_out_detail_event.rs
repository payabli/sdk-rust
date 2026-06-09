pub use crate::prelude::*;

/// Event data for an outbound transfer detail.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransferOutDetailEvent {
    /// Description of the transaction event.
    #[serde(rename = "TransEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_event: Option<String>,
    /// Additional event data.
    #[serde(rename = "EventData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data: Option<String>,
    /// Time the event occurred.
    #[serde(rename = "EventTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<String>,
}

impl TransferOutDetailEvent {
    pub fn builder() -> TransferOutDetailEventBuilder {
        <TransferOutDetailEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferOutDetailEventBuilder {
    trans_event: Option<String>,
    event_data: Option<String>,
    event_time: Option<String>,
}

impl TransferOutDetailEventBuilder {
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

    /// Consumes the builder and constructs a [`TransferOutDetailEvent`].
    pub fn build(self) -> Result<TransferOutDetailEvent, BuildError> {
        Ok(TransferOutDetailEvent {
            trans_event: self.trans_event,
            event_data: self.event_data,
            event_time: self.event_time,
        })
    }
}
