pub use crate::prelude::*;

/// A field that's missing or invalid for the requested services.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ValidationMissingField {
    /// The field's path on the entity record to fix, such as `business.taxReference` or `business.processingMetrics.numberOfTransactions`.
    #[serde(default)]
    pub field: String,
    /// A human-readable name for the field.
    #[serde(default)]
    pub label: String,
    /// A short code for why the field failed, such as `required`.
    #[serde(default)]
    pub reason: String,
}

impl ValidationMissingField {
    pub fn builder() -> ValidationMissingFieldBuilder {
        <ValidationMissingFieldBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ValidationMissingFieldBuilder {
    field: Option<String>,
    label: Option<String>,
    reason: Option<String>,
}

impl ValidationMissingFieldBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ValidationMissingField`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](ValidationMissingFieldBuilder::field)
    /// - [`label`](ValidationMissingFieldBuilder::label)
    /// - [`reason`](ValidationMissingFieldBuilder::reason)
    pub fn build(self) -> Result<ValidationMissingField, BuildError> {
        Ok(ValidationMissingField {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
        })
    }
}
