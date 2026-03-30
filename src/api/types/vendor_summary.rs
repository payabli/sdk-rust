pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VendorSummary {
    #[serde(rename = "ActiveBills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_bills: Option<i64>,
    #[serde(rename = "PendingBills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_bills: Option<i64>,
    #[serde(rename = "InTransitBills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_transit_bills: Option<i64>,
    #[serde(rename = "PaidBills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_bills: Option<i64>,
    #[serde(rename = "OverdueBills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overdue_bills: Option<i64>,
    #[serde(rename = "ApprovedBills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_bills: Option<i64>,
    #[serde(rename = "DisapprovedBills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disapproved_bills: Option<i64>,
    #[serde(rename = "TotalBills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bills: Option<i64>,
    #[serde(rename = "ActiveBillsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_bills_amount: Option<f64>,
    #[serde(rename = "PendingBillsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_bills_amount: Option<f64>,
    #[serde(rename = "InTransitBillsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub in_transit_bills_amount: Option<f64>,
    #[serde(rename = "PaidBillsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_bills_amount: Option<f64>,
    #[serde(rename = "OverdueBillsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overdue_bills_amount: Option<f64>,
    #[serde(rename = "ApprovedBillsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub approved_bills_amount: Option<f64>,
    #[serde(rename = "DisapprovedBillsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disapproved_bills_amount: Option<f64>,
    #[serde(rename = "TotalBillsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_bills_amount: Option<f64>,
}

impl VendorSummary {
    pub fn builder() -> VendorSummaryBuilder {
        <VendorSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VendorSummaryBuilder {
    active_bills: Option<i64>,
    pending_bills: Option<i64>,
    in_transit_bills: Option<i64>,
    paid_bills: Option<i64>,
    overdue_bills: Option<i64>,
    approved_bills: Option<i64>,
    disapproved_bills: Option<i64>,
    total_bills: Option<i64>,
    active_bills_amount: Option<f64>,
    pending_bills_amount: Option<f64>,
    in_transit_bills_amount: Option<f64>,
    paid_bills_amount: Option<f64>,
    overdue_bills_amount: Option<f64>,
    approved_bills_amount: Option<f64>,
    disapproved_bills_amount: Option<f64>,
    total_bills_amount: Option<f64>,
}

impl VendorSummaryBuilder {
    pub fn active_bills(mut self, value: i64) -> Self {
        self.active_bills = Some(value);
        self
    }

    pub fn pending_bills(mut self, value: i64) -> Self {
        self.pending_bills = Some(value);
        self
    }

    pub fn in_transit_bills(mut self, value: i64) -> Self {
        self.in_transit_bills = Some(value);
        self
    }

    pub fn paid_bills(mut self, value: i64) -> Self {
        self.paid_bills = Some(value);
        self
    }

    pub fn overdue_bills(mut self, value: i64) -> Self {
        self.overdue_bills = Some(value);
        self
    }

    pub fn approved_bills(mut self, value: i64) -> Self {
        self.approved_bills = Some(value);
        self
    }

    pub fn disapproved_bills(mut self, value: i64) -> Self {
        self.disapproved_bills = Some(value);
        self
    }

    pub fn total_bills(mut self, value: i64) -> Self {
        self.total_bills = Some(value);
        self
    }

    pub fn active_bills_amount(mut self, value: f64) -> Self {
        self.active_bills_amount = Some(value);
        self
    }

    pub fn pending_bills_amount(mut self, value: f64) -> Self {
        self.pending_bills_amount = Some(value);
        self
    }

    pub fn in_transit_bills_amount(mut self, value: f64) -> Self {
        self.in_transit_bills_amount = Some(value);
        self
    }

    pub fn paid_bills_amount(mut self, value: f64) -> Self {
        self.paid_bills_amount = Some(value);
        self
    }

    pub fn overdue_bills_amount(mut self, value: f64) -> Self {
        self.overdue_bills_amount = Some(value);
        self
    }

    pub fn approved_bills_amount(mut self, value: f64) -> Self {
        self.approved_bills_amount = Some(value);
        self
    }

    pub fn disapproved_bills_amount(mut self, value: f64) -> Self {
        self.disapproved_bills_amount = Some(value);
        self
    }

    pub fn total_bills_amount(mut self, value: f64) -> Self {
        self.total_bills_amount = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VendorSummary`].
    pub fn build(self) -> Result<VendorSummary, BuildError> {
        Ok(VendorSummary {
            active_bills: self.active_bills,
            pending_bills: self.pending_bills,
            in_transit_bills: self.in_transit_bills,
            paid_bills: self.paid_bills,
            overdue_bills: self.overdue_bills,
            approved_bills: self.approved_bills,
            disapproved_bills: self.disapproved_bills,
            total_bills: self.total_bills,
            active_bills_amount: self.active_bills_amount,
            pending_bills_amount: self.pending_bills_amount,
            in_transit_bills_amount: self.in_transit_bills_amount,
            paid_bills_amount: self.paid_bills_amount,
            overdue_bills_amount: self.overdue_bills_amount,
            approved_bills_amount: self.approved_bills_amount,
            disapproved_bills_amount: self.disapproved_bills_amount,
            total_bills_amount: self.total_bills_amount,
        })
    }
}
