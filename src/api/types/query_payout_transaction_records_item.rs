pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryPayoutTransactionRecordsItem {
    #[serde(rename = "BatchNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_number: Option<BatchNumber>,
    /// Identifier of the batch associated with payout transaction.
    #[serde(rename = "BatchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<i64>,
    /// Events associated with this transaction.
    #[serde(rename = "Bills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bills: Option<Vec<BillPayOutData>>,
    #[serde(rename = "CardToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_token: Option<String>,
    /// Object referencing paper check image.
    #[serde(rename = "CheckData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_data: Option<FileContent>,
    /// Paper check number related to payout transaction.
    #[serde(rename = "CheckNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_number: Option<String>,
    /// Any comment or description for payout transaction.
    #[serde(rename = "Comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Comments>,
    /// Timestamp when the payment was created, in UTC.
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    #[serde(rename = "EntryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_name: Option<Entrypointfield>,
    /// Events associated with this transaction.
    #[serde(rename = "Events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<QueryTransactionEvents>>,
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    #[serde(rename = "FeeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_amount: Option<FeeAmount>,
    #[serde(rename = "Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<Gatewayfield>,
    #[serde(rename = "HasVcardTransactions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_vcard_transactions: Option<HasVcardTransactions>,
    /// Identifier of payout transaction.
    #[serde(rename = "IdOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_out: Option<i64>,
    #[serde(rename = "IsSameDayACH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_same_day_ach: Option<IsSameDayAch>,
    /// Timestamp when payment record was updated.
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<LastModified>,
    /// Net amount paid.
    #[serde(rename = "NetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<Netamountnullable>,
    #[serde(rename = "ParentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<OrgParentName>,
    #[serde(rename = "ParentOrgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_id: Option<OrgParentId>,
    #[serde(rename = "PaymentData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_data: Option<QueryPayoutTransactionRecordsItemPaymentData>,
    #[serde(rename = "PaymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<PaymentIdString>,
    /// The payment method for the transaction.
    #[serde(rename = "PaymentMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    /// Status of payout transaction. See [Payout Transaction Statuses](guides/money-out-statuses#payout-transaction-statuses) for a full reference.
    #[serde(rename = "PaymentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_status: Option<String>,
    #[serde(rename = "PayoutProgram")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_program: Option<PayoutProgram>,
    #[serde(rename = "PaypointDbaname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dbaname: Option<Dbaname>,
    /// Paypoint legal name.
    #[serde(rename = "PaypointLegalname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legalname: Option<Legalname>,
    #[serde(rename = "RiskAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_action: Option<RiskAction>,
    #[serde(rename = "RiskActionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_action_code: Option<RiskActionCode>,
    #[serde(rename = "RiskFlagged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_flagged: Option<RiskFlagged>,
    #[serde(rename = "RiskFlaggedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_flagged_on: Option<RiskFlaggedOn>,
    #[serde(rename = "RiskReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_reason: Option<RiskReason>,
    #[serde(rename = "RiskStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_status: Option<RiskStatus>,
    #[serde(rename = "ScheduleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_id: Option<ScheduleId>,
    #[serde(rename = "SettlementStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_status: Option<SettlementStatusPayout>,
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    /// Internal status of transaction.
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// Transaction total amount (including service fee or sub-charge).
    #[serde(rename = "TotalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    /// Vendor related to the payout transaction.
    #[serde(rename = "Vendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<VendorQueryRecord>,
}
