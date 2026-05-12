pub use crate::prelude::*;

/// Payment details for payout subscriptions.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PayoutPaymentDetail {
    /// Total payout amount. If a service fee is included, this amount should include the service fee.
    #[serde(rename = "totalAmount")]
    #[serde(default)]
    pub total_amount: f64,
    /// Service fee to be deducted from the total amount. This amount must be a number, percentages aren't accepted.
    #[serde(rename = "serviceFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_fee: Option<f64>,
    /// Currency code ISO-4217. If no code is provided, the currency in the paypoint setting is used. Default is `USD`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// A check number for the payout. Required when the payment method is `check`.
    #[serde(rename = "checkNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_number: Option<String>,
    /// Description of the payout order.
    #[serde(rename = "orderDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_description: Option<String>,
    /// Order identifier associated with the payout.
    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Alternative order identifier.
    #[serde(rename = "orderIdAlternative")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id_alternative: Option<String>,
    /// Description of the payment.
    #[serde(rename = "paymentDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_description: Option<String>,
    /// Settlement descriptor for the payout.
    #[serde(rename = "settlementDescriptor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_descriptor: Option<String>,
    /// Group number for the payout.
    #[serde(rename = "groupNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_number: Option<String>,
    /// Source identifier for the payout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Payabli transaction identifier.
    #[serde(rename = "payabliTransId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payabli_trans_id: Option<String>,
    /// When `true`, each bill is processed as a separate payout. When `false` or not provided, multiple bills are paid with a single payout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unbundled: Option<bool>,
}

impl PayoutPaymentDetail {
    pub fn builder() -> PayoutPaymentDetailBuilder {
        <PayoutPaymentDetailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayoutPaymentDetailBuilder {
    total_amount: Option<f64>,
    service_fee: Option<f64>,
    currency: Option<String>,
    check_number: Option<String>,
    order_description: Option<String>,
    order_id: Option<String>,
    order_id_alternative: Option<String>,
    payment_description: Option<String>,
    settlement_descriptor: Option<String>,
    group_number: Option<String>,
    source: Option<String>,
    payabli_trans_id: Option<String>,
    unbundled: Option<bool>,
}

impl PayoutPaymentDetailBuilder {
    pub fn total_amount(mut self, value: f64) -> Self {
        self.total_amount = Some(value);
        self
    }

    pub fn service_fee(mut self, value: f64) -> Self {
        self.service_fee = Some(value);
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn check_number(mut self, value: impl Into<String>) -> Self {
        self.check_number = Some(value.into());
        self
    }

    pub fn order_description(mut self, value: impl Into<String>) -> Self {
        self.order_description = Some(value.into());
        self
    }

    pub fn order_id(mut self, value: impl Into<String>) -> Self {
        self.order_id = Some(value.into());
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

    pub fn settlement_descriptor(mut self, value: impl Into<String>) -> Self {
        self.settlement_descriptor = Some(value.into());
        self
    }

    pub fn group_number(mut self, value: impl Into<String>) -> Self {
        self.group_number = Some(value.into());
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn payabli_trans_id(mut self, value: impl Into<String>) -> Self {
        self.payabli_trans_id = Some(value.into());
        self
    }

    pub fn unbundled(mut self, value: bool) -> Self {
        self.unbundled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayoutPaymentDetail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total_amount`](PayoutPaymentDetailBuilder::total_amount)
    pub fn build(self) -> Result<PayoutPaymentDetail, BuildError> {
        Ok(PayoutPaymentDetail {
            total_amount: self
                .total_amount
                .ok_or_else(|| BuildError::missing_field("total_amount"))?,
            service_fee: self.service_fee,
            currency: self.currency,
            check_number: self.check_number,
            order_description: self.order_description,
            order_id: self.order_id,
            order_id_alternative: self.order_id_alternative,
            payment_description: self.payment_description,
            settlement_descriptor: self.settlement_descriptor,
            group_number: self.group_number,
            source: self.source,
            payabli_trans_id: self.payabli_trans_id,
            unbundled: self.unbundled,
        })
    }
}
