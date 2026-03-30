pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PayabliApiResponseCustomerQuery {
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<CustomerQueryRecords>,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
}

impl PayabliApiResponseCustomerQuery {
    pub fn builder() -> PayabliApiResponseCustomerQueryBuilder {
        <PayabliApiResponseCustomerQueryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayabliApiResponseCustomerQueryBuilder {
    is_success: Option<IsSuccess>,
    response_data: Option<CustomerQueryRecords>,
    response_text: Option<ResponseText>,
}

impl PayabliApiResponseCustomerQueryBuilder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_data(mut self, value: CustomerQueryRecords) -> Self {
        self.response_data = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayabliApiResponseCustomerQuery`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](PayabliApiResponseCustomerQueryBuilder::response_text)
    pub fn build(self) -> Result<PayabliApiResponseCustomerQuery, BuildError> {
        Ok(PayabliApiResponseCustomerQuery {
            is_success: self.is_success,
            response_data: self.response_data,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
        })
    }
}
