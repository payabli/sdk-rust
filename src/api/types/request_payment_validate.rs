pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RequestPaymentValidate {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountId>,
    #[serde(rename = "entryPoint")]
    #[serde(default)]
    pub entry_point: Entrypointfield,
    #[serde(rename = "orderDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_description: Option<Orderdescription>,
    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderId>,
    /// Object describing payment method to use for transaction.
    #[serde(rename = "paymentMethod")]
    pub payment_method: RequestPaymentValidatePaymentMethod,
}

impl RequestPaymentValidate {
    pub fn builder() -> RequestPaymentValidateBuilder {
        <RequestPaymentValidateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequestPaymentValidateBuilder {
    account_id: Option<AccountId>,
    entry_point: Option<Entrypointfield>,
    order_description: Option<Orderdescription>,
    order_id: Option<OrderId>,
    payment_method: Option<RequestPaymentValidatePaymentMethod>,
}

impl RequestPaymentValidateBuilder {
    pub fn account_id(mut self, value: AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn entry_point(mut self, value: Entrypointfield) -> Self {
        self.entry_point = Some(value);
        self
    }

    pub fn order_description(mut self, value: Orderdescription) -> Self {
        self.order_description = Some(value);
        self
    }

    pub fn order_id(mut self, value: OrderId) -> Self {
        self.order_id = Some(value);
        self
    }

    pub fn payment_method(mut self, value: RequestPaymentValidatePaymentMethod) -> Self {
        self.payment_method = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RequestPaymentValidate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entry_point`](RequestPaymentValidateBuilder::entry_point)
    /// - [`payment_method`](RequestPaymentValidateBuilder::payment_method)
    pub fn build(self) -> Result<RequestPaymentValidate, BuildError> {
        Ok(RequestPaymentValidate {
            account_id: self.account_id,
            entry_point: self
                .entry_point
                .ok_or_else(|| BuildError::missing_field("entry_point"))?,
            order_description: self.order_description,
            order_id: self.order_id,
            payment_method: self
                .payment_method
                .ok_or_else(|| BuildError::missing_field("payment_method"))?,
        })
    }
}
