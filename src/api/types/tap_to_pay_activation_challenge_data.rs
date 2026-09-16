pub use crate::prelude::*;

/// The issued activation code, its expiration, and whether it was reused.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TapToPayActivationChallengeData {
    /// The 6-digit activation code the partner delivers to the device
    /// user to activate the device. It can start with leading zeros, so
    /// keep it as a string.
    #[serde(default)]
    pub code: String,
    /// UTC time when the code expires, in ISO 8601 round-trip format. A
    /// code is valid for 30 minutes after it's issued.
    #[serde(rename = "expiresAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub expires_at: DateTime<Utc>,
    /// `true` when an unexpired code already exists for the device and
    /// this call returns it unchanged instead of generating a new one.
    #[serde(rename = "alreadyIssued")]
    #[serde(default)]
    pub already_issued: bool,
}

impl TapToPayActivationChallengeData {
    pub fn builder() -> TapToPayActivationChallengeDataBuilder {
        <TapToPayActivationChallengeDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TapToPayActivationChallengeDataBuilder {
    code: Option<String>,
    expires_at: Option<DateTime<Utc>>,
    already_issued: Option<bool>,
}

impl TapToPayActivationChallengeDataBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn expires_at(mut self, value: DateTime<Utc>) -> Self {
        self.expires_at = Some(value);
        self
    }

    pub fn already_issued(mut self, value: bool) -> Self {
        self.already_issued = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TapToPayActivationChallengeData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](TapToPayActivationChallengeDataBuilder::code)
    /// - [`expires_at`](TapToPayActivationChallengeDataBuilder::expires_at)
    /// - [`already_issued`](TapToPayActivationChallengeDataBuilder::already_issued)
    pub fn build(self) -> Result<TapToPayActivationChallengeData, BuildError> {
        Ok(TapToPayActivationChallengeData {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            expires_at: self
                .expires_at
                .ok_or_else(|| BuildError::missing_field("expires_at"))?,
            already_issued: self
                .already_issued
                .ok_or_else(|| BuildError::missing_field("already_issued"))?,
        })
    }
}
