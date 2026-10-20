pub use crate::prelude::*;

/// Expected Pay In processing profile for the business.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ProcessingMetrics {
    /// Expected monthly Pay In volume.
    #[serde(rename = "monthlyReceivablesVolume")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub monthly_receivables_volume: Option<f64>,
    /// Expected average transaction amount.
    #[serde(rename = "averageReceivableSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub average_receivable_size: Option<f64>,
    /// Expected largest transaction amount.
    #[serde(rename = "largestReceivableSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub largest_receivable_size: Option<f64>,
    /// Percent of volume from card-present transactions.
    #[serde(rename = "inboundPresentPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_present_percent: Option<i64>,
    /// Percent of volume from online transactions.
    #[serde(rename = "inboundOnlinePercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_online_percent: Option<i64>,
    /// Percent of volume from mail-order/telephone-order transactions.
    #[serde(rename = "inboundMotoPercent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_moto_percent: Option<i64>,
    /// The business's refund policy.
    #[serde(rename = "refundPolicy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_policy: Option<String>,
    /// Whether the business delivers goods or services before it's paid.
    #[serde(rename = "advancedDelivery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub advanced_delivery: Option<bool>,
    /// Percent of sales delivered before payment.
    #[serde(rename = "percentAdvancedDelivery")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_advanced_delivery: Option<i64>,
    /// Average number of days between payment and delivery.
    #[serde(rename = "advancedDeliveryDays")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub advanced_delivery_days: Option<f64>,
    /// Expected number of transactions per month.
    #[serde(rename = "numberOfTransactions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number_of_transactions: Option<i64>,
}

impl ProcessingMetrics {
    pub fn builder() -> ProcessingMetricsBuilder {
        <ProcessingMetricsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ProcessingMetricsBuilder {
    monthly_receivables_volume: Option<f64>,
    average_receivable_size: Option<f64>,
    largest_receivable_size: Option<f64>,
    inbound_present_percent: Option<i64>,
    inbound_online_percent: Option<i64>,
    inbound_moto_percent: Option<i64>,
    refund_policy: Option<String>,
    advanced_delivery: Option<bool>,
    percent_advanced_delivery: Option<i64>,
    advanced_delivery_days: Option<f64>,
    number_of_transactions: Option<i64>,
}

impl ProcessingMetricsBuilder {
    pub fn monthly_receivables_volume(mut self, value: f64) -> Self {
        self.monthly_receivables_volume = Some(value);
        self
    }

    pub fn average_receivable_size(mut self, value: f64) -> Self {
        self.average_receivable_size = Some(value);
        self
    }

    pub fn largest_receivable_size(mut self, value: f64) -> Self {
        self.largest_receivable_size = Some(value);
        self
    }

    pub fn inbound_present_percent(mut self, value: i64) -> Self {
        self.inbound_present_percent = Some(value);
        self
    }

    pub fn inbound_online_percent(mut self, value: i64) -> Self {
        self.inbound_online_percent = Some(value);
        self
    }

    pub fn inbound_moto_percent(mut self, value: i64) -> Self {
        self.inbound_moto_percent = Some(value);
        self
    }

    pub fn refund_policy(mut self, value: impl Into<String>) -> Self {
        self.refund_policy = Some(value.into());
        self
    }

    pub fn advanced_delivery(mut self, value: bool) -> Self {
        self.advanced_delivery = Some(value);
        self
    }

    pub fn percent_advanced_delivery(mut self, value: i64) -> Self {
        self.percent_advanced_delivery = Some(value);
        self
    }

    pub fn advanced_delivery_days(mut self, value: f64) -> Self {
        self.advanced_delivery_days = Some(value);
        self
    }

    pub fn number_of_transactions(mut self, value: i64) -> Self {
        self.number_of_transactions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ProcessingMetrics`].
    pub fn build(self) -> Result<ProcessingMetrics, BuildError> {
        Ok(ProcessingMetrics {
            monthly_receivables_volume: self.monthly_receivables_volume,
            average_receivable_size: self.average_receivable_size,
            largest_receivable_size: self.largest_receivable_size,
            inbound_present_percent: self.inbound_present_percent,
            inbound_online_percent: self.inbound_online_percent,
            inbound_moto_percent: self.inbound_moto_percent,
            refund_policy: self.refund_policy,
            advanced_delivery: self.advanced_delivery,
            percent_advanced_delivery: self.percent_advanced_delivery,
            advanced_delivery_days: self.advanced_delivery_days,
            number_of_transactions: self.number_of_transactions,
        })
    }
}
