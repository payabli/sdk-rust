pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BatchDetailResponseSummary {
    #[serde(rename = "serviceFees")]
    #[serde(default)]
    pub service_fees: f64,
    #[serde(rename = "transferAmount")]
    #[serde(default)]
    pub transfer_amount: f64,
    #[serde(default)]
    pub refunds: f64,
    #[serde(rename = "heldAmount")]
    #[serde(default)]
    pub held_amount: f64,
    #[serde(rename = "totalRecords")]
    #[serde(default)]
    pub total_records: Totalrecords,
    #[serde(rename = "totalAmount")]
    #[serde(default)]
    pub total_amount: f64,
    #[serde(rename = "totalNetAmount")]
    #[serde(default)]
    pub total_net_amount: f64,
    #[serde(rename = "totalPages")]
    #[serde(default)]
    pub total_pages: Totalpages,
    #[serde(rename = "pageSize")]
    #[serde(default)]
    pub page_size: Pagesize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageidentifier: Option<PageIdentifier>,
}

impl BatchDetailResponseSummary {
    pub fn builder() -> BatchDetailResponseSummaryBuilder {
        <BatchDetailResponseSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchDetailResponseSummaryBuilder {
    service_fees: Option<f64>,
    transfer_amount: Option<f64>,
    refunds: Option<f64>,
    held_amount: Option<f64>,
    total_records: Option<Totalrecords>,
    total_amount: Option<f64>,
    total_net_amount: Option<f64>,
    total_pages: Option<Totalpages>,
    page_size: Option<Pagesize>,
    pageidentifier: Option<PageIdentifier>,
}

impl BatchDetailResponseSummaryBuilder {
    pub fn service_fees(mut self, value: f64) -> Self {
        self.service_fees = Some(value);
        self
    }

    pub fn transfer_amount(mut self, value: f64) -> Self {
        self.transfer_amount = Some(value);
        self
    }

    pub fn refunds(mut self, value: f64) -> Self {
        self.refunds = Some(value);
        self
    }

    pub fn held_amount(mut self, value: f64) -> Self {
        self.held_amount = Some(value);
        self
    }

    pub fn total_records(mut self, value: Totalrecords) -> Self {
        self.total_records = Some(value);
        self
    }

    pub fn total_amount(mut self, value: f64) -> Self {
        self.total_amount = Some(value);
        self
    }

    pub fn total_net_amount(mut self, value: f64) -> Self {
        self.total_net_amount = Some(value);
        self
    }

    pub fn total_pages(mut self, value: Totalpages) -> Self {
        self.total_pages = Some(value);
        self
    }

    pub fn page_size(mut self, value: Pagesize) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn pageidentifier(mut self, value: PageIdentifier) -> Self {
        self.pageidentifier = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BatchDetailResponseSummary`].
    /// This method will fail if any of the following fields are not set:
    /// - [`service_fees`](BatchDetailResponseSummaryBuilder::service_fees)
    /// - [`transfer_amount`](BatchDetailResponseSummaryBuilder::transfer_amount)
    /// - [`refunds`](BatchDetailResponseSummaryBuilder::refunds)
    /// - [`held_amount`](BatchDetailResponseSummaryBuilder::held_amount)
    /// - [`total_records`](BatchDetailResponseSummaryBuilder::total_records)
    /// - [`total_amount`](BatchDetailResponseSummaryBuilder::total_amount)
    /// - [`total_net_amount`](BatchDetailResponseSummaryBuilder::total_net_amount)
    /// - [`total_pages`](BatchDetailResponseSummaryBuilder::total_pages)
    /// - [`page_size`](BatchDetailResponseSummaryBuilder::page_size)
    pub fn build(self) -> Result<BatchDetailResponseSummary, BuildError> {
        Ok(BatchDetailResponseSummary {
            service_fees: self
                .service_fees
                .ok_or_else(|| BuildError::missing_field("service_fees"))?,
            transfer_amount: self
                .transfer_amount
                .ok_or_else(|| BuildError::missing_field("transfer_amount"))?,
            refunds: self
                .refunds
                .ok_or_else(|| BuildError::missing_field("refunds"))?,
            held_amount: self
                .held_amount
                .ok_or_else(|| BuildError::missing_field("held_amount"))?,
            total_records: self
                .total_records
                .ok_or_else(|| BuildError::missing_field("total_records"))?,
            total_amount: self
                .total_amount
                .ok_or_else(|| BuildError::missing_field("total_amount"))?,
            total_net_amount: self
                .total_net_amount
                .ok_or_else(|| BuildError::missing_field("total_net_amount"))?,
            total_pages: self
                .total_pages
                .ok_or_else(|| BuildError::missing_field("total_pages"))?,
            page_size: self
                .page_size
                .ok_or_else(|| BuildError::missing_field("page_size"))?,
            pageidentifier: self.pageidentifier,
        })
    }
}
