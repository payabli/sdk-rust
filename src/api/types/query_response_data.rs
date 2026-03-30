pub use crate::prelude::*;

/// The transaction's response data.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryResponseData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authcode: Option<Authcode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avsresponse: Option<AvsResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avsresponse_text: Option<AvsResponseText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvvresponse: Option<CvvResponse>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvvresponse_text: Option<CvvResponseText>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emv_auth_response_data: Option<EmvAuthResponseData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub orderid: Option<OrderId>,
    /// Response text for operation: 'Success' or 'Declined'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response: Option<String>,
    /// Internal result code processing the transaction. Value 1 indicates successful operation, values 2 and 3 indicate errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<String>,
    /// Text describing the result. If resultCode = 1, will return 'Approved' or a general success message. If resultCode = 2 or 3, will contain the cause of the decline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code_text: Option<String>,
    /// Text describing the result. If resultCode = 1, will return 'Approved' or a general success message. If resultCode = 2 or 3, will contain the cause of the decline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responsetext: Option<String>,
    #[serde(rename = "resultCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_code: Option<ResultCodev2>,
    #[serde(rename = "resultCodeText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_code_text: Option<ResultCodeText>,
    /// The transaction identifier in Payabli.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transactionid: Option<String>,
    /// Type of transaction or operation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl QueryResponseData {
    pub fn builder() -> QueryResponseDataBuilder {
        <QueryResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryResponseDataBuilder {
    authcode: Option<Authcode>,
    avsresponse: Option<AvsResponse>,
    avsresponse_text: Option<AvsResponseText>,
    cvvresponse: Option<CvvResponse>,
    cvvresponse_text: Option<CvvResponseText>,
    emv_auth_response_data: Option<EmvAuthResponseData>,
    orderid: Option<OrderId>,
    response: Option<String>,
    response_code: Option<String>,
    response_code_text: Option<String>,
    responsetext: Option<String>,
    result_code: Option<ResultCodev2>,
    result_code_text: Option<ResultCodeText>,
    transactionid: Option<String>,
    r#type: Option<String>,
}

impl QueryResponseDataBuilder {
    pub fn authcode(mut self, value: Authcode) -> Self {
        self.authcode = Some(value);
        self
    }

    pub fn avsresponse(mut self, value: AvsResponse) -> Self {
        self.avsresponse = Some(value);
        self
    }

    pub fn avsresponse_text(mut self, value: AvsResponseText) -> Self {
        self.avsresponse_text = Some(value);
        self
    }

    pub fn cvvresponse(mut self, value: CvvResponse) -> Self {
        self.cvvresponse = Some(value);
        self
    }

    pub fn cvvresponse_text(mut self, value: CvvResponseText) -> Self {
        self.cvvresponse_text = Some(value);
        self
    }

    pub fn emv_auth_response_data(mut self, value: EmvAuthResponseData) -> Self {
        self.emv_auth_response_data = Some(value);
        self
    }

    pub fn orderid(mut self, value: OrderId) -> Self {
        self.orderid = Some(value);
        self
    }

    pub fn response(mut self, value: impl Into<String>) -> Self {
        self.response = Some(value.into());
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

    pub fn responsetext(mut self, value: impl Into<String>) -> Self {
        self.responsetext = Some(value.into());
        self
    }

    pub fn result_code(mut self, value: ResultCodev2) -> Self {
        self.result_code = Some(value);
        self
    }

    pub fn result_code_text(mut self, value: ResultCodeText) -> Self {
        self.result_code_text = Some(value);
        self
    }

    pub fn transactionid(mut self, value: impl Into<String>) -> Self {
        self.transactionid = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryResponseData`].
    pub fn build(self) -> Result<QueryResponseData, BuildError> {
        Ok(QueryResponseData {
            authcode: self.authcode,
            avsresponse: self.avsresponse,
            avsresponse_text: self.avsresponse_text,
            cvvresponse: self.cvvresponse,
            cvvresponse_text: self.cvvresponse_text,
            emv_auth_response_data: self.emv_auth_response_data,
            orderid: self.orderid,
            response: self.response,
            response_code: self.response_code,
            response_code_text: self.response_code_text,
            responsetext: self.responsetext,
            result_code: self.result_code,
            result_code_text: self.result_code_text,
            transactionid: self.transactionid,
            r#type: self.r#type,
        })
    }
}
