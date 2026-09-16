pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TapToPayActivationChallengeRequest {
    #[serde(default)]
    pub entry: Entry,
    /// The device identifier (`poiId`) returned when the device was registered.
    #[serde(rename = "deviceId")]
    #[serde(default)]
    pub device_id: String,
}

impl TapToPayActivationChallengeRequest {
    pub fn builder() -> TapToPayActivationChallengeRequestBuilder {
        <TapToPayActivationChallengeRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TapToPayActivationChallengeRequestBuilder {
    entry: Option<Entry>,
    device_id: Option<String>,
}

impl TapToPayActivationChallengeRequestBuilder {
    pub fn entry(mut self, value: Entry) -> Self {
        self.entry = Some(value);
        self
    }

    pub fn device_id(mut self, value: impl Into<String>) -> Self {
        self.device_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TapToPayActivationChallengeRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entry`](TapToPayActivationChallengeRequestBuilder::entry)
    /// - [`device_id`](TapToPayActivationChallengeRequestBuilder::device_id)
    pub fn build(self) -> Result<TapToPayActivationChallengeRequest, BuildError> {
        Ok(TapToPayActivationChallengeRequest {
            entry: self
                .entry
                .ok_or_else(|| BuildError::missing_field("entry"))?,
            device_id: self
                .device_id
                .ok_or_else(|| BuildError::missing_field("device_id"))?,
        })
    }
}
