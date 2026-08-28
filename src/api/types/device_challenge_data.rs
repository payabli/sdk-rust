pub use crate::prelude::*;

/// The issued activation code and the time it expires.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeviceChallengeData {
    /// The 6-digit verification code the operator enters on the device's
    /// terminal to activate it. It can start with leading zeros, so keep it as
    /// a string.
    #[serde(default)]
    pub code: String,
    /// UTC time when the code expires, in ISO-8601 round-trip format. A code is
    /// valid for 5 minutes after it's issued.
    #[serde(rename = "expiresAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub expires_at: DateTime<Utc>,
}

impl DeviceChallengeData {
    pub fn builder() -> DeviceChallengeDataBuilder {
        <DeviceChallengeDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeviceChallengeDataBuilder {
    code: Option<String>,
    expires_at: Option<DateTime<Utc>>,
}

impl DeviceChallengeDataBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeviceChallengeData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](DeviceChallengeDataBuilder::code)
    /// - [`expires_at`](DeviceChallengeDataBuilder::expires_at)
    pub fn build(self) -> Result<DeviceChallengeData, BuildError> {
        Ok(DeviceChallengeData {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
        })
    }
}
