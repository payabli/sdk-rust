pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayabliApiResponse0ResponseData {
    #[serde(rename = "AuthCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<Authcode>,
    #[serde(rename = "avsResponseText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avs_response_text: Option<AvsResponseText>,
    #[serde(rename = "CustomerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<Customeridtrans>,
    #[serde(rename = "cvvResponseText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvv_response_text: Option<CvvResponseText>,
    #[serde(rename = "methodReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_reference_id: Option<MethodReferenceId>,
    #[serde(rename = "ReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<Referenceidtrans>,
    #[serde(rename = "ResultCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_code: Option<ResultCode>,
    #[serde(rename = "ResultText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_text: Option<Resulttext>,
}

impl PayabliApiResponse0ResponseData {
    pub fn builder() -> PayabliApiResponse0ResponseDataBuilder {
        <PayabliApiResponse0ResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayabliApiResponse0ResponseDataBuilder {
    auth_code: Option<Authcode>,
    avs_response_text: Option<AvsResponseText>,
    customer_id: Option<Customeridtrans>,
    cvv_response_text: Option<CvvResponseText>,
    method_reference_id: Option<MethodReferenceId>,
    reference_id: Option<Referenceidtrans>,
    result_code: Option<ResultCode>,
    result_text: Option<Resulttext>,
}

impl PayabliApiResponse0ResponseDataBuilder {
    pub fn auth_code(mut self, value: Authcode) -> Self {
        self.auth_code = Some(value);
        self
    }

    pub fn avs_response_text(mut self, value: AvsResponseText) -> Self {
        self.avs_response_text = Some(value);
        self
    }

    pub fn customer_id(mut self, value: Customeridtrans) -> Self {
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

    pub fn result_text(mut self, value: Resulttext) -> Self {
        self.result_text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayabliApiResponse0ResponseData`].
    pub fn build(self) -> Result<PayabliApiResponse0ResponseData, BuildError> {
        Ok(PayabliApiResponse0ResponseData {
            auth_code: self.auth_code,
            avs_response_text: self.avs_response_text,
            customer_id: self.customer_id,
            cvv_response_text: self.cvv_response_text,
            method_reference_id: self.method_reference_id,
            reference_id: self.reference_id,
            result_code: self.result_code,
            result_text: self.result_text,
        })
    }
}
