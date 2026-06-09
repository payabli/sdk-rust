pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateGhostCardResponseData {
    /// Card token for the ghost card. Use this value to reference the card in subsequent operations (update, cancel, etc.).
    #[serde(rename = "ReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<String>,
    #[serde(rename = "ResultCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_code: Option<ResultCode>,
    #[serde(rename = "ResultText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_text: Option<Resulttext>,
}

impl CreateGhostCardResponseData {
    pub fn builder() -> CreateGhostCardResponseDataBuilder {
        <CreateGhostCardResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateGhostCardResponseDataBuilder {
    reference_id: Option<String>,
    result_code: Option<ResultCode>,
    result_text: Option<Resulttext>,
}

impl CreateGhostCardResponseDataBuilder {
    pub fn reference_id(mut self, value: impl Into<String>) -> Self {
        self.reference_id = Some(value.into());
        self
    }

    pub fn result_code(mut self, value: ResultCode) -> Self {
        self.result_code = Some(value);
        self
    }

    pub fn result_text(mut self, value: Resulttext) -> Self {
        self.result_text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateGhostCardResponseData`].
    pub fn build(self) -> Result<CreateGhostCardResponseData, BuildError> {
        Ok(CreateGhostCardResponseData {
            reference_id: self.reference_id,
            result_code: self.result_code,
            result_text: self.result_text,
        })
    }
}
