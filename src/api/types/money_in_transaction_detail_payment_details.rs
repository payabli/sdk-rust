pub use crate::prelude::*;

/// Detailed breakdown of payment amounts and identifiers
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TransactionDetailPaymentDetails {
    #[serde(rename = "totalAmount")]
    #[serde(default)]
    pub total_amount: f64,
    #[serde(rename = "serviceFee")]
    #[serde(default)]
    pub service_fee: f64,
    #[serde(rename = "checkNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_number: Option<String>,
    #[serde(rename = "checkImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_image: Option<serde_json::Value>,
    #[serde(rename = "checkUniqueId")]
    #[serde(default)]
    pub check_unique_id: String,
    #[serde(default)]
    pub currency: String,
    #[serde(rename = "orderDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_description: Option<Orderdescription>,
    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderId>,
    #[serde(rename = "orderIdAlternative")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id_alternative: Option<String>,
    #[serde(rename = "paymentDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_description: Option<String>,
    #[serde(rename = "groupNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(rename = "payabliTransId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payabli_trans_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unbundled: Option<serde_json::Value>,
    #[serde(default)]
    pub categories: Vec<serde_json::Value>,
    #[serde(rename = "splitFunding")]
    #[serde(default)]
    pub split_funding: Vec<serde_json::Value>,
}

impl TransactionDetailPaymentDetails {
    pub fn builder() -> TransactionDetailPaymentDetailsBuilder {
        <TransactionDetailPaymentDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionDetailPaymentDetailsBuilder {
    total_amount: Option<f64>,
    service_fee: Option<f64>,
    check_number: Option<String>,
    check_image: Option<serde_json::Value>,
    check_unique_id: Option<String>,
    currency: Option<String>,
    order_description: Option<Orderdescription>,
    order_id: Option<OrderId>,
    order_id_alternative: Option<String>,
    payment_description: Option<String>,
    group_number: Option<String>,
    source: Option<Source>,
    payabli_trans_id: Option<String>,
    unbundled: Option<serde_json::Value>,
    categories: Option<Vec<serde_json::Value>>,
    split_funding: Option<Vec<serde_json::Value>>,
}

impl TransactionDetailPaymentDetailsBuilder {
    pub fn total_amount(mut self, value: f64) -> Self {
        self.total_amount = Some(value);
        self
    }

    pub fn service_fee(mut self, value: f64) -> Self {
        self.service_fee = Some(value);
        self
    }

    pub fn check_number(mut self, value: impl Into<String>) -> Self {
        self.check_number = Some(value.into());
        self
    }

    pub fn check_image(mut self, value: serde_json::Value) -> Self {
        self.check_image = Some(value);
        self
    }

    pub fn check_unique_id(mut self, value: impl Into<String>) -> Self {
        self.check_unique_id = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
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

    pub fn order_id_alternative(mut self, value: impl Into<String>) -> Self {
        self.order_id_alternative = Some(value.into());
        self
    }

    pub fn payment_description(mut self, value: impl Into<String>) -> Self {
        self.payment_description = Some(value.into());
        self
    }

    pub fn group_number(mut self, value: impl Into<String>) -> Self {
        self.group_number = Some(value.into());
        self
    }

    pub fn source(mut self, value: Source) -> Self {
        self.source = Some(value);
        self
    }

    pub fn payabli_trans_id(mut self, value: impl Into<String>) -> Self {
        self.payabli_trans_id = Some(value.into());
        self
    }

    pub fn unbundled(mut self, value: serde_json::Value) -> Self {
        self.unbundled = Some(value);
        self
    }

    pub fn categories(mut self, value: Vec<serde_json::Value>) -> Self {
        self.categories = Some(value);
        self
    }

    pub fn split_funding(mut self, value: Vec<serde_json::Value>) -> Self {
        self.split_funding = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransactionDetailPaymentDetails`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total_amount`](TransactionDetailPaymentDetailsBuilder::total_amount)
    /// - [`service_fee`](TransactionDetailPaymentDetailsBuilder::service_fee)
    /// - [`check_unique_id`](TransactionDetailPaymentDetailsBuilder::check_unique_id)
    /// - [`currency`](TransactionDetailPaymentDetailsBuilder::currency)
    /// - [`categories`](TransactionDetailPaymentDetailsBuilder::categories)
    /// - [`split_funding`](TransactionDetailPaymentDetailsBuilder::split_funding)
    pub fn build(self) -> Result<TransactionDetailPaymentDetails, BuildError> {
        Ok(TransactionDetailPaymentDetails {
            total_amount: self
                .total_amount
                .ok_or_else(|| BuildError::missing_field("total_amount"))?,
            service_fee: self
                .service_fee
                .ok_or_else(|| BuildError::missing_field("service_fee"))?,
            check_number: self.check_number,
            check_image: self.check_image,
            check_unique_id: self
                .check_unique_id
                .ok_or_else(|| BuildError::missing_field("check_unique_id"))?,
            currency: self
                .currency
                .ok_or_else(|| BuildError::missing_field("currency"))?,
            order_description: self.order_description,
            order_id: self.order_id,
            order_id_alternative: self.order_id_alternative,
            payment_description: self.payment_description,
            group_number: self.group_number,
            source: self.source,
            payabli_trans_id: self.payabli_trans_id,
            unbundled: self.unbundled,
            categories: self
                .categories
                .ok_or_else(|| BuildError::missing_field("categories"))?,
            split_funding: self
                .split_funding
                .ok_or_else(|| BuildError::missing_field("split_funding"))?,
        })
    }
}
