pub use crate::prelude::*;

/// A single transition in an application's status history.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StatusHistoryEntry {
    /// The status before the transition. The transition that creates the application has a `fromStatus` of `none`.
    #[serde(rename = "fromStatus")]
    #[serde(default)]
    pub from_status: String,
    /// The status after the transition.
    #[serde(rename = "toStatus")]
    #[serde(default)]
    pub to_status: String,
    /// What triggered the transition (for example, `Create` or `Withdraw`).
    #[serde(default)]
    pub trigger: String,
    /// The identifier of the actor that triggered the transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actor: Option<String>,
    /// The reason recorded for the transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Additional metadata about the transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// When the transition occurred.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
}

impl StatusHistoryEntry {
    pub fn builder() -> StatusHistoryEntryBuilder {
        <StatusHistoryEntryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatusHistoryEntryBuilder {
    from_status: Option<String>,
    to_status: Option<String>,
    trigger: Option<String>,
    actor: Option<String>,
    reason: Option<String>,
    metadata: Option<HashMap<String, serde_json::Value>>,
    created_at: Option<String>,
}

impl StatusHistoryEntryBuilder {
    pub fn from_status(mut self, value: impl Into<String>) -> Self {
        self.from_status = Some(value.into());
        self
    }

    pub fn to_status(mut self, value: impl Into<String>) -> Self {
        self.to_status = Some(value.into());
        self
    }

    pub fn trigger(mut self, value: impl Into<String>) -> Self {
        self.trigger = Some(value.into());
        self
    }

    pub fn actor(mut self, value: impl Into<String>) -> Self {
        self.actor = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn metadata(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`StatusHistoryEntry`].
    /// This method will fail if any of the following fields are not set:
    /// - [`from_status`](StatusHistoryEntryBuilder::from_status)
    /// - [`to_status`](StatusHistoryEntryBuilder::to_status)
    /// - [`trigger`](StatusHistoryEntryBuilder::trigger)
    /// - [`created_at`](StatusHistoryEntryBuilder::created_at)
    pub fn build(self) -> Result<StatusHistoryEntry, BuildError> {
        Ok(StatusHistoryEntry {
            from_status: self
                .from_status
                .ok_or_else(|| BuildError::missing_field("from_status"))?,
            to_status: self
                .to_status
                .ok_or_else(|| BuildError::missing_field("to_status"))?,
            trigger: self
                .trigger
                .ok_or_else(|| BuildError::missing_field("trigger"))?,
            actor: self.actor,
            reason: self.reason,
            metadata: self.metadata,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
        })
    }
}
