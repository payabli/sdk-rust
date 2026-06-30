pub use crate::prelude::*;

/// Latest AI outreach call activity for a vendor. The populated block depends on the `state` discriminator.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VendorCallStatusResponse {
    /// ID of the vendor this status applies to.
    #[serde(rename = "vendorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<i64>,
    /// Current call state. Values are: `none` (no call activity for the vendor), `scheduled` (a call is queued or being retried), `successful` (a call completed and returned data), or `failed` (the call didn't complete successfully).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Populated when `state` is `scheduled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled: Option<VendorCallStatusScheduled>,
    /// Populated when `state` is `successful`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed: Option<VendorCallStatusCompleted>,
    /// Populated when `state` is `failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed: Option<VendorCallStatusFailed>,
}

impl VendorCallStatusResponse {
    pub fn builder() -> VendorCallStatusResponseBuilder {
        <VendorCallStatusResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VendorCallStatusResponseBuilder {
    vendor_id: Option<i64>,
    state: Option<String>,
    scheduled: Option<VendorCallStatusScheduled>,
    completed: Option<VendorCallStatusCompleted>,
    failed: Option<VendorCallStatusFailed>,
}

impl VendorCallStatusResponseBuilder {
    pub fn vendor_id(mut self, value: i64) -> Self {
        self.vendor_id = Some(value);
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn scheduled(mut self, value: VendorCallStatusScheduled) -> Self {
        self.scheduled = Some(value);
        self
    }

    pub fn completed(mut self, value: VendorCallStatusCompleted) -> Self {
        self.completed = Some(value);
        self
    }

    pub fn failed(mut self, value: VendorCallStatusFailed) -> Self {
        self.failed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VendorCallStatusResponse`].
    pub fn build(self) -> Result<VendorCallStatusResponse, BuildError> {
        Ok(VendorCallStatusResponse {
            vendor_id: self.vendor_id,
            state: self.state,
            scheduled: self.scheduled,
            completed: self.completed,
            failed: self.failed,
        })
    }
}
