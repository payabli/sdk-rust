pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct StatisticsVendorQueryRecord {
    /// Statistical grouping identifier
    #[serde(rename = "statX")]
    #[serde(default)]
    pub stat_x: String,
    /// Number of active transactions
    #[serde(default)]
    pub active: i64,
    /// Volume of active transactions
    #[serde(rename = "activeVolume")]
    #[serde(default)]
    pub active_volume: f64,
    /// Number of transactions sent to approval
    #[serde(rename = "sentToApproval")]
    #[serde(default)]
    pub sent_to_approval: i64,
    /// Volume of transactions sent to approval
    #[serde(rename = "sentToApprovalVolume")]
    #[serde(default)]
    pub sent_to_approval_volume: f64,
    /// Number of transactions to approval
    #[serde(rename = "toApproval")]
    #[serde(default)]
    pub to_approval: i64,
    /// Volume of transactions to approval
    #[serde(rename = "toApprovalVolume")]
    #[serde(default)]
    pub to_approval_volume: f64,
    /// Number of approved transactions
    #[serde(default)]
    pub approved: i64,
    /// Volume of approved transactions
    #[serde(rename = "approvedVolume")]
    #[serde(default)]
    pub approved_volume: f64,
    /// Number of disapproved transactions
    #[serde(default)]
    pub disapproved: i64,
    /// Volume of disapproved transactions
    #[serde(rename = "disapprovedVolume")]
    #[serde(default)]
    pub disapproved_volume: f64,
    /// Number of cancelled transactions
    #[serde(default)]
    pub cancelled: i64,
    /// Volume of cancelled transactions
    #[serde(rename = "cancelledVolume")]
    #[serde(default)]
    pub cancelled_volume: f64,
    /// Number of transactions in transit
    #[serde(rename = "inTransit")]
    #[serde(default)]
    pub in_transit: i64,
    /// Volume of transactions in transit
    #[serde(rename = "inTransitVolume")]
    #[serde(default)]
    pub in_transit_volume: f64,
    /// Number of paid transactions
    #[serde(default)]
    pub paid: i64,
    /// Volume of paid transactions
    #[serde(rename = "paidVolume")]
    #[serde(default)]
    pub paid_volume: f64,
}

impl StatisticsVendorQueryRecord {
    pub fn builder() -> StatisticsVendorQueryRecordBuilder {
        <StatisticsVendorQueryRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StatisticsVendorQueryRecordBuilder {
    stat_x: Option<String>,
    active: Option<i64>,
    active_volume: Option<f64>,
    sent_to_approval: Option<i64>,
    sent_to_approval_volume: Option<f64>,
    to_approval: Option<i64>,
    to_approval_volume: Option<f64>,
    approved: Option<i64>,
    approved_volume: Option<f64>,
    disapproved: Option<i64>,
    disapproved_volume: Option<f64>,
    cancelled: Option<i64>,
    cancelled_volume: Option<f64>,
    in_transit: Option<i64>,
    in_transit_volume: Option<f64>,
    paid: Option<i64>,
    paid_volume: Option<f64>,
}

impl StatisticsVendorQueryRecordBuilder {
    pub fn stat_x(mut self, value: impl Into<String>) -> Self {
        self.stat_x = Some(value.into());
        self
    }

    pub fn active(mut self, value: i64) -> Self {
        self.active = Some(value);
        self
    }

    pub fn active_volume(mut self, value: f64) -> Self {
        self.active_volume = Some(value);
        self
    }

    pub fn sent_to_approval(mut self, value: i64) -> Self {
        self.sent_to_approval = Some(value);
        self
    }

    pub fn sent_to_approval_volume(mut self, value: f64) -> Self {
        self.sent_to_approval_volume = Some(value);
        self
    }

    pub fn to_approval(mut self, value: i64) -> Self {
        self.to_approval = Some(value);
        self
    }

    pub fn to_approval_volume(mut self, value: f64) -> Self {
        self.to_approval_volume = Some(value);
        self
    }

    pub fn approved(mut self, value: i64) -> Self {
        self.approved = Some(value);
        self
    }

    pub fn approved_volume(mut self, value: f64) -> Self {
        self.approved_volume = Some(value);
        self
    }

    pub fn disapproved(mut self, value: i64) -> Self {
        self.disapproved = Some(value);
        self
    }

    pub fn disapproved_volume(mut self, value: f64) -> Self {
        self.disapproved_volume = Some(value);
        self
    }

    pub fn cancelled(mut self, value: i64) -> Self {
        self.cancelled = Some(value);
        self
    }

    pub fn cancelled_volume(mut self, value: f64) -> Self {
        self.cancelled_volume = Some(value);
        self
    }

    pub fn in_transit(mut self, value: i64) -> Self {
        self.in_transit = Some(value);
        self
    }

    pub fn in_transit_volume(mut self, value: f64) -> Self {
        self.in_transit_volume = Some(value);
        self
    }

    pub fn paid(mut self, value: i64) -> Self {
        self.paid = Some(value);
        self
    }

    pub fn paid_volume(mut self, value: f64) -> Self {
        self.paid_volume = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StatisticsVendorQueryRecord`].
    /// This method will fail if any of the following fields are not set:
    /// - [`stat_x`](StatisticsVendorQueryRecordBuilder::stat_x)
    /// - [`active`](StatisticsVendorQueryRecordBuilder::active)
    /// - [`active_volume`](StatisticsVendorQueryRecordBuilder::active_volume)
    /// - [`sent_to_approval`](StatisticsVendorQueryRecordBuilder::sent_to_approval)
    /// - [`sent_to_approval_volume`](StatisticsVendorQueryRecordBuilder::sent_to_approval_volume)
    /// - [`to_approval`](StatisticsVendorQueryRecordBuilder::to_approval)
    /// - [`to_approval_volume`](StatisticsVendorQueryRecordBuilder::to_approval_volume)
    /// - [`approved`](StatisticsVendorQueryRecordBuilder::approved)
    /// - [`approved_volume`](StatisticsVendorQueryRecordBuilder::approved_volume)
    /// - [`disapproved`](StatisticsVendorQueryRecordBuilder::disapproved)
    /// - [`disapproved_volume`](StatisticsVendorQueryRecordBuilder::disapproved_volume)
    /// - [`cancelled`](StatisticsVendorQueryRecordBuilder::cancelled)
    /// - [`cancelled_volume`](StatisticsVendorQueryRecordBuilder::cancelled_volume)
    /// - [`in_transit`](StatisticsVendorQueryRecordBuilder::in_transit)
    /// - [`in_transit_volume`](StatisticsVendorQueryRecordBuilder::in_transit_volume)
    /// - [`paid`](StatisticsVendorQueryRecordBuilder::paid)
    /// - [`paid_volume`](StatisticsVendorQueryRecordBuilder::paid_volume)
    pub fn build(self) -> Result<StatisticsVendorQueryRecord, BuildError> {
        Ok(StatisticsVendorQueryRecord {
            stat_x: self
                .stat_x
                .ok_or_else(|| BuildError::missing_field("stat_x"))?,
            active: self
                .active
                .ok_or_else(|| BuildError::missing_field("active"))?,
            active_volume: self
                .active_volume
                .ok_or_else(|| BuildError::missing_field("active_volume"))?,
            sent_to_approval: self
                .sent_to_approval
                .ok_or_else(|| BuildError::missing_field("sent_to_approval"))?,
            sent_to_approval_volume: self
                .sent_to_approval_volume
                .ok_or_else(|| BuildError::missing_field("sent_to_approval_volume"))?,
            to_approval: self
                .to_approval
                .ok_or_else(|| BuildError::missing_field("to_approval"))?,
            to_approval_volume: self
                .to_approval_volume
                .ok_or_else(|| BuildError::missing_field("to_approval_volume"))?,
            approved: self
                .approved
                .ok_or_else(|| BuildError::missing_field("approved"))?,
            approved_volume: self
                .approved_volume
                .ok_or_else(|| BuildError::missing_field("approved_volume"))?,
            disapproved: self
                .disapproved
                .ok_or_else(|| BuildError::missing_field("disapproved"))?,
            disapproved_volume: self
                .disapproved_volume
                .ok_or_else(|| BuildError::missing_field("disapproved_volume"))?,
            cancelled: self
                .cancelled
                .ok_or_else(|| BuildError::missing_field("cancelled"))?,
            cancelled_volume: self
                .cancelled_volume
                .ok_or_else(|| BuildError::missing_field("cancelled_volume"))?,
            in_transit: self
                .in_transit
                .ok_or_else(|| BuildError::missing_field("in_transit"))?,
            in_transit_volume: self
                .in_transit_volume
                .ok_or_else(|| BuildError::missing_field("in_transit_volume"))?,
            paid: self.paid.ok_or_else(|| BuildError::missing_field("paid"))?,
            paid_volume: self
                .paid_volume
                .ok_or_else(|| BuildError::missing_field("paid_volume"))?,
        })
    }
}
