pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MfaValidationData {
    #[serde(rename = "mfaCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_code: Option<String>,
    #[serde(rename = "mfaValidationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_validation_code: Option<MfaValidationCode>,
}

impl MfaValidationData {
    pub fn builder() -> MfaValidationDataBuilder {
        <MfaValidationDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MfaValidationDataBuilder {
    mfa_code: Option<String>,
    mfa_validation_code: Option<MfaValidationCode>,
}

impl MfaValidationDataBuilder {
    pub fn mfa_code(mut self, value: impl Into<String>) -> Self {
        self.mfa_code = Some(value.into());
        self
    }

    pub fn mfa_validation_code(mut self, value: MfaValidationCode) -> Self {
        self.mfa_validation_code = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MfaValidationData`].
    pub fn build(self) -> Result<MfaValidationData, BuildError> {
        Ok(MfaValidationData {
            mfa_code: self.mfa_code,
            mfa_validation_code: self.mfa_validation_code,
        })
    }
}
