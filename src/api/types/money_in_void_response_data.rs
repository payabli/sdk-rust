pub use crate::prelude::*;

/// Response data for void transactions
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VoidResponseData {
    #[serde(rename = "authCode")]
    #[serde(default)]
    pub auth_code: Authcode,
    #[serde(rename = "referenceId")]
    #[serde(default)]
    pub reference_id: Referenceidtrans,
    #[serde(rename = "resultCode")]
    #[serde(default)]
    pub result_code: ResultCode,
    #[serde(rename = "resultText")]
    #[serde(default)]
    pub result_text: Resulttext,
    #[serde(rename = "avsResponseText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avs_response_text: Option<AvsResponseText>,
    #[serde(rename = "cvvResponseText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvv_response_text: Option<CvvResponseText>,
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<Customeridtrans>,
    #[serde(rename = "methodReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_reference_id: Option<MethodReferenceId>,
}

impl VoidResponseData {
    pub fn builder() -> VoidResponseDataBuilder {
        <VoidResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VoidResponseDataBuilder {
    auth_code: Option<Authcode>,
    reference_id: Option<Referenceidtrans>,
    result_code: Option<ResultCode>,
    result_text: Option<Resulttext>,
    avs_response_text: Option<AvsResponseText>,
    cvv_response_text: Option<CvvResponseText>,
    customer_id: Option<Customeridtrans>,
    method_reference_id: Option<MethodReferenceId>,
}

impl VoidResponseDataBuilder {
    pub fn auth_code(mut self, value: Authcode) -> Self {
        self.auth_code = Some(value);
        self
    }

    pub fn reference_id(mut self, value: Referenceidtrans) -> Self {
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

    pub fn avs_response_text(mut self, value: AvsResponseText) -> Self {
        self.avs_response_text = Some(value);
        self
    }

    pub fn cvv_response_text(mut self, value: CvvResponseText) -> Self {
        self.cvv_response_text = Some(value);
        self
    }

    pub fn customer_id(mut self, value: Customeridtrans) -> Self {
        self.customer_id = Some(value);
        self
    }

    pub fn method_reference_id(mut self, value: MethodReferenceId) -> Self {
        self.method_reference_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VoidResponseData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`auth_code`](VoidResponseDataBuilder::auth_code)
    /// - [`reference_id`](VoidResponseDataBuilder::reference_id)
    /// - [`result_code`](VoidResponseDataBuilder::result_code)
    /// - [`result_text`](VoidResponseDataBuilder::result_text)
    pub fn build(self) -> Result<VoidResponseData, BuildError> {
        Ok(VoidResponseData {
            auth_code: self
                .auth_code
                .ok_or_else(|| BuildError::missing_field("auth_code"))?,
            reference_id: self
                .reference_id
                .ok_or_else(|| BuildError::missing_field("reference_id"))?,
            result_code: self
                .result_code
                .ok_or_else(|| BuildError::missing_field("result_code"))?,
            result_text: self
                .result_text
                .ok_or_else(|| BuildError::missing_field("result_text"))?,
            avs_response_text: self.avs_response_text,
            cvv_response_text: self.cvv_response_text,
            customer_id: self.customer_id,
            method_reference_id: self.method_reference_id,
        })
    }
}
