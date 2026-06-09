pub use crate::prelude::*;

/// A record representing an outbound transfer detail.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
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
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub gross_amount: Option<f64>,
    /// Amount returned.
    #[serde(rename = "returnedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub returned_amount: Option<f64>,
    /// Amount refunded.
    #[serde(rename = "refundAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub refund_amount: Option<f64>,
    /// Amount being held.
    #[serde(rename = "holdAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub hold_amount: Option<f64>,
    /// Amount released.
    #[serde(rename = "releasedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub released_amount: Option<f64>,
    /// Billing fees amount.
    #[serde(rename = "billingFeesAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub billing_fees_amount: Option<f64>,
    /// Adjustments amount.
    #[serde(rename = "adjustmentsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub adjustments_amount: Option<f64>,
    /// Net transfer amount after deductions.
    #[serde(rename = "netTransferAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
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
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_amount: Option<f64>,
    /// Net amount of the transaction.
    #[serde(rename = "NetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub net_amount: Option<f64>,
    /// Fee amount for the transaction.
    #[serde(rename = "FeeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
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

impl TransferOutDetailRecord {
    pub fn builder() -> TransferOutDetailRecordBuilder {
        <TransferOutDetailRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferOutDetailRecordBuilder {
    transfer_detail_id: Option<i64>,
    transfer_id: Option<i64>,
    transaction_id: Option<String>,
    id_out: Option<i64>,
    method: Option<String>,
    r#type: Option<String>,
    category: Option<String>,
    gross_amount: Option<f64>,
    returned_amount: Option<f64>,
    refund_amount: Option<f64>,
    hold_amount: Option<f64>,
    released_amount: Option<f64>,
    billing_fees_amount: Option<f64>,
    adjustments_amount: Option<f64>,
    net_transfer_amount: Option<f64>,
    billing_fees_details: Option<Vec<BillingFeeDetail>>,
    created_at: Option<String>,
    comments: Option<String>,
    vendor: Option<TransferOutDetailVendor>,
    paypoint_dbaname: Option<String>,
    paypoint_legalname: Option<String>,
    paypoint_id: Option<i64>,
    status: Option<i64>,
    payment_id: Option<String>,
    trans_id: Option<String>,
    trans_status: Option<i64>,
    trans_status_detail: Option<String>,
    trans_status_name: Option<String>,
    trans_status_category: Option<String>,
    last_updated: Option<String>,
    total_amount: Option<f64>,
    net_amount: Option<f64>,
    fee_amount: Option<f64>,
    source: Option<String>,
    parent_org_name: Option<String>,
    parent_org_id: Option<i64>,
    batch_number: Option<String>,
    payment_status: Option<String>,
    payment_method: Option<String>,
    card_token: Option<String>,
    check_number: Option<String>,
    check_data: Option<TransferOutDetailCheckData>,
    payment_data: Option<TransferOutDetailPaymentData>,
    bills: Option<Vec<TransferOutDetailBill>>,
    events: Option<Vec<TransferOutDetailEvent>>,
    external_paypoint_id: Option<String>,
    entry_name: Option<String>,
    gateway: Option<String>,
    batch_id: Option<i64>,
    has_vcard_transactions: Option<bool>,
    is_same_day_ach: Option<bool>,
    schedule_id: Option<i64>,
    settlement_status: Option<String>,
    settlement_status_name: Option<String>,
    settlement_date: Option<String>,
    risk_flagged: Option<bool>,
    risk_flagged_on: Option<String>,
    risk_status: Option<String>,
    risk_reason: Option<String>,
    risk_action: Option<String>,
    risk_action_code: Option<i64>,
    payout_program: Option<String>,
    ach_trace_number: Option<String>,
}

impl TransferOutDetailRecordBuilder {
    pub fn transfer_detail_id(mut self, value: i64) -> Self {
        self.transfer_detail_id = Some(value);
        self
    }

    pub fn transfer_id(mut self, value: i64) -> Self {
        self.transfer_id = Some(value);
        self
    }

    pub fn transaction_id(mut self, value: impl Into<String>) -> Self {
        self.transaction_id = Some(value.into());
        self
    }

    pub fn id_out(mut self, value: i64) -> Self {
        self.id_out = Some(value);
        self
    }

    pub fn method(mut self, value: impl Into<String>) -> Self {
        self.method = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn gross_amount(mut self, value: f64) -> Self {
        self.gross_amount = Some(value);
        self
    }

    pub fn returned_amount(mut self, value: f64) -> Self {
        self.returned_amount = Some(value);
        self
    }

    pub fn refund_amount(mut self, value: f64) -> Self {
        self.refund_amount = Some(value);
        self
    }

    pub fn hold_amount(mut self, value: f64) -> Self {
        self.hold_amount = Some(value);
        self
    }

    pub fn released_amount(mut self, value: f64) -> Self {
        self.released_amount = Some(value);
        self
    }

    pub fn billing_fees_amount(mut self, value: f64) -> Self {
        self.billing_fees_amount = Some(value);
        self
    }

    pub fn adjustments_amount(mut self, value: f64) -> Self {
        self.adjustments_amount = Some(value);
        self
    }

    pub fn net_transfer_amount(mut self, value: f64) -> Self {
        self.net_transfer_amount = Some(value);
        self
    }

    pub fn billing_fees_details(mut self, value: Vec<BillingFeeDetail>) -> Self {
        self.billing_fees_details = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn comments(mut self, value: impl Into<String>) -> Self {
        self.comments = Some(value.into());
        self
    }

    pub fn vendor(mut self, value: TransferOutDetailVendor) -> Self {
        self.vendor = Some(value);
        self
    }

    pub fn paypoint_dbaname(mut self, value: impl Into<String>) -> Self {
        self.paypoint_dbaname = Some(value.into());
        self
    }

    pub fn paypoint_legalname(mut self, value: impl Into<String>) -> Self {
        self.paypoint_legalname = Some(value.into());
        self
    }

    pub fn paypoint_id(mut self, value: i64) -> Self {
        self.paypoint_id = Some(value);
        self
    }

    pub fn status(mut self, value: i64) -> Self {
        self.status = Some(value);
        self
    }

    pub fn payment_id(mut self, value: impl Into<String>) -> Self {
        self.payment_id = Some(value.into());
        self
    }

    pub fn trans_id(mut self, value: impl Into<String>) -> Self {
        self.trans_id = Some(value.into());
        self
    }

    pub fn trans_status(mut self, value: i64) -> Self {
        self.trans_status = Some(value);
        self
    }

    pub fn trans_status_detail(mut self, value: impl Into<String>) -> Self {
        self.trans_status_detail = Some(value.into());
        self
    }

    pub fn trans_status_name(mut self, value: impl Into<String>) -> Self {
        self.trans_status_name = Some(value.into());
        self
    }

    pub fn trans_status_category(mut self, value: impl Into<String>) -> Self {
        self.trans_status_category = Some(value.into());
        self
    }

    pub fn last_updated(mut self, value: impl Into<String>) -> Self {
        self.last_updated = Some(value.into());
        self
    }

    pub fn total_amount(mut self, value: f64) -> Self {
        self.total_amount = Some(value);
        self
    }

    pub fn net_amount(mut self, value: f64) -> Self {
        self.net_amount = Some(value);
        self
    }

    pub fn fee_amount(mut self, value: f64) -> Self {
        self.fee_amount = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn parent_org_name(mut self, value: impl Into<String>) -> Self {
        self.parent_org_name = Some(value.into());
        self
    }

    pub fn parent_org_id(mut self, value: i64) -> Self {
        self.parent_org_id = Some(value);
        self
    }

    pub fn batch_number(mut self, value: impl Into<String>) -> Self {
        self.batch_number = Some(value.into());
        self
    }

    pub fn payment_status(mut self, value: impl Into<String>) -> Self {
        self.payment_status = Some(value.into());
        self
    }

    pub fn payment_method(mut self, value: impl Into<String>) -> Self {
        self.payment_method = Some(value.into());
        self
    }

    pub fn card_token(mut self, value: impl Into<String>) -> Self {
        self.card_token = Some(value.into());
        self
    }

    pub fn check_number(mut self, value: impl Into<String>) -> Self {
        self.check_number = Some(value.into());
        self
    }

    pub fn check_data(mut self, value: TransferOutDetailCheckData) -> Self {
        self.check_data = Some(value);
        self
    }

    pub fn payment_data(mut self, value: TransferOutDetailPaymentData) -> Self {
        self.payment_data = Some(value);
        self
    }

    pub fn bills(mut self, value: Vec<TransferOutDetailBill>) -> Self {
        self.bills = Some(value);
        self
    }

    pub fn events(mut self, value: Vec<TransferOutDetailEvent>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn external_paypoint_id(mut self, value: impl Into<String>) -> Self {
        self.external_paypoint_id = Some(value.into());
        self
    }

    pub fn entry_name(mut self, value: impl Into<String>) -> Self {
        self.entry_name = Some(value.into());
        self
    }

    pub fn gateway(mut self, value: impl Into<String>) -> Self {
        self.gateway = Some(value.into());
        self
    }

    pub fn batch_id(mut self, value: i64) -> Self {
        self.batch_id = Some(value);
        self
    }

    pub fn has_vcard_transactions(mut self, value: bool) -> Self {
        self.has_vcard_transactions = Some(value);
        self
    }

    pub fn is_same_day_ach(mut self, value: bool) -> Self {
        self.is_same_day_ach = Some(value);
        self
    }

    pub fn schedule_id(mut self, value: i64) -> Self {
        self.schedule_id = Some(value);
        self
    }

    pub fn settlement_status(mut self, value: impl Into<String>) -> Self {
        self.settlement_status = Some(value.into());
        self
    }

    pub fn settlement_status_name(mut self, value: impl Into<String>) -> Self {
        self.settlement_status_name = Some(value.into());
        self
    }

    pub fn settlement_date(mut self, value: impl Into<String>) -> Self {
        self.settlement_date = Some(value.into());
        self
    }

    pub fn risk_flagged(mut self, value: bool) -> Self {
        self.risk_flagged = Some(value);
        self
    }

    pub fn risk_flagged_on(mut self, value: impl Into<String>) -> Self {
        self.risk_flagged_on = Some(value.into());
        self
    }

    pub fn risk_status(mut self, value: impl Into<String>) -> Self {
        self.risk_status = Some(value.into());
        self
    }

    pub fn risk_reason(mut self, value: impl Into<String>) -> Self {
        self.risk_reason = Some(value.into());
        self
    }

    pub fn risk_action(mut self, value: impl Into<String>) -> Self {
        self.risk_action = Some(value.into());
        self
    }

    pub fn risk_action_code(mut self, value: i64) -> Self {
        self.risk_action_code = Some(value);
        self
    }

    pub fn payout_program(mut self, value: impl Into<String>) -> Self {
        self.payout_program = Some(value.into());
        self
    }

    pub fn ach_trace_number(mut self, value: impl Into<String>) -> Self {
        self.ach_trace_number = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransferOutDetailRecord`].
    pub fn build(self) -> Result<TransferOutDetailRecord, BuildError> {
        Ok(TransferOutDetailRecord {
            transfer_detail_id: self.transfer_detail_id,
            transfer_id: self.transfer_id,
            transaction_id: self.transaction_id,
            id_out: self.id_out,
            method: self.method,
            r#type: self.r#type,
            category: self.category,
            gross_amount: self.gross_amount,
            returned_amount: self.returned_amount,
            refund_amount: self.refund_amount,
            hold_amount: self.hold_amount,
            released_amount: self.released_amount,
            billing_fees_amount: self.billing_fees_amount,
            adjustments_amount: self.adjustments_amount,
            net_transfer_amount: self.net_transfer_amount,
            billing_fees_details: self.billing_fees_details,
            created_at: self.created_at,
            comments: self.comments,
            vendor: self.vendor,
            paypoint_dbaname: self.paypoint_dbaname,
            paypoint_legalname: self.paypoint_legalname,
            paypoint_id: self.paypoint_id,
            status: self.status,
            payment_id: self.payment_id,
            trans_id: self.trans_id,
            trans_status: self.trans_status,
            trans_status_detail: self.trans_status_detail,
            trans_status_name: self.trans_status_name,
            trans_status_category: self.trans_status_category,
            last_updated: self.last_updated,
            total_amount: self.total_amount,
            net_amount: self.net_amount,
            fee_amount: self.fee_amount,
            source: self.source,
            parent_org_name: self.parent_org_name,
            parent_org_id: self.parent_org_id,
            batch_number: self.batch_number,
            payment_status: self.payment_status,
            payment_method: self.payment_method,
            card_token: self.card_token,
            check_number: self.check_number,
            check_data: self.check_data,
            payment_data: self.payment_data,
            bills: self.bills,
            events: self.events,
            external_paypoint_id: self.external_paypoint_id,
            entry_name: self.entry_name,
            gateway: self.gateway,
            batch_id: self.batch_id,
            has_vcard_transactions: self.has_vcard_transactions,
            is_same_day_ach: self.is_same_day_ach,
            schedule_id: self.schedule_id,
            settlement_status: self.settlement_status,
            settlement_status_name: self.settlement_status_name,
            settlement_date: self.settlement_date,
            risk_flagged: self.risk_flagged,
            risk_flagged_on: self.risk_flagged_on,
            risk_status: self.risk_status,
            risk_reason: self.risk_reason,
            risk_action: self.risk_action,
            risk_action_code: self.risk_action_code,
            payout_program: self.payout_program,
            ach_trace_number: self.ach_trace_number,
        })
    }
}
