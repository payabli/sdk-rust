pub use crate::prelude::*;

/// Expected Pay Out profile for the business.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PayoutMetrics {
    /// Expected monthly Pay Out volume.
    #[serde(rename = "monthlyPayablesVolume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub monthly_payables_volume: Option<f64>,
    /// Expected average payout amount.
    #[serde(rename = "averagePayablesSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub average_payables_size: Option<f64>,
    /// Expected largest payout amount.
    #[serde(rename = "largestPayablesSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub largest_payables_size: Option<f64>,
    /// Expected number of payouts per month.
    #[serde(rename = "numberOfTransactions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_transactions: Option<i64>,
}

impl PayoutMetrics {
    pub fn builder() -> PayoutMetricsBuilder {
        <PayoutMetricsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayoutMetricsBuilder {
    monthly_payables_volume: Option<f64>,
    average_payables_size: Option<f64>,
    largest_payables_size: Option<f64>,
    number_of_transactions: Option<i64>,
}

impl PayoutMetricsBuilder {
    pub fn monthly_payables_volume(mut self, value: f64) -> Self {
        self.monthly_payables_volume = Some(value);
        self
    }

    pub fn average_payables_size(mut self, value: f64) -> Self {
        self.average_payables_size = Some(value);
        self
    }

    pub fn largest_payables_size(mut self, value: f64) -> Self {
        self.largest_payables_size = Some(value);
        self
    }

    pub fn number_of_transactions(mut self, value: i64) -> Self {
        self.number_of_transactions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayoutMetrics`].
    pub fn build(self) -> Result<PayoutMetrics, BuildError> {
        Ok(PayoutMetrics {
            monthly_payables_volume: self.monthly_payables_volume,
            average_payables_size: self.average_payables_size,
            largest_payables_size: self.largest_payables_size,
            number_of_transactions: self.number_of_transactions,
        })
    }
}
