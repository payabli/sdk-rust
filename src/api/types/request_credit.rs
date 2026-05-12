pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestCredit {
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountId>,
    /// Object describing the customer/payor.
    #[serde(rename = "customerData")]
    #[serde(default)]
    pub customer_data: PayorDataRequest,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypoint: Option<Entrypointfield>,
    #[serde(rename = "orderDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_description: Option<Orderdescription>,
    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderId>,
    #[serde(rename = "paymentDetails")]
    #[serde(default)]
    pub payment_details: PaymentDetailCredit,
    /// Object describing the ACH payment method to use for transaction.
    #[serde(rename = "paymentMethod")]
    pub payment_method: RequestCreditPaymentMethod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<Subdomain>,
    #[serde(rename = "forceCustomerCreation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_customer_creation: Option<ForceCustomerCreation>,
}

impl RequestCredit {
    pub fn builder() -> RequestCreditBuilder {
        <RequestCreditBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequestCreditBuilder {
    account_id: Option<AccountId>,
    customer_data: Option<PayorDataRequest>,
    entrypoint: Option<Entrypointfield>,
    order_description: Option<Orderdescription>,
    order_id: Option<OrderId>,
    payment_details: Option<PaymentDetailCredit>,
    payment_method: Option<RequestCreditPaymentMethod>,
    source: Option<Source>,
    subdomain: Option<Subdomain>,
    force_customer_creation: Option<ForceCustomerCreation>,
}

impl RequestCreditBuilder {
    pub fn account_id(mut self, value: AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn customer_data(mut self, value: PayorDataRequest) -> Self {
        self.customer_data = Some(value);
        self
    }

    pub fn entrypoint(mut self, value: Entrypointfield) -> Self {
        self.entrypoint = Some(value);
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

    pub fn payment_details(mut self, value: PaymentDetailCredit) -> Self {
        self.payment_details = Some(value);
        self
    }

    pub fn payment_method(mut self, value: RequestCreditPaymentMethod) -> Self {
        self.payment_method = Some(value);
        self
    }

    pub fn source(mut self, value: Source) -> Self {
        self.source = Some(value);
        self
    }

    pub fn subdomain(mut self, value: Subdomain) -> Self {
        self.subdomain = Some(value);
        self
    }

    pub fn force_customer_creation(mut self, value: ForceCustomerCreation) -> Self {
        self.force_customer_creation = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RequestCredit`].
    /// This method will fail if any of the following fields are not set:
    /// - [`customer_data`](RequestCreditBuilder::customer_data)
    /// - [`payment_details`](RequestCreditBuilder::payment_details)
    /// - [`payment_method`](RequestCreditBuilder::payment_method)
    pub fn build(self) -> Result<RequestCredit, BuildError> {
        Ok(RequestCredit {
            account_id: self.account_id,
            customer_data: self
                .customer_data
                .ok_or_else(|| BuildError::missing_field("customer_data"))?,
            entrypoint: self.entrypoint,
            order_description: self.order_description,
            order_id: self.order_id,
            payment_details: self
                .payment_details
                .ok_or_else(|| BuildError::missing_field("payment_details"))?,
            payment_method: self
                .payment_method
                .ok_or_else(|| BuildError::missing_field("payment_method"))?,
            source: self.source,
            subdomain: self.subdomain,
            force_customer_creation: self.force_customer_creation,
        })
    }
}
