pub use crate::prelude::*;

/// EMV data exchanged during the authorization of a device transaction. `null`
/// when both values are `null`, such as for swiped cards, device transactions
/// that never reached the processor, and non-device transactions.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EmvAuthData {
    /// EMV tag-length-value (TLV) data the device sent with the authorization request.
    #[serde(rename = "RequestEmvData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_emv_data: Option<String>,
    /// EMV data returned in the authorization response. Same value as `ResponseData.emv_auth_response_data`.
    #[serde(rename = "ResponseEmvData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_emv_data: Option<String>,
}

impl EmvAuthData {
    pub fn builder() -> EmvAuthDataBuilder {
        <EmvAuthDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EmvAuthDataBuilder {
    request_emv_data: Option<String>,
    response_emv_data: Option<String>,
}

impl EmvAuthDataBuilder {
    pub fn request_emv_data(mut self, value: impl Into<String>) -> Self {
        self.request_emv_data = Some(value.into());
        self
    }

    pub fn response_emv_data(mut self, value: impl Into<String>) -> Self {
        self.response_emv_data = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`EmvAuthData`].
    pub fn build(self) -> Result<EmvAuthData, BuildError> {
        Ok(EmvAuthData {
            request_emv_data: self.request_emv_data,
            response_emv_data: self.response_emv_data,
        })
    }
}
