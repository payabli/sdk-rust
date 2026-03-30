pub use crate::prelude::*;

/// Query parameters for GetAttachedFileFromInvoice
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetAttachedFileFromInvoiceQueryRequest {
    /// When `true`, the request returns the file content as a Base64-encoded string.
    #[serde(rename = "returnObject")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_object: Option<bool>,
}

impl GetAttachedFileFromInvoiceQueryRequest {
    pub fn builder() -> GetAttachedFileFromInvoiceQueryRequestBuilder {
        <GetAttachedFileFromInvoiceQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetAttachedFileFromInvoiceQueryRequestBuilder {
    return_object: Option<bool>,
}

impl GetAttachedFileFromInvoiceQueryRequestBuilder {
    pub fn return_object(mut self, value: bool) -> Self {
        self.return_object = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetAttachedFileFromInvoiceQueryRequest`].
    pub fn build(self) -> Result<GetAttachedFileFromInvoiceQueryRequest, BuildError> {
        Ok(GetAttachedFileFromInvoiceQueryRequest {
            return_object: self.return_object,
        })
    }
}
