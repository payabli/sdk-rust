pub use crate::prelude::*;

/// Payment details for the payout subscription.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PayoutSubscriptionPaymentDetails {
    /// Total payout amount, including any service fee.
    #[serde(rename = "totalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_amount: Option<f64>,
    /// Service fee deducted from the total amount.
    #[serde(rename = "serviceFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_fee: Option<f64>,
    /// Surcharge amount. Always `0` for payout subscriptions.
    #[serde(rename = "surchargeFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surcharge_fee: Option<f64>,
    /// Check number for the payout. Set when the payment method is `check`.
    #[serde(rename = "checkNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_number: Option<String>,
    /// Object containing an image of a paper check.
    #[serde(rename = "checkImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_image: Option<HashMap<String, serde_json::Value>>,
    /// Unique identifier for a processed check image.
    #[serde(rename = "checkUniqueId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_unique_id: Option<String>,
    /// Currency code ISO-4217.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
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
    /// Settlement descriptor for the payout. Always `null` on this endpoint.
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
    /// Payabli transaction identifier. Always `null` on this endpoint.
    #[serde(rename = "payabliTransId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payabli_trans_id: Option<String>,
    /// When `true`, each bill is processed as a separate payout. Always `null` on this endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unbundled: Option<bool>,
    /// Payment categories or line items describing the amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<PaymentCategories>>,
    #[serde(rename = "splitFunding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_funding: Option<SplitFunding>,
}

impl PayoutSubscriptionPaymentDetails {
    pub fn builder() -> PayoutSubscriptionPaymentDetailsBuilder {
        <PayoutSubscriptionPaymentDetailsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayoutSubscriptionPaymentDetailsBuilder {
    total_amount: Option<f64>,
    service_fee: Option<f64>,
    surcharge_fee: Option<f64>,
    check_number: Option<String>,
    check_image: Option<HashMap<String, serde_json::Value>>,
    check_unique_id: Option<String>,
    currency: Option<String>,
    order_description: Option<String>,
    order_id: Option<String>,
    order_id_alternative: Option<String>,
    payment_description: Option<String>,
    settlement_descriptor: Option<String>,
    group_number: Option<String>,
    source: Option<String>,
    payabli_trans_id: Option<String>,
    unbundled: Option<bool>,
    categories: Option<Vec<PaymentCategories>>,
    split_funding: Option<SplitFunding>,
}

impl PayoutSubscriptionPaymentDetailsBuilder {
    pub fn total_amount(mut self, value: f64) -> Self {
        self.total_amount = Some(value);
        self
    }

    pub fn service_fee(mut self, value: f64) -> Self {
        self.service_fee = Some(value);
        self
    }

    pub fn surcharge_fee(mut self, value: f64) -> Self {
        self.surcharge_fee = Some(value);
        self
    }

    pub fn check_number(mut self, value: impl Into<String>) -> Self {
        self.check_number = Some(value.into());
        self
    }

    pub fn check_image(mut self, value: HashMap<String, serde_json::Value>) -> Self {
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

    pub fn categories(mut self, value: Vec<PaymentCategories>) -> Self {
        self.categories = Some(value);
        self
    }

    pub fn split_funding(mut self, value: SplitFunding) -> Self {
        self.split_funding = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayoutSubscriptionPaymentDetails`].
    pub fn build(self) -> Result<PayoutSubscriptionPaymentDetails, BuildError> {
        Ok(PayoutSubscriptionPaymentDetails {
            total_amount: self.total_amount,
            service_fee: self.service_fee,
            surcharge_fee: self.surcharge_fee,
            check_number: self.check_number,
            check_image: self.check_image,
            check_unique_id: self.check_unique_id,
            currency: self.currency,
            order_description: self.order_description,
            order_id: self.order_id,
            order_id_alternative: self.order_id_alternative,
            payment_description: self.payment_description,
            settlement_descriptor: self.settlement_descriptor,
            group_number: self.group_number,
            source: self.source,
            payabli_trans_id: self.payabli_trans_id,
            unbundled: self.unbundled,
            categories: self.categories,
            split_funding: self.split_funding,
        })
    }
}
