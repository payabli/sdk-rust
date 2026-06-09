pub use crate::prelude::*;

/// Query parameters for deleteAttachedFromBill
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeleteAttachedFromBillQueryRequest {
    /// When `true`, the response includes the full bill object.
    #[serde(rename = "returnObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_object: Option<bool>,
}

impl DeleteAttachedFromBillQueryRequest {
    pub fn builder() -> DeleteAttachedFromBillQueryRequestBuilder {
        <DeleteAttachedFromBillQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeleteAttachedFromBillQueryRequestBuilder {
    return_object: Option<bool>,
}

impl DeleteAttachedFromBillQueryRequestBuilder {
    pub fn return_object(mut self, value: bool) -> Self {
        self.return_object = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeleteAttachedFromBillQueryRequest`].
    pub fn build(self) -> Result<DeleteAttachedFromBillQueryRequest, BuildError> {
        Ok(DeleteAttachedFromBillQueryRequest {
            return_object: self.return_object,
        })
    }
}
