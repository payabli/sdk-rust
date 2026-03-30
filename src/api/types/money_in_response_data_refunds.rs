pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ResponseDataRefunds {
    #[serde(rename = "authCode")]
    #[serde(default)]
    pub auth_code: Authcode,
    #[serde(rename = "expectedProcessingDateTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_processing_date_time: Option<ExpectedProcessingDateTime>,
    /// This field isn't applicable to refund operations.
    #[serde(rename = "avsResponseText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avs_response_text: Option<AvsResponseText>,
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<CustomerId>,
    /// This field isn't applicable to refund operations.
    #[serde(rename = "cvvResponseText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvv_response_text: Option<CvvResponseText>,
    /// This field isn't applicable to refund operations.
    #[serde(rename = "methodReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_reference_id: Option<MethodReferenceId>,
    #[serde(rename = "referenceId")]
    #[serde(default)]
    pub reference_id: Referenceidtrans,
    #[serde(rename = "resultCode")]
    #[serde(default)]
    pub result_code: ResultCode,
    /// Text description of the transaction result
    #[serde(rename = "resultText")]
    #[serde(default)]
    pub result_text: String,
}

impl ResponseDataRefunds {
    pub fn builder() -> ResponseDataRefundsBuilder {
        <ResponseDataRefundsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ResponseDataRefundsBuilder {
    auth_code: Option<Authcode>,
    expected_processing_date_time: Option<ExpectedProcessingDateTime>,
    avs_response_text: Option<AvsResponseText>,
    customer_id: Option<CustomerId>,
    cvv_response_text: Option<CvvResponseText>,
    method_reference_id: Option<MethodReferenceId>,
    reference_id: Option<Referenceidtrans>,
    result_code: Option<ResultCode>,
    result_text: Option<String>,
}

impl ResponseDataRefundsBuilder {
    pub fn auth_code(mut self, value: Authcode) -> Self {
        self.auth_code = Some(value);
        self
    }

    pub fn expected_processing_date_time(mut self, value: ExpectedProcessingDateTime) -> Self {
        self.expected_processing_date_time = Some(value);
        self
    }

    pub fn avs_response_text(mut self, value: AvsResponseText) -> Self {
        self.avs_response_text = Some(value);
        self
    }

    pub fn customer_id(mut self, value: CustomerId) -> Self {
        self.customer_id = Some(value);
        self
    }

    pub fn cvv_response_text(mut self, value: CvvResponseText) -> Self {
        self.cvv_response_text = Some(value);
        self
    }

    pub fn method_reference_id(mut self, value: MethodReferenceId) -> Self {
        self.method_reference_id = Some(value);
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

    pub fn result_text(mut self, value: impl Into<String>) -> Self {
        self.result_text = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ResponseDataRefunds`].
    /// This method will fail if any of the following fields are not set:
    /// - [`auth_code`](ResponseDataRefundsBuilder::auth_code)
    /// - [`reference_id`](ResponseDataRefundsBuilder::reference_id)
    /// - [`result_code`](ResponseDataRefundsBuilder::result_code)
    /// - [`result_text`](ResponseDataRefundsBuilder::result_text)
    pub fn build(self) -> Result<ResponseDataRefunds, BuildError> {
        Ok(ResponseDataRefunds {
            auth_code: self
                .auth_code
                .ok_or_else(|| BuildError::missing_field("auth_code"))?,
            expected_processing_date_time: self.expected_processing_date_time,
            avs_response_text: self.avs_response_text,
            customer_id: self.customer_id,
            cvv_response_text: self.cvv_response_text,
            method_reference_id: self.method_reference_id,
            reference_id: self
                .reference_id
                .ok_or_else(|| BuildError::missing_field("reference_id"))?,
            result_code: self
                .result_code
                .ok_or_else(|| BuildError::missing_field("result_code"))?,
            result_text: self
                .result_text
                .ok_or_else(|| BuildError::missing_field("result_text"))?,
        })
    }
}
