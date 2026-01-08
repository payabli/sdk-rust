pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BillDetailResponse {
    /// Events associated to this transaction.
    #[serde(rename = "Bills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bills: Option<Vec<BillDetailsResponse>>,
    /// Object referencing to paper check image.
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
    #[serde(rename = "CreatedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<CreatedAt>,
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    /// Events associated to this transaction.
    #[serde(rename = "Events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<QueryTransactionEvents>>,
    #[serde(rename = "FeeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_amount: Option<FeeAmount>,
    #[serde(rename = "Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<Gatewayfield>,
    /// Identifier of payout transaction.
    #[serde(rename = "IdOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_out: Option<i64>,
    /// Timestamp when payment record was updated, in UTC.
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<LastModified>,
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
    pub payment_data: Option<QueryPaymentData>,
    /// Unique identifier for group or batch containing the transaction.
    #[serde(rename = "PaymentGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_group: Option<String>,
    #[serde(rename = "PaymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<PaymentIdString>,
    /// Method of payment applied to the transaction.
    #[serde(rename = "PaymentMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    /// Status of payout transaction.
    #[serde(rename = "PaymentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_status: Option<String>,
    #[serde(rename = "PaypointDbaname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dbaname: Option<Dbaname>,
    /// Paypoint legal name.
    #[serde(rename = "PaypointLegalname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legalname: Option<Legalname>,
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    /// Internal status of transaction.
    ///
    /// Payout statuses, also known as money out transaction statuses, appear in PartnerHub and PayHub, and the API, and describe where a payout transaction is in its lifecycle.
    ///
    /// | Status | Key | Description | Events |
    /// |--------|-----|-------------|---------|
    /// | **Authorized** | 11 | A payout is authorized. These are queued payouts, and nothing happens with them until they're captured. | Authorized |
    /// | **Captured** | 1 | A payout is captured and is now part of the batch for payout. | Captured |
    /// | **Canceled** | 0 | An authorized payout has been canceled. A captured payout can be canceled before batch close at 5 PM ET. | Cancelled |
    /// | **Processing** | 2 | A payout is being processed. | Waiting funds, Funded, Pending (payment type is pending), Generating check |
    /// | **Processed** | 3 | A payment method is defined for the vendor, and the payout has been sent to the recipient. | Open (vCard issued, ACH sent, check generated but not yet cashed), Processed (Payment Type is no longer pending), Reissued, Returned, Errored |
    /// | **OnHold** | 4 | A payout has been placed on hold and requires review before proceeding. | OnHold |
    /// | **Paid** | 5 | A payout has been paid and the recipient has redeemed the funds. | Paid (check cleared, vCard used, ACH settled) |
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// Status of payout transaction.
    #[serde(rename = "StatusText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_text: Option<String>,
    /// Transaction total amount (including service fee or sub-charge).
    #[serde(rename = "TotalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    /// Vendor related to the payout transaction.
    #[serde(rename = "Vendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<VendorQueryRecord>,
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    #[serde(rename = "EntryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_name: Option<Entry>,
    /// Identifier for the batch in which this transaction was processed. Used to track and reconcile batch-level operations.
    #[serde(rename = "BatchId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_id: Option<String>,
    #[serde(rename = "HasVcardTransactions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_vcard_transactions: Option<HasVcardTransactions>,
    #[serde(rename = "IsSameDayACH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_same_day_ach: Option<IsSameDayAch>,
    #[serde(rename = "ScheduleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_id: Option<ScheduleId>,
    #[serde(rename = "SettlementStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_status: Option<SettlementStatus>,
    #[serde(rename = "RiskFlagged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_flagged: Option<RiskFlagged>,
    #[serde(rename = "RiskFlaggedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_flagged_on: Option<RiskFlaggedOn>,
    #[serde(rename = "RiskStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_status: Option<RiskStatus>,
    #[serde(rename = "RiskReason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_reason: Option<RiskReason>,
    #[serde(rename = "RiskAction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_action: Option<RiskAction>,
    #[serde(rename = "RiskActionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_action_code: Option<RiskActionCode>,
}
