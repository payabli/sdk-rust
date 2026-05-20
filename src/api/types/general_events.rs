pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GeneralEvents {
    /// Event description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Event timestamp, in UTC.
    #[serde(rename = "eventTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub event_time: Option<DateTime<Utc>>,
    /// Extra data.
    #[serde(rename = "extraData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra_data: Option<String>,
    /// Reference data.
    #[serde(rename = "refData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_data: Option<String>,
    /// The event source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
}

impl GeneralEvents {
    pub fn builder() -> GeneralEventsBuilder {
        <GeneralEventsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GeneralEventsBuilder {
    description: Option<String>,
    event_time: Option<DateTime<Utc>>,
    extra_data: Option<String>,
    ref_data: Option<String>,
    source: Option<Source>,
}

impl GeneralEventsBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn event_time(mut self, value: DateTime<Utc>) -> Self {
        self.event_time = Some(value);
        self
    }

    pub fn extra_data(mut self, value: impl Into<String>) -> Self {
        self.extra_data = Some(value.into());
        self
    }

    pub fn ref_data(mut self, value: impl Into<String>) -> Self {
        self.ref_data = Some(value.into());
        self
    }

    pub fn source(mut self, value: Source) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GeneralEvents`].
    pub fn build(self) -> Result<GeneralEvents, BuildError> {
        Ok(GeneralEvents {
            description: self.description,
            event_time: self.event_time,
            extra_data: self.extra_data,
            ref_data: self.ref_data,
            source: self.source,
        })
    }
}
