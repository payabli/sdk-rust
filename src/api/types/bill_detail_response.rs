pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BillDetailResponse {
    /// Bills associated with this transaction.
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
    /// Payout statuses, also known as money out transaction statuses, appear in PartnerHub and PayHub, and the API, and describe where a payout transaction is in its lifecycle.
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
    /// Text description of the payout transaction status.
    #[serde(rename = "StatusText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_text: Option<String>,
    /// Transaction total amount (including service fee or sub-charge).
    #[serde(rename = "TotalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
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

impl BillDetailResponse {
    pub fn builder() -> BillDetailResponseBuilder {
        <BillDetailResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillDetailResponseBuilder {
    bills: Option<Vec<BillDetailsResponse>>,
    check_data: Option<FileContent>,
    check_number: Option<String>,
    comments: Option<Comments>,
    created_date: Option<CreatedAt>,
    created_at: Option<CreatedAt>,
    events: Option<Vec<QueryTransactionEvents>>,
    fee_amount: Option<FeeAmount>,
    gateway: Option<Gatewayfield>,
    id_out: Option<i64>,
    last_updated: Option<LastModified>,
    net_amount: Option<Netamountnullable>,
    parent_org_name: Option<OrgParentName>,
    parent_org_id: Option<OrgParentId>,
    payment_data: Option<QueryPaymentData>,
    payment_group: Option<String>,
    payment_id: Option<PaymentIdString>,
    payment_method: Option<String>,
    payment_status: Option<String>,
    paypoint_dbaname: Option<Dbaname>,
    paypoint_legalname: Option<Legalname>,
    source: Option<Source>,
    status: Option<i64>,
    status_text: Option<String>,
    total_amount: Option<f64>,
    vendor: Option<VendorQueryRecord>,
    external_paypoint_id: Option<ExternalPaypointId>,
    entry_name: Option<Entry>,
    batch_id: Option<String>,
    has_vcard_transactions: Option<HasVcardTransactions>,
    is_same_day_ach: Option<IsSameDayAch>,
    schedule_id: Option<ScheduleId>,
    settlement_status: Option<SettlementStatus>,
    risk_flagged: Option<RiskFlagged>,
    risk_flagged_on: Option<RiskFlaggedOn>,
    risk_status: Option<RiskStatus>,
    risk_reason: Option<RiskReason>,
    risk_action: Option<RiskAction>,
    risk_action_code: Option<RiskActionCode>,
}

impl BillDetailResponseBuilder {
    pub fn bills(mut self, value: Vec<BillDetailsResponse>) -> Self {
        self.bills = Some(value);
        self
    }

    pub fn check_data(mut self, value: FileContent) -> Self {
        self.check_data = Some(value);
        self
    }

    pub fn check_number(mut self, value: impl Into<String>) -> Self {
        self.check_number = Some(value.into());
        self
    }

    pub fn comments(mut self, value: Comments) -> Self {
        self.comments = Some(value);
        self
    }

    pub fn created_date(mut self, value: CreatedAt) -> Self {
        self.created_date = Some(value);
        self
    }

    pub fn created_at(mut self, value: CreatedAt) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn events(mut self, value: Vec<QueryTransactionEvents>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn fee_amount(mut self, value: FeeAmount) -> Self {
        self.fee_amount = Some(value);
        self
    }

    pub fn gateway(mut self, value: Gatewayfield) -> Self {
        self.gateway = Some(value);
        self
    }

    pub fn id_out(mut self, value: i64) -> Self {
        self.id_out = Some(value);
        self
    }

    pub fn last_updated(mut self, value: LastModified) -> Self {
        self.last_updated = Some(value);
        self
    }

    pub fn net_amount(mut self, value: Netamountnullable) -> Self {
        self.net_amount = Some(value);
        self
    }

    pub fn parent_org_name(mut self, value: OrgParentName) -> Self {
        self.parent_org_name = Some(value);
        self
    }

    pub fn parent_org_id(mut self, value: OrgParentId) -> Self {
        self.parent_org_id = Some(value);
        self
    }

    pub fn payment_data(mut self, value: QueryPaymentData) -> Self {
        self.payment_data = Some(value);
        self
    }

    pub fn payment_group(mut self, value: impl Into<String>) -> Self {
        self.payment_group = Some(value.into());
        self
    }

    pub fn payment_id(mut self, value: PaymentIdString) -> Self {
        self.payment_id = Some(value);
        self
    }

    pub fn payment_method(mut self, value: impl Into<String>) -> Self {
        self.payment_method = Some(value.into());
        self
    }

    pub fn payment_status(mut self, value: impl Into<String>) -> Self {
        self.payment_status = Some(value.into());
        self
    }

    pub fn paypoint_dbaname(mut self, value: Dbaname) -> Self {
        self.paypoint_dbaname = Some(value);
        self
    }

    pub fn paypoint_legalname(mut self, value: Legalname) -> Self {
        self.paypoint_legalname = Some(value);
        self
    }

    pub fn source(mut self, value: Source) -> Self {
        self.source = Some(value);
        self
    }

    pub fn status(mut self, value: i64) -> Self {
        self.status = Some(value);
        self
    }

    pub fn status_text(mut self, value: impl Into<String>) -> Self {
        self.status_text = Some(value.into());
        self
    }

    pub fn total_amount(mut self, value: f64) -> Self {
        self.total_amount = Some(value);
        self
    }

    pub fn vendor(mut self, value: VendorQueryRecord) -> Self {
        self.vendor = Some(value);
        self
    }

    pub fn external_paypoint_id(mut self, value: ExternalPaypointId) -> Self {
        self.external_paypoint_id = Some(value);
        self
    }

    pub fn entry_name(mut self, value: Entry) -> Self {
        self.entry_name = Some(value);
        self
    }

    pub fn batch_id(mut self, value: impl Into<String>) -> Self {
        self.batch_id = Some(value.into());
        self
    }

    pub fn has_vcard_transactions(mut self, value: HasVcardTransactions) -> Self {
        self.has_vcard_transactions = Some(value);
        self
    }

    pub fn is_same_day_ach(mut self, value: IsSameDayAch) -> Self {
        self.is_same_day_ach = Some(value);
        self
    }

    pub fn schedule_id(mut self, value: ScheduleId) -> Self {
        self.schedule_id = Some(value);
        self
    }

    pub fn settlement_status(mut self, value: SettlementStatus) -> Self {
        self.settlement_status = Some(value);
        self
    }

    pub fn risk_flagged(mut self, value: RiskFlagged) -> Self {
        self.risk_flagged = Some(value);
        self
    }

    pub fn risk_flagged_on(mut self, value: RiskFlaggedOn) -> Self {
        self.risk_flagged_on = Some(value);
        self
    }

    pub fn risk_status(mut self, value: RiskStatus) -> Self {
        self.risk_status = Some(value);
        self
    }

    pub fn risk_reason(mut self, value: RiskReason) -> Self {
        self.risk_reason = Some(value);
        self
    }

    pub fn risk_action(mut self, value: RiskAction) -> Self {
        self.risk_action = Some(value);
        self
    }

    pub fn risk_action_code(mut self, value: RiskActionCode) -> Self {
        self.risk_action_code = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BillDetailResponse`].
    pub fn build(self) -> Result<BillDetailResponse, BuildError> {
        Ok(BillDetailResponse {
            bills: self.bills,
            check_data: self.check_data,
            check_number: self.check_number,
            comments: self.comments,
            created_date: self.created_date,
            created_at: self.created_at,
            events: self.events,
            fee_amount: self.fee_amount,
            gateway: self.gateway,
            id_out: self.id_out,
            last_updated: self.last_updated,
            net_amount: self.net_amount,
            parent_org_name: self.parent_org_name,
            parent_org_id: self.parent_org_id,
            payment_data: self.payment_data,
            payment_group: self.payment_group,
            payment_id: self.payment_id,
            payment_method: self.payment_method,
            payment_status: self.payment_status,
            paypoint_dbaname: self.paypoint_dbaname,
            paypoint_legalname: self.paypoint_legalname,
            source: self.source,
            status: self.status,
            status_text: self.status_text,
            total_amount: self.total_amount,
            vendor: self.vendor,
            external_paypoint_id: self.external_paypoint_id,
            entry_name: self.entry_name,
            batch_id: self.batch_id,
            has_vcard_transactions: self.has_vcard_transactions,
            is_same_day_ach: self.is_same_day_ach,
            schedule_id: self.schedule_id,
            settlement_status: self.settlement_status,
            risk_flagged: self.risk_flagged,
            risk_flagged_on: self.risk_flagged_on,
            risk_status: self.risk_status,
            risk_reason: self.risk_reason,
            risk_action: self.risk_action,
            risk_action_code: self.risk_action_code,
        })
    }
}
