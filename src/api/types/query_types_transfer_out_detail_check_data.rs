pub use crate::prelude::*;

/// Check data for an outbound transfer detail.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransferOutDetailCheckData {
    /// The check number.
    #[serde(rename = "CheckNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_number: Option<String>,
    /// Additional check data.
    #[serde(rename = "CheckData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_data: Option<String>,
}

impl TransferOutDetailCheckData {
    pub fn builder() -> TransferOutDetailCheckDataBuilder {
        <TransferOutDetailCheckDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferOutDetailCheckDataBuilder {
    check_number: Option<String>,
    check_data: Option<String>,
}

impl TransferOutDetailCheckDataBuilder {
    pub fn check_number(mut self, value: impl Into<String>) -> Self {
        self.check_number = Some(value.into());
        self
    }

    pub fn check_data(mut self, value: impl Into<String>) -> Self {
        self.check_data = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransferOutDetailCheckData`].
    pub fn build(self) -> Result<TransferOutDetailCheckData, BuildError> {
        Ok(TransferOutDetailCheckData {
            check_number: self.check_number,
            check_data: self.check_data,
        })
    }
}
