pub use crate::prelude::*;

/// Response data for canceling a single payout transaction. Mirrors the general response data, with `VendorId` added alongside `CustomerId`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CancelPayoutResponseData {
    #[serde(rename = "AuthCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<Authcode>,
    #[serde(rename = "avsResponseText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avs_response_text: Option<AvsResponseText>,
    /// Payabli-generated unique ID of the vendor on the payout. Returns the same value as `VendorId`, or `0` when no vendor is associated.
    #[serde(rename = "CustomerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<Vendoridtrans>,
    /// Payabli-generated unique ID of the vendor on the payout. Returns the same value as `CustomerId`, or `0` when no vendor is associated.
    #[serde(rename = "VendorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<Vendoridtrans>,
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

impl CancelPayoutResponseData {
    pub fn builder() -> CancelPayoutResponseDataBuilder {
        <CancelPayoutResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CancelPayoutResponseDataBuilder {
    auth_code: Option<Authcode>,
    avs_response_text: Option<AvsResponseText>,
    customer_id: Option<Vendoridtrans>,
    vendor_id: Option<Vendoridtrans>,
    cvv_response_text: Option<CvvResponseText>,
    method_reference_id: Option<MethodReferenceId>,
    reference_id: Option<Referenceidtrans>,
    result_code: Option<ResultCode>,
    result_text: Option<Resulttext>,
}

impl CancelPayoutResponseDataBuilder {
    pub fn auth_code(mut self, value: Authcode) -> Self {
        self.auth_code = Some(value);
        self
    }

    pub fn avs_response_text(mut self, value: AvsResponseText) -> Self {
        self.avs_response_text = Some(value);
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

    /// Consumes the builder and constructs a [`CancelPayoutResponseData`].
    pub fn build(self) -> Result<CancelPayoutResponseData, BuildError> {
        Ok(CancelPayoutResponseData {
            auth_code: self.auth_code,
            avs_response_text: self.avs_response_text,
            customer_id: self.customer_id,
            vendor_id: self.vendor_id,
            cvv_response_text: self.cvv_response_text,
            method_reference_id: self.method_reference_id,
            reference_id: self.reference_id,
            result_code: self.result_code,
            result_text: self.result_text,
        })
    }
}
