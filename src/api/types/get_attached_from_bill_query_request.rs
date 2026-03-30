pub use crate::prelude::*;

/// Query parameters for getAttachedFromBill
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAttachedFromBillQueryRequest {
    /// When `true`, the request returns the file content as a Base64-encoded string.
    #[serde(rename = "returnObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_object: Option<bool>,
}

impl GetAttachedFromBillQueryRequest {
    pub fn builder() -> GetAttachedFromBillQueryRequestBuilder {
        <GetAttachedFromBillQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAttachedFromBillQueryRequestBuilder {
    return_object: Option<bool>,
}

impl GetAttachedFromBillQueryRequestBuilder {
    pub fn return_object(mut self, value: bool) -> Self {
        self.return_object = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAttachedFromBillQueryRequest`].
    pub fn build(self) -> Result<GetAttachedFromBillQueryRequest, BuildError> {
        Ok(GetAttachedFromBillQueryRequest {
            return_object: self.return_object,
        })
    }
}
