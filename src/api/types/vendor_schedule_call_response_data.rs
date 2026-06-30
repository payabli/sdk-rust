pub use crate::prelude::*;

/// Scheduled call details.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VendorScheduleCallResponseData {
    /// Identifier for the scheduled call.
    #[serde(rename = "callScheduleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_schedule_id: Option<i64>,
    /// ID of the enrichment run associated with this call. When the request omits `enrichmentId`, Payabli generates one and returns it here.
    #[serde(rename = "enrichmentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enrichment_id: Option<String>,
    /// ISO-8601 timestamp of the next scheduled call attempt.
    #[serde(rename = "scheduledCallDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_call_date: Option<String>,
    /// Status of the call schedule. Values are `pending`, `dispatched`, `retry_scheduled`, `completed`, and `fallback_applied`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl VendorScheduleCallResponseData {
    pub fn builder() -> VendorScheduleCallResponseDataBuilder {
        <VendorScheduleCallResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VendorScheduleCallResponseDataBuilder {
    call_schedule_id: Option<i64>,
    enrichment_id: Option<String>,
    scheduled_call_date: Option<String>,
    status: Option<String>,
}

impl VendorScheduleCallResponseDataBuilder {
    pub fn call_schedule_id(mut self, value: i64) -> Self {
        self.call_schedule_id = Some(value);
        self
    }

    pub fn enrichment_id(mut self, value: impl Into<String>) -> Self {
        self.enrichment_id = Some(value.into());
        self
    }

    pub fn scheduled_call_date(mut self, value: impl Into<String>) -> Self {
        self.scheduled_call_date = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VendorScheduleCallResponseData`].
    pub fn build(self) -> Result<VendorScheduleCallResponseData, BuildError> {
        Ok(VendorScheduleCallResponseData {
            call_schedule_id: self.call_schedule_id,
            enrichment_id: self.enrichment_id,
            scheduled_call_date: self.scheduled_call_date,
            status: self.status,
        })
    }
}
