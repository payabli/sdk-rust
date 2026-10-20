pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreatePaymentMethodsRequest {
    /// The payment methods to create.
    #[serde(default)]
    pub data: Vec<CreatePaymentMethodRequest>,
}

impl CreatePaymentMethodsRequest {
    pub fn builder() -> CreatePaymentMethodsRequestBuilder {
        <CreatePaymentMethodsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaymentMethodsRequestBuilder {
    data: Option<Vec<CreatePaymentMethodRequest>>,
}

impl CreatePaymentMethodsRequestBuilder {
    pub fn data(mut self, value: Vec<CreatePaymentMethodRequest>) -> Self {
        self.data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePaymentMethodsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`data`](CreatePaymentMethodsRequestBuilder::data)
    pub fn build(self) -> Result<CreatePaymentMethodsRequest, BuildError> {
        Ok(CreatePaymentMethodsRequest {
            data: self.data.ok_or_else(|| BuildError::missing_field("data"))?,
        })
    }
}
