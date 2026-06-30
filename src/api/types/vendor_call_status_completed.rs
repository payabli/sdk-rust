pub use crate::prelude::*;

/// Details of a completed outreach call that returned data.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VendorCallStatusCompleted {
    /// ISO-8601 timestamp when the call ended.
    #[serde(rename = "completedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<String>,
    /// Call duration in seconds.
    #[serde(rename = "durationSeconds")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_seconds: Option<i64>,
    /// Short summary of the call.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    /// Reference identifier for the call.
    #[serde(rename = "callId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_id: Option<String>,
    /// Full call transcript. `null` when no transcript is available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcript: Option<String>,
    /// Payment and contact details collected during the call.
    #[serde(rename = "extractedData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extracted_data: Option<VendorCallStatusExtractedData>,
}

impl VendorCallStatusCompleted {
    pub fn builder() -> VendorCallStatusCompletedBuilder {
        <VendorCallStatusCompletedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VendorCallStatusCompletedBuilder {
    completed_at: Option<String>,
    duration_seconds: Option<i64>,
    summary: Option<String>,
    call_id: Option<String>,
    transcript: Option<String>,
    extracted_data: Option<VendorCallStatusExtractedData>,
}

impl VendorCallStatusCompletedBuilder {
    pub fn completed_at(mut self, value: impl Into<String>) -> Self {
        self.completed_at = Some(value.into());
        self
    }

    pub fn duration_seconds(mut self, value: i64) -> Self {
        self.duration_seconds = Some(value);
        self
    }

    pub fn summary(mut self, value: impl Into<String>) -> Self {
        self.summary = Some(value.into());
        self
    }

    pub fn call_id(mut self, value: impl Into<String>) -> Self {
        self.call_id = Some(value.into());
        self
    }

    pub fn transcript(mut self, value: impl Into<String>) -> Self {
        self.transcript = Some(value.into());
        self
    }

    pub fn extracted_data(mut self, value: VendorCallStatusExtractedData) -> Self {
        self.extracted_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VendorCallStatusCompleted`].
    pub fn build(self) -> Result<VendorCallStatusCompleted, BuildError> {
        Ok(VendorCallStatusCompleted {
            completed_at: self.completed_at,
            duration_seconds: self.duration_seconds,
            summary: self.summary,
            call_id: self.call_id,
            transcript: self.transcript,
            extracted_data: self.extracted_data,
        })
    }
}
