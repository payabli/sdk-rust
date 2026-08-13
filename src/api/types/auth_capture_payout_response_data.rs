pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AuthCapturePayoutResponseData {
    #[serde(rename = "authCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<Authcode>,
    /// The transaction reference ID, used to capture the transaction. Returns `null` when no transaction is created, such as a declined authorization.
    #[serde(rename = "referenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<Referenceidtrans>,
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
    /// Payabli-generated unique ID of the vendor on the payout. Returns the same value as `vendorId`, or `0` when no vendor is associated.
    #[serde(rename = "customerId")]
    #[serde(default)]
    pub customer_id: Vendoridtrans,
    /// Payabli-generated unique ID of the vendor on the payout. Returns the same value as `customerId`, or `0` when no vendor is associated.
    #[serde(rename = "vendorId")]
    #[serde(default)]
    pub vendor_id: Vendoridtrans,
    #[serde(rename = "methodReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_reference_id: Option<MethodReferenceId>,
}

impl AuthCapturePayoutResponseData {
    pub fn builder() -> AuthCapturePayoutResponseDataBuilder {
        <AuthCapturePayoutResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthCapturePayoutResponseDataBuilder {
    auth_code: Option<Authcode>,
    reference_id: Option<Referenceidtrans>,
    result_code: Option<ResultCode>,
    result_text: Option<Resulttext>,
    avs_response_text: Option<AvsResponseText>,
    cvv_response_text: Option<CvvResponseText>,
    customer_id: Option<Vendoridtrans>,
    vendor_id: Option<Vendoridtrans>,
    method_reference_id: Option<MethodReferenceId>,
}

impl AuthCapturePayoutResponseDataBuilder {
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

    pub fn customer_id(mut self, value: Vendoridtrans) -> Self {
        self.customer_id = Some(value);
        self
    }

    pub fn vendor_id(mut self, value: Vendoridtrans) -> Self {
        self.vendor_id = Some(value);
        self
    }

    pub fn method_reference_id(mut self, value: MethodReferenceId) -> Self {
        self.method_reference_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AuthCapturePayoutResponseData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`result_code`](AuthCapturePayoutResponseDataBuilder::result_code)
    /// - [`result_text`](AuthCapturePayoutResponseDataBuilder::result_text)
    /// - [`customer_id`](AuthCapturePayoutResponseDataBuilder::customer_id)
    /// - [`vendor_id`](AuthCapturePayoutResponseDataBuilder::vendor_id)
    pub fn build(self) -> Result<AuthCapturePayoutResponseData, BuildError> {
        Ok(AuthCapturePayoutResponseData {
            auth_code: self.auth_code,
            reference_id: self.reference_id,
            result_code: self
                .result_code
                .ok_or_else(|| BuildError::missing_field("result_code"))?,
            result_text: self
                .result_text
                .ok_or_else(|| BuildError::missing_field("result_text"))?,
            avs_response_text: self.avs_response_text,
            cvv_response_text: self.cvv_response_text,
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            vendor_id: self
                .vendor_id
                .ok_or_else(|| BuildError::missing_field("vendor_id"))?,
            method_reference_id: self.method_reference_id,
        })
    }
}
