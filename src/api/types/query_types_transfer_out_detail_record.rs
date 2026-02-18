pub use crate::prelude::*;

/// A record representing an outbound transfer detail.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransferOutDetailRecord {
    /// Unique identifier for the transfer detail.
    #[serde(rename = "transferDetailId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_detail_id: Option<i64>,
    /// The ID of the transfer this detail belongs to.
    #[serde(rename = "transferId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<i64>,
    /// The transaction ID in Payabli's system.
    #[serde(rename = "transactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// The outbound transaction ID.
    #[serde(rename = "IdOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_out: Option<i64>,
    /// Payment method used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// The transaction type (credit or debit).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Category of the transaction detail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// The gross amount of the transaction.
    #[serde(rename = "grossAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gross_amount: Option<f64>,
    /// Amount returned.
    #[serde(rename = "returnedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_amount: Option<f64>,
    /// Amount refunded.
    #[serde(rename = "refundAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_amount: Option<f64>,
    /// Amount being held.
    #[serde(rename = "holdAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hold_amount: Option<f64>,
    /// Amount released.
    #[serde(rename = "releasedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_amount: Option<f64>,
    /// Billing fees amount.
    #[serde(rename = "billingFeesAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_fees_amount: Option<f64>,
    /// Adjustments amount.
    #[serde(rename = "adjustmentsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjustments_amount: Option<f64>,
    /// Net transfer amount after deductions.
    #[serde(rename = "netTransferAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_transfer_amount: Option<f64>,
    /// Detailed breakdown of billing fees.
    #[serde(rename = "billingFeesDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_fees_details: Option<Vec<BillingFeeDetail>>,
    /// Date and time the record was created.
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Comments on the transfer detail.
    #[serde(rename = "Comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Vendor information.
    #[serde(rename = "Vendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<TransferOutDetailVendor>,
    /// DBA name of the paypoint.
    #[serde(rename = "PaypointDbaname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dbaname: Option<String>,
    /// Legal name of the paypoint.
    #[serde(rename = "PaypointLegalname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legalname: Option<String>,
    /// ID of the paypoint.
    #[serde(rename = "PaypointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_id: Option<i64>,
    /// Status of the transfer detail.
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// Payment ID.
    #[serde(rename = "PaymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    /// Transaction ID.
    #[serde(rename = "TransId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_id: Option<String>,
    /// Transaction status.
    #[serde(rename = "TransStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_status: Option<i64>,
    /// Detailed transaction status.
    #[serde(rename = "TransStatusDetail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_status_detail: Option<String>,
    /// Name of the transaction status.
    #[serde(rename = "TransStatusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_status_name: Option<String>,
    /// Category of the transaction status.
    #[serde(rename = "TransStatusCategory")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_status_category: Option<String>,
    /// Date and time the record was last updated.
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    /// Total amount of the transaction.
    #[serde(rename = "TotalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    /// Net amount of the transaction.
    #[serde(rename = "NetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<f64>,
    /// Fee amount for the transaction.
    #[serde(rename = "FeeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_amount: Option<f64>,
    /// Source of the transaction.
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Name of the parent organization.
    #[serde(rename = "ParentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<String>,
    /// ID of the parent organization.
    #[serde(rename = "ParentOrgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_id: Option<i64>,
    /// Batch number for the transfer.
    #[serde(rename = "BatchNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_number: Option<String>,
    /// Status of the payment.
    #[serde(rename = "PaymentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_status: Option<String>,
    /// Payment method used.
    #[serde(rename = "PaymentMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    /// Token for the card used.
    #[serde(rename = "CardToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_token: Option<String>,
    /// Check number if paid by check.
    #[serde(rename = "CheckNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_number: Option<String>,
    /// Check data if paid by check.
    #[serde(rename = "CheckData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_data: Option<TransferOutDetailCheckData>,
    /// Payment data for the transaction.
    #[serde(rename = "PaymentData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_data: Option<TransferOutDetailPaymentData>,
    /// Bills associated with the transfer.
    #[serde(rename = "Bills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bills: Option<Vec<TransferOutDetailBill>>,
    /// Events associated with the transfer.
    #[serde(rename = "Events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<TransferOutDetailEvent>>,
    /// External paypoint ID.
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<String>,
    /// Entry name for the paypoint.
    #[serde(rename = "EntryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_name: Option<String>,
    /// Gateway used for the transaction.
    #[serde(rename = "Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<String>,
    /// ID of the batch.
    #[serde(rename = "BatchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<i64>,
    /// Whether the transfer has virtual card transactions.
    #[serde(rename = "HasVcardTransactions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_vcard_transactions: Option<bool>,
    /// Whether this is a same-day ACH transaction.
    #[serde(rename = "IsSameDayACH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_same_day_ach: Option<bool>,
    /// ID of the schedule if applicable.
    #[serde(rename = "ScheduleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_id: Option<i64>,
    /// Settlement status.
    #[serde(rename = "SettlementStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_status: Option<String>,
    /// Name of the settlement status.
    #[serde(rename = "SettlementStatusName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_status_name: Option<String>,
    /// Date of settlement.
    #[serde(rename = "SettlementDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_date: Option<String>,
    /// Whether the transaction was flagged for risk.
    #[serde(rename = "RiskFlagged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_flagged: Option<bool>,
    /// Date and time the transaction was flagged.
    #[serde(rename = "RiskFlaggedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_flagged_on: Option<String>,
    /// Risk status of the transaction.
    #[serde(rename = "RiskStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_status: Option<String>,
    /// Reason for the risk flag.
    #[serde(rename = "RiskReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_reason: Option<String>,
    /// Action taken for risk.
    #[serde(rename = "RiskAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_action: Option<String>,
    /// Code for the risk action.
    #[serde(rename = "RiskActionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_action_code: Option<i64>,
    /// Payout program used.
    #[serde(rename = "PayoutProgram")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_program: Option<String>,
    /// ACH trace number.
    #[serde(rename = "AchTraceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_trace_number: Option<String>,
}