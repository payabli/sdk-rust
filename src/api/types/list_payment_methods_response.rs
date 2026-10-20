pub use crate::prelude::*;

/// A page of payment methods.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPaymentMethodsResponse {
    /// The payment methods on this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<PaymentMethodDetail>>,
    /// Pagination metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}

impl ListPaymentMethodsResponse {
    pub fn builder() -> ListPaymentMethodsResponseBuilder {
        <ListPaymentMethodsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPaymentMethodsResponseBuilder {
    data: Option<Vec<PaymentMethodDetail>>,
    pagination: Option<Pagination>,
}

impl ListPaymentMethodsResponseBuilder {
    pub fn data(mut self, value: Vec<PaymentMethodDetail>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn pagination(mut self, value: Pagination) -> Self {
        self.pagination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPaymentMethodsResponse`].
    pub fn build(self) -> Result<ListPaymentMethodsResponse, BuildError> {
        Ok(ListPaymentMethodsResponse {
            data: self.data,
            pagination: self.pagination,
        })
    }
}
