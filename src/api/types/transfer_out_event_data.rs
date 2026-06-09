pub use crate::prelude::*;

/// Event data associated with an outbound transfer.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TransferOutEventData {
    /// Description of the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The time the event occurred.
    #[serde(rename = "eventTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<String>,
    /// Reference data for the event.
    #[serde(rename = "refData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_data: Option<String>,
    /// Additional event data, which may contain detailed transaction information.
    #[serde(rename = "extraData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_data: Option<serde_json::Value>,
    /// The source of the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

impl TransferOutEventData {
    pub fn builder() -> TransferOutEventDataBuilder {
        <TransferOutEventDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferOutEventDataBuilder {
    description: Option<String>,
    event_time: Option<String>,
    ref_data: Option<String>,
    extra_data: Option<serde_json::Value>,
    source: Option<String>,
}

impl TransferOutEventDataBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn event_time(mut self, value: impl Into<String>) -> Self {
        self.event_time = Some(value.into());
        self
    }

    pub fn ref_data(mut self, value: impl Into<String>) -> Self {
        self.ref_data = Some(value.into());
        self
    }

    pub fn extra_data(mut self, value: serde_json::Value) -> Self {
        self.extra_data = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransferOutEventData`].
    pub fn build(self) -> Result<TransferOutEventData, BuildError> {
        Ok(TransferOutEventData {
            description: self.description,
            event_time: self.event_time,
            ref_data: self.ref_data,
            extra_data: self.extra_data,
            source: self.source,
        })
    }
}
