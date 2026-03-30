pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AuthResponseResponseData {
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
    #[serde(default)]
    pub avs_response_text: AvsResponseText,
    #[serde(rename = "cvvResponseText")]
    #[serde(default)]
    pub cvv_response_text: CvvResponseText,
    #[serde(rename = "customerId")]
    #[serde(default)]
    pub customer_id: Customeridtrans,
    #[serde(rename = "methodReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_reference_id: Option<MethodReferenceId>,
}

impl AuthResponseResponseData {
    pub fn builder() -> AuthResponseResponseDataBuilder {
        <AuthResponseResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthResponseResponseDataBuilder {
    auth_code: Option<Authcode>,
    reference_id: Option<Referenceidtrans>,
    result_code: Option<ResultCode>,
    result_text: Option<Resulttext>,
    avs_response_text: Option<AvsResponseText>,
    cvv_response_text: Option<CvvResponseText>,
    customer_id: Option<Customeridtrans>,
    method_reference_id: Option<MethodReferenceId>,
}

impl AuthResponseResponseDataBuilder {
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

    /// Consumes the builder and constructs a [`AuthResponseResponseData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`auth_code`](AuthResponseResponseDataBuilder::auth_code)
    /// - [`reference_id`](AuthResponseResponseDataBuilder::reference_id)
    /// - [`result_code`](AuthResponseResponseDataBuilder::result_code)
    /// - [`result_text`](AuthResponseResponseDataBuilder::result_text)
    /// - [`avs_response_text`](AuthResponseResponseDataBuilder::avs_response_text)
    /// - [`cvv_response_text`](AuthResponseResponseDataBuilder::cvv_response_text)
    /// - [`customer_id`](AuthResponseResponseDataBuilder::customer_id)
    pub fn build(self) -> Result<AuthResponseResponseData, BuildError> {
        Ok(AuthResponseResponseData {
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
            avs_response_text: self
                .avs_response_text
                .ok_or_else(|| BuildError::missing_field("avs_response_text"))?,
            cvv_response_text: self
                .cvv_response_text
                .ok_or_else(|| BuildError::missing_field("cvv_response_text"))?,
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            method_reference_id: self.method_reference_id,
        })
    }
}
