pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayabliApiResponsePaymethodDeleteResponseData {
    /// The method's reference ID.
    #[serde(rename = "referenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<MethodReferenceId>,
    #[serde(rename = "resultCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_code: Option<ResultCode>,
    #[serde(rename = "resultText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_text: Option<Resulttext>,
}

impl PayabliApiResponsePaymethodDeleteResponseData {
    pub fn builder() -> PayabliApiResponsePaymethodDeleteResponseDataBuilder {
        <PayabliApiResponsePaymethodDeleteResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayabliApiResponsePaymethodDeleteResponseDataBuilder {
    reference_id: Option<MethodReferenceId>,
    result_code: Option<ResultCode>,
    result_text: Option<Resulttext>,
}

impl PayabliApiResponsePaymethodDeleteResponseDataBuilder {
    pub fn reference_id(mut self, value: MethodReferenceId) -> Self {
        self.reference_id = Some(value);
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

    /// Consumes the builder and constructs a [`PayabliApiResponsePaymethodDeleteResponseData`].
    pub fn build(self) -> Result<PayabliApiResponsePaymethodDeleteResponseData, BuildError> {
        Ok(PayabliApiResponsePaymethodDeleteResponseData {
            reference_id: self.reference_id,
            result_code: self.result_code,
            result_text: self.result_text,
        })
    }
}
