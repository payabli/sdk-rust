pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TierItemPass {
    #[serde(rename = "amountFeeone-time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount_feeone_time: Option<f64>,
    #[serde(rename = "amountFeeRecurring")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount_fee_recurring: Option<f64>,
    #[serde(rename = "highPayRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub high_pay_range: Option<f64>,
    #[serde(rename = "lowPayRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub low_pay_range: Option<f64>,
    #[serde(rename = "percentFeeone-time")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub percent_feeone_time: Option<f64>,
    #[serde(rename = "percentFeeRecurring")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub percent_fee_recurring: Option<f64>,
}

impl TierItemPass {
    pub fn builder() -> TierItemPassBuilder {
        <TierItemPassBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TierItemPassBuilder {
    amount_feeone_time: Option<f64>,
    amount_fee_recurring: Option<f64>,
    high_pay_range: Option<f64>,
    low_pay_range: Option<f64>,
    percent_feeone_time: Option<f64>,
    percent_fee_recurring: Option<f64>,
}

impl TierItemPassBuilder {
    pub fn amount_feeone_time(mut self, value: f64) -> Self {
        self.amount_feeone_time = Some(value);
        self
    }

    pub fn amount_fee_recurring(mut self, value: f64) -> Self {
        self.amount_fee_recurring = Some(value);
        self
    }

    pub fn high_pay_range(mut self, value: f64) -> Self {
        self.high_pay_range = Some(value);
        self
    }

    pub fn low_pay_range(mut self, value: f64) -> Self {
        self.low_pay_range = Some(value);
        self
    }

    pub fn percent_feeone_time(mut self, value: f64) -> Self {
        self.percent_feeone_time = Some(value);
        self
    }

    pub fn percent_fee_recurring(mut self, value: f64) -> Self {
        self.percent_fee_recurring = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TierItemPass`].
    pub fn build(self) -> Result<TierItemPass, BuildError> {
        Ok(TierItemPass {
            amount_feeone_time: self.amount_feeone_time,
            amount_fee_recurring: self.amount_fee_recurring,
            high_pay_range: self.high_pay_range,
            low_pay_range: self.low_pay_range,
            percent_feeone_time: self.percent_feeone_time,
            percent_fee_recurring: self.percent_fee_recurring,
        })
    }
}
