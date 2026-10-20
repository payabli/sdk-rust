pub use crate::prelude::*;

/// Whether the application's stored records meet the requirements for the services it requests.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ValidationResult {
    /// Whether nothing is missing or invalid in any section.
    #[serde(default)]
    pub valid: bool,
    /// The requirement groups that validation checked, named with Payabli's internal service names (for example, `payInCard` for the `Card` Pay In service).
    #[serde(rename = "servicesEvaluated")]
    #[serde(default)]
    pub services_evaluated: Vec<String>,
    #[serde(default)]
    pub sections: ValidationSections,
}

impl ValidationResult {
    pub fn builder() -> ValidationResultBuilder {
        <ValidationResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ValidationResultBuilder {
    valid: Option<bool>,
    services_evaluated: Option<Vec<String>>,
    sections: Option<ValidationSections>,
}

impl ValidationResultBuilder {
    pub fn valid(mut self, value: bool) -> Self {
        self.valid = Some(value);
        self
    }

    pub fn services_evaluated(mut self, value: Vec<String>) -> Self {
        self.services_evaluated = Some(value);
        self
    }

    pub fn sections(mut self, value: ValidationSections) -> Self {
        self.sections = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ValidationResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`valid`](ValidationResultBuilder::valid)
    /// - [`services_evaluated`](ValidationResultBuilder::services_evaluated)
    /// - [`sections`](ValidationResultBuilder::sections)
    pub fn build(self) -> Result<ValidationResult, BuildError> {
        Ok(ValidationResult {
            valid: self
                .valid
                .ok_or_else(|| BuildError::missing_field("valid"))?,
            services_evaluated: self
                .services_evaluated
                .ok_or_else(|| BuildError::missing_field("services_evaluated"))?,
            sections: self
                .sections
                .ok_or_else(|| BuildError::missing_field("sections"))?,
        })
    }
}
