pub use crate::prelude::*;

/// The validation result for one section of the application.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ValidationSection {
    /// Whether the section has no missing or invalid fields.
    #[serde(default)]
    pub complete: bool,
    /// The fields that are missing or invalid in this section. Empty when the section is complete.
    #[serde(rename = "missingFields")]
    #[serde(default)]
    pub missing_fields: Vec<ValidationMissingField>,
}

impl ValidationSection {
    pub fn builder() -> ValidationSectionBuilder {
        <ValidationSectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ValidationSectionBuilder {
    complete: Option<bool>,
    missing_fields: Option<Vec<ValidationMissingField>>,
}

impl ValidationSectionBuilder {
    pub fn complete(mut self, value: bool) -> Self {
        self.complete = Some(value);
        self
    }

    pub fn missing_fields(mut self, value: Vec<ValidationMissingField>) -> Self {
        self.missing_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ValidationSection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`complete`](ValidationSectionBuilder::complete)
    /// - [`missing_fields`](ValidationSectionBuilder::missing_fields)
    pub fn build(self) -> Result<ValidationSection, BuildError> {
        Ok(ValidationSection {
            complete: self
                .complete
                .ok_or_else(|| BuildError::missing_field("complete"))?,
            missing_fields: self
                .missing_fields
                .ok_or_else(|| BuildError::missing_field("missing_fields"))?,
        })
    }
}
