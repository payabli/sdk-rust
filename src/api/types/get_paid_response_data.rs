pub use crate::prelude::*;

/// Response data for GetPaid transactions
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetPaidResponseData {
    #[serde(rename = "authCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<Authcode>,
    /// Details of the transaction. Present only if `includeDetails` query parameter is set to `true` in the request.
    #[serde(rename = "transactionDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_details: Option<TransactionDetailRecord>,
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

impl GetPaidResponseData {
    pub fn builder() -> GetPaidResponseDataBuilder {
        <GetPaidResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPaidResponseDataBuilder {
    auth_code: Option<Authcode>,
    transaction_details: Option<TransactionDetailRecord>,
    reference_id: Option<Referenceidtrans>,
    result_code: Option<ResultCode>,
    result_text: Option<Resulttext>,
    avs_response_text: Option<AvsResponseText>,
    cvv_response_text: Option<CvvResponseText>,
    customer_id: Option<Customeridtrans>,
    method_reference_id: Option<MethodReferenceId>,
}

impl GetPaidResponseDataBuilder {
    pub fn auth_code(mut self, value: Authcode) -> Self {
        self.auth_code = Some(value);
        self
    }

    pub fn transaction_details(mut self, value: TransactionDetailRecord) -> Self {
        self.transaction_details = Some(value);
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

    /// Consumes the builder and constructs a [`GetPaidResponseData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`reference_id`](GetPaidResponseDataBuilder::reference_id)
    /// - [`result_code`](GetPaidResponseDataBuilder::result_code)
    /// - [`result_text`](GetPaidResponseDataBuilder::result_text)
    /// - [`avs_response_text`](GetPaidResponseDataBuilder::avs_response_text)
    /// - [`cvv_response_text`](GetPaidResponseDataBuilder::cvv_response_text)
    /// - [`customer_id`](GetPaidResponseDataBuilder::customer_id)
    pub fn build(self) -> Result<GetPaidResponseData, BuildError> {
        Ok(GetPaidResponseData {
            auth_code: self.auth_code,
            transaction_details: self.transaction_details,
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
