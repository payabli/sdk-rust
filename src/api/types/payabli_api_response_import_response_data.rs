pub use crate::prelude::*;

/// The response data containing the result of the import operation.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayabliApiResponseImportResponseData {
    /// The number of records successfully added.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub added: Option<i64>,
    /// List of errors, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<String>>,
    /// The number of records that were rejected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejected: Option<i64>,
}

impl PayabliApiResponseImportResponseData {
    pub fn builder() -> PayabliApiResponseImportResponseDataBuilder {
        <PayabliApiResponseImportResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayabliApiResponseImportResponseDataBuilder {
    added: Option<i64>,
    errors: Option<Vec<String>>,
    rejected: Option<i64>,
}

impl PayabliApiResponseImportResponseDataBuilder {
    pub fn added(mut self, value: i64) -> Self {
        self.added = Some(value);
        self
    }

    pub fn errors(mut self, value: Vec<String>) -> Self {
        self.errors = Some(value);
        self
    }

    pub fn rejected(mut self, value: i64) -> Self {
        self.rejected = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayabliApiResponseImportResponseData`].
    pub fn build(self) -> Result<PayabliApiResponseImportResponseData, BuildError> {
        Ok(PayabliApiResponseImportResponseData {
            added: self.added,
            errors: self.errors,
            rejected: self.rejected,
        })
    }
}
