pub use crate::prelude::*;

/// A single bank-verification result code returned by the verification provider.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VerificationCode {
    /// The numeric result code.
    #[serde(default)]
    pub code: i64,
    /// The short code name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// A human-readable description of the result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

impl VerificationCode {
    pub fn builder() -> VerificationCodeBuilder {
        <VerificationCodeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VerificationCodeBuilder {
    code: Option<i64>,
    name: Option<String>,
    description: Option<String>,
}

impl VerificationCodeBuilder {
    pub fn code(mut self, value: i64) -> Self {
        self.code = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VerificationCode`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](VerificationCodeBuilder::code)
    pub fn build(self) -> Result<VerificationCode, BuildError> {
        Ok(VerificationCode {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            name: self.name,
            description: self.description,
        })
    }
}
