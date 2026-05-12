pub use crate::prelude::*;

/// Details about the payment.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaymentDetail {
    /// Array of payment categories/line items describing the amount to be paid.
    /// **Note**: These categories are for information only and aren't validated against the total amount provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<PaymentCategories>>,
    /// Object containing image of paper check.
    #[serde(rename = "checkImage")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_image: Option<HashMap<String, serde_json::Value>>,
    /// A check number to be used in the ach transaction. **Required** for payment method = 'check'.
    #[serde(rename = "checkNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_number: Option<String>,
    /// The currency for the transaction, `USD` or `CAD`. If your paypoint is configured for CAD, you must send the `CAD` value in this field, otherwise it defaults to USD, which will cause the transaction to fail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Service fee to be deducted from the total amount. This amount must be a number, percentages aren't accepted. If you are using a percentage-based fee schedule, you must calculate the value manually.
    #[serde(rename = "serviceFee")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_fee: Option<f64>,
    /// Split funding instructions for the transaction. See [Split a Transaction](/developers/developer-guides/money-in-split-funding) for more.
    #[serde(rename = "splitFunding")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_funding: Option<SplitFunding>,
    /// Unique identifier for a processed check image. Required for RDC (Remote Deposit Capture) transactions where `achCode` is `BOC`. Use the `id` value from the [check processing](/developers/api-reference/moneyin/check-capture) response.
    #[serde(rename = "checkUniqueId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_unique_id: Option<String>,
    /// Total amount to be charged. If a service fee is sent, then this amount should include the service fee."
    #[serde(rename = "totalAmount")]
    #[serde(default)]
    pub total_amount: f64,
}

impl PaymentDetail {
    pub fn builder() -> PaymentDetailBuilder {
        <PaymentDetailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentDetailBuilder {
    categories: Option<Vec<PaymentCategories>>,
    check_image: Option<HashMap<String, serde_json::Value>>,
    check_number: Option<String>,
    currency: Option<String>,
    service_fee: Option<f64>,
    split_funding: Option<SplitFunding>,
    check_unique_id: Option<String>,
    total_amount: Option<f64>,
}

impl PaymentDetailBuilder {
    pub fn categories(mut self, value: Vec<PaymentCategories>) -> Self {
        self.categories = Some(value);
        self
    }

    pub fn check_image(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.check_image = Some(value);
        self
    }

    pub fn check_number(mut self, value: impl Into<String>) -> Self {
        self.check_number = Some(value.into());
        self
    }

    pub fn currency(mut self, value: impl Into<String>) -> Self {
        self.currency = Some(value.into());
        self
    }

    pub fn service_fee(mut self, value: f64) -> Self {
        self.service_fee = Some(value);
        self
    }

    pub fn split_funding(mut self, value: SplitFunding) -> Self {
        self.split_funding = Some(value);
        self
    }

    pub fn check_unique_id(mut self, value: impl Into<String>) -> Self {
        self.check_unique_id = Some(value.into());
        self
    }

    pub fn total_amount(mut self, value: f64) -> Self {
        self.total_amount = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentDetail`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total_amount`](PaymentDetailBuilder::total_amount)
    pub fn build(self) -> Result<PaymentDetail, BuildError> {
        Ok(PaymentDetail {
            categories: self.categories,
            check_image: self.check_image,
            check_number: self.check_number,
            currency: self.currency,
            service_fee: self.service_fee,
            split_funding: self.split_funding,
            check_unique_id: self.check_unique_id,
            total_amount: self
                .total_amount
                .ok_or_else(|| BuildError::missing_field("total_amount"))?,
        })
    }
}
