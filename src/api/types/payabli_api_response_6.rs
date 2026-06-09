pub use crate::prelude::*;

/// Response schema for line item operations.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayabliApiResponse6 {
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
    /// If `isSuccess` = true, this contains the line item identifier. If `isSuccess` = false, this contains the reason for the error.
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<Responsedatanonobject>,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
}

impl PayabliApiResponse6 {
    pub fn builder() -> PayabliApiResponse6Builder {
        <PayabliApiResponse6Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayabliApiResponse6Builder {
    is_success: Option<IsSuccess>,
    page_identifier: Option<PageIdentifier>,
    response_data: Option<Responsedatanonobject>,
    response_text: Option<ResponseText>,
}

impl PayabliApiResponse6Builder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn page_identifier(mut self, value: PageIdentifier) -> Self {
        self.page_identifier = Some(value);
        self
    }

    pub fn response_data(mut self, value: Responsedatanonobject) -> Self {
        self.response_data = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayabliApiResponse6`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](PayabliApiResponse6Builder::response_text)
    pub fn build(self) -> Result<PayabliApiResponse6, BuildError> {
        Ok(PayabliApiResponse6 {
            is_success: self.is_success,
            page_identifier: self.page_identifier,
            response_data: self.response_data,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
        })
    }
}
