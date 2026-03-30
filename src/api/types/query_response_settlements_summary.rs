pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryResponseSettlementsSummary {
    /// Funds being held for fraud or risk concerns.
    #[serde(rename = "heldAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub held_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageidentifier: Option<PageIdentifier>,
    /// Number of records per page.
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    /// Total refunds deducted from the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunds: Option<f64>,
    /// Service fees are any pass-through fees charged to the customer at the time of payment. These aren't transferred to the merchant when the batch is transferred and funded.
    #[serde(rename = "serviceFees")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_fees: Option<f64>,
    /// The total sum of the settlements in the response.
    #[serde(rename = "totalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    /// The total sum of the settlements in the response.
    #[serde(rename = "totalNetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_net_amount: Option<f64>,
    /// Number of pages in the response.
    #[serde(rename = "totalPages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<i64>,
    /// Number of records in the response.
    #[serde(rename = "totalRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_records: Option<i64>,
    /// The transfer amount is the net batch amount plus or minus any returns, refunds, billing and fees items, chargebacks, adjustments, and third party payments. This is the amount from the batch that's transferred to the merchant bank account.
    #[serde(rename = "transferAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_amount: Option<f64>,
}

impl QueryResponseSettlementsSummary {
    pub fn builder() -> QueryResponseSettlementsSummaryBuilder {
        <QueryResponseSettlementsSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryResponseSettlementsSummaryBuilder {
    held_amount: Option<f64>,
    pageidentifier: Option<PageIdentifier>,
    page_size: Option<i64>,
    refunds: Option<f64>,
    service_fees: Option<f64>,
    total_amount: Option<f64>,
    total_net_amount: Option<f64>,
    total_pages: Option<i64>,
    total_records: Option<i64>,
    transfer_amount: Option<f64>,
}

impl QueryResponseSettlementsSummaryBuilder {
    pub fn held_amount(mut self, value: f64) -> Self {
        self.held_amount = Some(value);
        self
    }

    pub fn pageidentifier(mut self, value: PageIdentifier) -> Self {
        self.pageidentifier = Some(value);
        self
    }

    pub fn page_size(mut self, value: i64) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn refunds(mut self, value: f64) -> Self {
        self.refunds = Some(value);
        self
    }

    pub fn service_fees(mut self, value: f64) -> Self {
        self.service_fees = Some(value);
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

    pub fn total_pages(mut self, value: i64) -> Self {
        self.total_pages = Some(value);
        self
    }

    pub fn total_records(mut self, value: i64) -> Self {
        self.total_records = Some(value);
        self
    }

    pub fn transfer_amount(mut self, value: f64) -> Self {
        self.transfer_amount = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryResponseSettlementsSummary`].
    pub fn build(self) -> Result<QueryResponseSettlementsSummary, BuildError> {
        Ok(QueryResponseSettlementsSummary {
            held_amount: self.held_amount,
            pageidentifier: self.pageidentifier,
            page_size: self.page_size,
            refunds: self.refunds,
            service_fees: self.service_fees,
            total_amount: self.total_amount,
            total_net_amount: self.total_net_amount,
            total_pages: self.total_pages,
            total_records: self.total_records,
            transfer_amount: self.transfer_amount,
        })
    }
}
