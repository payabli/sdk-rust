pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryTransactionEvents {
    /// Event descriptor. See [TransEvent Reference](/guides/pay-in-transevents-reference) for more details.
    #[serde(rename = "TransEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_event: Option<String>,
    /// Any data associated to the event received from processor. Contents vary by event type.
    #[serde(rename = "EventData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_data: Option<QueryTransactionEventsEventData>,
    /// Date and time of event.
    #[serde(rename = "EventTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub event_time: Option<DateTime<Utc>>,
}

impl QueryTransactionEvents {
    pub fn builder() -> QueryTransactionEventsBuilder {
        <QueryTransactionEventsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryTransactionEventsBuilder {
    trans_event: Option<String>,
    event_data: Option<QueryTransactionEventsEventData>,
    event_time: Option<DateTime<Utc>>,
}

impl QueryTransactionEventsBuilder {
    pub fn trans_event(mut self, value: impl Into<String>) -> Self {
        self.trans_event = Some(value.into());
        self
    }

    pub fn event_data(mut self, value: QueryTransactionEventsEventData) -> Self {
        self.event_data = Some(value);
        self
    }

    pub fn event_time(mut self, value: DateTime<Utc>) -> Self {
        self.event_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryTransactionEvents`].
    pub fn build(self) -> Result<QueryTransactionEvents, BuildError> {
        Ok(QueryTransactionEvents {
            trans_event: self.trans_event,
            event_data: self.event_data,
            event_time: self.event_time,
        })
    }
}
