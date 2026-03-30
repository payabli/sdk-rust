pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MfaData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa: Option<bool>,
    #[serde(rename = "mfaMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_mode: Option<MfaMode>,
}

impl MfaData {
    pub fn builder() -> MfaDataBuilder {
        <MfaDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MfaDataBuilder {
    mfa: Option<bool>,
    mfa_mode: Option<MfaMode>,
}

impl MfaDataBuilder {
    pub fn mfa(mut self, value: bool) -> Self {
        self.mfa = Some(value);
        self
    }

    pub fn mfa_mode(mut self, value: MfaMode) -> Self {
        self.mfa_mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MfaData`].
    pub fn build(self) -> Result<MfaData, BuildError> {
        Ok(MfaData {
            mfa: self.mfa,
            mfa_mode: self.mfa_mode,
        })
    }
}
