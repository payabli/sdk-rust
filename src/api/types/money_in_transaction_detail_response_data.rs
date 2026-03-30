pub use crate::prelude::*;

/// Response data from payment processor
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransactionDetailResponseData {
    /// Unified result code for the transaction. See [Pay In unified response codes](/guides/pay-in-unified-response-codes-reference) for more information.
    #[serde(rename = "resultCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_code: Option<String>,
    /// Description of the result code. See [Pay In unified response codes](/guides/pay-in-unified-response-codes-reference) for more information.
    #[serde(rename = "resultCodeText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_code_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
    #[serde(default)]
    pub responsetext: Resulttext,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authcode: Option<Authcode>,
    #[serde(default)]
    pub transactionid: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avsresponse: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avsresponse_text: Option<AvsResponseText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvvresponse: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvvresponse_text: Option<CvvResponseText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderid: Option<OrderId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(default)]
    pub response_code: String,
    #[serde(default)]
    pub response_code_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_vault_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emv_auth_response_data: Option<EmvAuthResponseData>,
}

impl TransactionDetailResponseData {
    pub fn builder() -> TransactionDetailResponseDataBuilder {
        <TransactionDetailResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionDetailResponseDataBuilder {
    result_code: Option<String>,
    result_code_text: Option<String>,
    response: Option<String>,
    responsetext: Option<Resulttext>,
    authcode: Option<Authcode>,
    transactionid: Option<String>,
    avsresponse: Option<String>,
    avsresponse_text: Option<AvsResponseText>,
    cvvresponse: Option<String>,
    cvvresponse_text: Option<CvvResponseText>,
    orderid: Option<OrderId>,
    r#type: Option<String>,
    response_code: Option<String>,
    response_code_text: Option<String>,
    customer_vault_id: Option<String>,
    emv_auth_response_data: Option<EmvAuthResponseData>,
}

impl TransactionDetailResponseDataBuilder {
    pub fn result_code(mut self, value: impl Into<String>) -> Self {
        self.result_code = Some(value.into());
        self
    }

    pub fn result_code_text(mut self, value: impl Into<String>) -> Self {
        self.result_code_text = Some(value.into());
        self
    }

    pub fn response(mut self, value: impl Into<String>) -> Self {
        self.response = Some(value.into());
        self
    }

    pub fn responsetext(mut self, value: Resulttext) -> Self {
        self.responsetext = Some(value);
        self
    }

    pub fn authcode(mut self, value: Authcode) -> Self {
        self.authcode = Some(value);
        self
    }

    pub fn transactionid(mut self, value: impl Into<String>) -> Self {
        self.transactionid = Some(value.into());
        self
    }

    pub fn avsresponse(mut self, value: impl Into<String>) -> Self {
        self.avsresponse = Some(value.into());
        self
    }

    pub fn avsresponse_text(mut self, value: AvsResponseText) -> Self {
        self.avsresponse_text = Some(value);
        self
    }

    pub fn cvvresponse(mut self, value: impl Into<String>) -> Self {
        self.cvvresponse = Some(value.into());
        self
    }

    pub fn cvvresponse_text(mut self, value: CvvResponseText) -> Self {
        self.cvvresponse_text = Some(value);
        self
    }

    pub fn orderid(mut self, value: OrderId) -> Self {
        self.orderid = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn response_code(mut self, value: impl Into<String>) -> Self {
        self.response_code = Some(value.into());
        self
    }

    pub fn response_code_text(mut self, value: impl Into<String>) -> Self {
        self.response_code_text = Some(value.into());
        self
    }

    pub fn customer_vault_id(mut self, value: impl Into<String>) -> Self {
        self.customer_vault_id = Some(value.into());
        self
    }

    pub fn emv_auth_response_data(mut self, value: EmvAuthResponseData) -> Self {
        self.emv_auth_response_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransactionDetailResponseData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`responsetext`](TransactionDetailResponseDataBuilder::responsetext)
    /// - [`transactionid`](TransactionDetailResponseDataBuilder::transactionid)
    /// - [`response_code`](TransactionDetailResponseDataBuilder::response_code)
    /// - [`response_code_text`](TransactionDetailResponseDataBuilder::response_code_text)
    pub fn build(self) -> Result<TransactionDetailResponseData, BuildError> {
        Ok(TransactionDetailResponseData {
            result_code: self.result_code,
            result_code_text: self.result_code_text,
            response: self.response,
            responsetext: self
                .responsetext
                .ok_or_else(|| BuildError::missing_field("responsetext"))?,
            authcode: self.authcode,
            transactionid: self
                .transactionid
                .ok_or_else(|| BuildError::missing_field("transactionid"))?,
            avsresponse: self.avsresponse,
            avsresponse_text: self.avsresponse_text,
            cvvresponse: self.cvvresponse,
            cvvresponse_text: self.cvvresponse_text,
            orderid: self.orderid,
            r#type: self.r#type,
            response_code: self
                .response_code
                .ok_or_else(|| BuildError::missing_field("response_code"))?,
            response_code_text: self
                .response_code_text
                .ok_or_else(|| BuildError::missing_field("response_code_text"))?,
            customer_vault_id: self.customer_vault_id,
            emv_auth_response_data: self.emv_auth_response_data,
        })
    }
}
