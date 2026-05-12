pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TransactionOutQueryRecord {
    /// Identifier of payout transaction.
    #[serde(rename = "IdOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_out: Option<i64>,
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    /// Any comment or description for payout transaction.
    #[serde(rename = "Comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Comments>,
    /// Vendor related to the payout transaction.
    #[serde(rename = "Vendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<VendorQueryRecord>,
    #[serde(rename = "PaypointDbaname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dbaname: Option<Dbaname>,
    #[serde(rename = "PaypointLegalname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legalname: Option<Legalname>,
    /// Internal status of transaction.
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// Timestamp when payment record was updated, in UTC.
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<LastModified>,
    /// Transaction total amount (including service fee or sub-charge).
    #[serde(rename = "TotalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_amount: Option<f64>,
    #[serde(rename = "NetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<Netamountnullable>,
    #[serde(rename = "FeeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_amount: Option<FeeAmount>,
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(rename = "ParentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<OrgParentName>,
    #[serde(rename = "ParentOrgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_id: Option<Orgid>,
    /// The batch number for the payout transaction.
    #[serde(rename = "BatchNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_number: Option<String>,
    /// Status of payout transaction.
    #[serde(rename = "PaymentStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_status: Option<String>,
    /// Method of payment applied to the transaction.
    #[serde(rename = "PaymentMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    #[serde(rename = "CardToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_token: Option<String>,
    /// Paper check number related to payout transaction.
    #[serde(rename = "CheckNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_number: Option<String>,
    /// Object referencing to paper check image.
    #[serde(rename = "CheckData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_data: Option<FileContent>,
    #[serde(rename = "PaymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<PaymentIdString>,
    #[serde(rename = "PaymentData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_data: Option<QueryPaymentData>,
    /// Bills associated with this transaction.
    #[serde(rename = "Bills")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bills: Option<Vec<BillPayOutData>>,
    /// Events associated with this transaction.
    #[serde(rename = "Events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<QueryTransactionEvents>>,
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    #[serde(rename = "EntryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_name: Option<Entrypointfield>,
    #[serde(rename = "Gateway")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<Gatewayfield>,
    /// ID of the batch the transaction belongs to.
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
    pub settlement_status: Option<SettlementStatusPayout>,
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
    #[serde(rename = "PayoutProgram")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_program: Option<PayoutProgram>,
}

impl TransactionOutQueryRecord {
    pub fn builder() -> TransactionOutQueryRecordBuilder {
        <TransactionOutQueryRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionOutQueryRecordBuilder {
    id_out: Option<i64>,
    created_at: Option<CreatedAt>,
    comments: Option<Comments>,
    vendor: Option<VendorQueryRecord>,
    paypoint_dbaname: Option<Dbaname>,
    paypoint_legalname: Option<Legalname>,
    status: Option<i64>,
    last_updated: Option<LastModified>,
    total_amount: Option<f64>,
    net_amount: Option<Netamountnullable>,
    fee_amount: Option<FeeAmount>,
    source: Option<Source>,
    parent_org_name: Option<OrgParentName>,
    parent_org_id: Option<Orgid>,
    batch_number: Option<String>,
    payment_status: Option<String>,
    payment_method: Option<String>,
    card_token: Option<String>,
    check_number: Option<String>,
    check_data: Option<FileContent>,
    payment_id: Option<PaymentIdString>,
    payment_data: Option<QueryPaymentData>,
    bills: Option<Vec<BillPayOutData>>,
    events: Option<Vec<QueryTransactionEvents>>,
    external_paypoint_id: Option<ExternalPaypointId>,
    entry_name: Option<Entrypointfield>,
    gateway: Option<Gatewayfield>,
    batch_id: Option<String>,
    has_vcard_transactions: Option<HasVcardTransactions>,
    is_same_day_ach: Option<IsSameDayAch>,
    schedule_id: Option<ScheduleId>,
    settlement_status: Option<SettlementStatusPayout>,
    risk_flagged: Option<RiskFlagged>,
    risk_flagged_on: Option<RiskFlaggedOn>,
    risk_status: Option<RiskStatus>,
    risk_reason: Option<RiskReason>,
    risk_action: Option<RiskAction>,
    risk_action_code: Option<RiskActionCode>,
    payout_program: Option<PayoutProgram>,
}

impl TransactionOutQueryRecordBuilder {
    pub fn id_out(mut self, value: i64) -> Self {
        self.id_out = Some(value);
        self
    }

    pub fn created_at(mut self, value: CreatedAt) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn comments(mut self, value: Comments) -> Self {
        self.comments = Some(value);
        self
    }

    pub fn vendor(mut self, value: VendorQueryRecord) -> Self {
        self.vendor = Some(value);
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

    pub fn status(mut self, value: i64) -> Self {
        self.status = Some(value);
        self
    }

    pub fn last_updated(mut self, value: LastModified) -> Self {
        self.last_updated = Some(value);
        self
    }

    pub fn total_amount(mut self, value: f64) -> Self {
        self.total_amount = Some(value);
        self
    }

    pub fn net_amount(mut self, value: Netamountnullable) -> Self {
        self.net_amount = Some(value);
        self
    }

    pub fn fee_amount(mut self, value: FeeAmount) -> Self {
        self.fee_amount = Some(value);
        self
    }

    pub fn source(mut self, value: Source) -> Self {
        self.source = Some(value);
        self
    }

    pub fn parent_org_name(mut self, value: OrgParentName) -> Self {
        self.parent_org_name = Some(value);
        self
    }

    pub fn parent_org_id(mut self, value: Orgid) -> Self {
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

    pub fn check_data(mut self, value: FileContent) -> Self {
        self.check_data = Some(value);
        self
    }

    pub fn payment_id(mut self, value: PaymentIdString) -> Self {
        self.payment_id = Some(value);
        self
    }

    pub fn payment_data(mut self, value: QueryPaymentData) -> Self {
        self.payment_data = Some(value);
        self
    }

    pub fn bills(mut self, value: Vec<BillPayOutData>) -> Self {
        self.bills = Some(value);
        self
    }

    pub fn events(mut self, value: Vec<QueryTransactionEvents>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn external_paypoint_id(mut self, value: ExternalPaypointId) -> Self {
        self.external_paypoint_id = Some(value);
        self
    }

    pub fn entry_name(mut self, value: Entrypointfield) -> Self {
        self.entry_name = Some(value);
        self
    }

    pub fn gateway(mut self, value: Gatewayfield) -> Self {
        self.gateway = Some(value);
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

    pub fn settlement_status(mut self, value: SettlementStatusPayout) -> Self {
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

    pub fn payout_program(mut self, value: PayoutProgram) -> Self {
        self.payout_program = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransactionOutQueryRecord`].
    pub fn build(self) -> Result<TransactionOutQueryRecord, BuildError> {
        Ok(TransactionOutQueryRecord {
            id_out: self.id_out,
            created_at: self.created_at,
            comments: self.comments,
            vendor: self.vendor,
            paypoint_dbaname: self.paypoint_dbaname,
            paypoint_legalname: self.paypoint_legalname,
            status: self.status,
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
            payment_id: self.payment_id,
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
            risk_flagged: self.risk_flagged,
            risk_flagged_on: self.risk_flagged_on,
            risk_status: self.risk_status,
            risk_reason: self.risk_reason,
            risk_action: self.risk_action,
            risk_action_code: self.risk_action_code,
            payout_program: self.payout_program,
        })
    }
}
