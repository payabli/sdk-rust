pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryTransferSummary {
    /// ACH returns deducted from the batch.
    #[serde(rename = "achReturns")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_returns: Option<f64>,
    /// Corrections applied to Billing & Fees charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustments: Option<f64>,
    /// Charges applied for transactions and services.
    #[serde(rename = "billingFees")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_fees: Option<f64>,
    /// Chargebacks deducted from batch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chargebacks: Option<f64>,
    /// The gross batch amount before deductions.
    #[serde(rename = "grossTransferAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_transfer_amount: Option<f64>,
    /// Previously held funds that have been released after a risk review.
    #[serde(rename = "releaseAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_amount: Option<f64>,
    /// Payments captured in the batch cycle that are deposited separately. For example,  checks or cash payments recorded in the batch but not deposited via Payabli,  or card brands making a direct transfer in certain situations.
    #[serde(rename = "thirdPartyPaid")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub third_party_paid: Option<f64>,
    /// The gross batch amount minus service fees.
    #[serde(rename = "totalNetAmountTransfer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_net_amount_transfer: Option<f64>,
    /// The sum of each splitFundingAmount of each record in the transfer.
    #[serde(rename = "splitAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_amount: Option<f64>,
    /// Service fees are any pass-through fees charged to the customer at the time of payment.  These aren't transferred to the merchant when the batch is transferred and funded.
    #[serde(rename = "serviceFees")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_fees: Option<f64>,
    /// The net batch amount is the gross batch amount minus any returns, refunds,
    /// billing and fees items, chargebacks, adjustments, and third party payments.
    #[serde(rename = "netBatchAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_batch_amount: Option<f64>,
    /// The transfer amount is the net batch amount plus or minus any returns, refunds,  billing and fees items, chargebacks, adjustments, and third party payments.  This is the amount from the batch that is transferred to the merchant bank account.
    #[serde(rename = "transferAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_amount: Option<f64>,
    /// Refunds deducted from batch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunds: Option<f64>,
    /// Funds being held for fraud or risk concerns.
    #[serde(rename = "heldAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub held_amount: Option<f64>,
    /// Total amount rejected by card networks or issuing banks after authorization or settling. This value is the sum of all rejected amounts for transactions in the transfer.
    #[serde(rename = "cardRejectedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_rejected_amount: Option<f64>,
    /// Number of records in the response.
    #[serde(rename = "totalRecords")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_records: Option<Totalrecords>,
    /// The total sum of the transfers in the response.
    #[serde(rename = "totalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    /// The total sum of the transfers in the response.
    #[serde(rename = "totalNetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_net_amount: Option<f64>,
    /// Number of pages in the response.
    #[serde(rename = "totalPages")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_pages: Option<Totalpages>,
    /// Number of records per page.
    #[serde(rename = "pageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<Pagesize>,
    /// Auxiliary validation used internally by payment pages and components.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageidentifier: Option<PageIdentifier>,
}

impl QueryTransferSummary {
    pub fn builder() -> QueryTransferSummaryBuilder {
        <QueryTransferSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryTransferSummaryBuilder {
    ach_returns: Option<f64>,
    adjustments: Option<f64>,
    billing_fees: Option<f64>,
    chargebacks: Option<f64>,
    gross_transfer_amount: Option<f64>,
    release_amount: Option<f64>,
    third_party_paid: Option<f64>,
    total_net_amount_transfer: Option<f64>,
    split_amount: Option<f64>,
    service_fees: Option<f64>,
    net_batch_amount: Option<f64>,
    transfer_amount: Option<f64>,
    refunds: Option<f64>,
    held_amount: Option<f64>,
    card_rejected_amount: Option<f64>,
    total_records: Option<Totalrecords>,
    total_amount: Option<f64>,
    total_net_amount: Option<f64>,
    total_pages: Option<Totalpages>,
    page_size: Option<Pagesize>,
    pageidentifier: Option<PageIdentifier>,
}

impl QueryTransferSummaryBuilder {
    pub fn ach_returns(mut self, value: f64) -> Self {
        self.ach_returns = Some(value);
        self
    }

    pub fn adjustments(mut self, value: f64) -> Self {
        self.adjustments = Some(value);
        self
    }

    pub fn billing_fees(mut self, value: f64) -> Self {
        self.billing_fees = Some(value);
        self
    }

    pub fn chargebacks(mut self, value: f64) -> Self {
        self.chargebacks = Some(value);
        self
    }

    pub fn gross_transfer_amount(mut self, value: f64) -> Self {
        self.gross_transfer_amount = Some(value);
        self
    }

    pub fn release_amount(mut self, value: f64) -> Self {
        self.release_amount = Some(value);
        self
    }

    pub fn third_party_paid(mut self, value: f64) -> Self {
        self.third_party_paid = Some(value);
        self
    }

    pub fn total_net_amount_transfer(mut self, value: f64) -> Self {
        self.total_net_amount_transfer = Some(value);
        self
    }

    pub fn split_amount(mut self, value: f64) -> Self {
        self.split_amount = Some(value);
        self
    }

    pub fn service_fees(mut self, value: f64) -> Self {
        self.service_fees = Some(value);
        self
    }

    pub fn net_batch_amount(mut self, value: f64) -> Self {
        self.net_batch_amount = Some(value);
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

    pub fn card_rejected_amount(mut self, value: f64) -> Self {
        self.card_rejected_amount = Some(value);
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

    /// Consumes the builder and constructs a [`QueryTransferSummary`].
    pub fn build(self) -> Result<QueryTransferSummary, BuildError> {
        Ok(QueryTransferSummary {
            ach_returns: self.ach_returns,
            adjustments: self.adjustments,
            billing_fees: self.billing_fees,
            chargebacks: self.chargebacks,
            gross_transfer_amount: self.gross_transfer_amount,
            release_amount: self.release_amount,
            third_party_paid: self.third_party_paid,
            total_net_amount_transfer: self.total_net_amount_transfer,
            split_amount: self.split_amount,
            service_fees: self.service_fees,
            net_batch_amount: self.net_batch_amount,
            transfer_amount: self.transfer_amount,
            refunds: self.refunds,
            held_amount: self.held_amount,
            card_rejected_amount: self.card_rejected_amount,
            total_records: self.total_records,
            total_amount: self.total_amount,
            total_net_amount: self.total_net_amount,
            total_pages: self.total_pages,
            page_size: self.page_size,
            pageidentifier: self.pageidentifier,
        })
    }
}
