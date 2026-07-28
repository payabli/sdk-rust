pub use crate::prelude::*;

/// The result of validating a bank account change before creating a case.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PreCreationValidationResult {
    /// Whether the request can be created. False when there are blocking conditions.
    #[serde(rename = "isValid")]
    #[serde(default)]
    pub is_valid: bool,
    /// Conditions that prevent creation. Must be resolved first.
    #[serde(rename = "blockingConditions")]
    #[serde(default)]
    pub blocking_conditions: Vec<String>,
    /// Informational warnings. Creation can still proceed.
    #[serde(default)]
    pub warnings: Vec<String>,
    /// Field-level validation errors.
    #[serde(rename = "validationErrors")]
    #[serde(default)]
    pub validation_errors: Vec<String>,
}

impl PreCreationValidationResult {
    pub fn builder() -> PreCreationValidationResultBuilder {
        <PreCreationValidationResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreCreationValidationResultBuilder {
    is_valid: Option<bool>,
    blocking_conditions: Option<Vec<String>>,
    warnings: Option<Vec<String>>,
    validation_errors: Option<Vec<String>>,
}

impl PreCreationValidationResultBuilder {
    pub fn is_valid(mut self, value: bool) -> Self {
        self.is_valid = Some(value);
        self
    }

    pub fn blocking_conditions(mut self, value: Vec<String>) -> Self {
        self.blocking_conditions = Some(value);
        self
    }

    pub fn warnings(mut self, value: Vec<String>) -> Self {
        self.warnings = Some(value);
        self
    }

    pub fn validation_errors(mut self, value: Vec<String>) -> Self {
        self.validation_errors = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PreCreationValidationResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_valid`](PreCreationValidationResultBuilder::is_valid)
    /// - [`blocking_conditions`](PreCreationValidationResultBuilder::blocking_conditions)
    /// - [`warnings`](PreCreationValidationResultBuilder::warnings)
    /// - [`validation_errors`](PreCreationValidationResultBuilder::validation_errors)
    pub fn build(self) -> Result<PreCreationValidationResult, BuildError> {
        Ok(PreCreationValidationResult {
            is_valid: self
                .is_valid
                .ok_or_else(|| BuildError::missing_field("is_valid"))?,
            blocking_conditions: self
                .blocking_conditions
                .ok_or_else(|| BuildError::missing_field("blocking_conditions"))?,
            warnings: self
                .warnings
                .ok_or_else(|| BuildError::missing_field("warnings"))?,
            validation_errors: self
                .validation_errors
                .ok_or_else(|| BuildError::missing_field("validation_errors"))?,
        })
    }
}
