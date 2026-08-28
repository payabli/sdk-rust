pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TransferDetailRecord {
    /// Unique identifier for the transfer detail record
    #[serde(rename = "transferDetailId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_detail_id: Option<i64>,
    /// The ID of the transfer this detail belongs to
    #[serde(rename = "transferId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<i64>,
    /// The transaction ID in Payabli's system
    #[serde(rename = "transactionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    /// External transaction reference number
    #[serde(rename = "transactionNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_number: Option<String>,
    /// The transaction type (credit or debit)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// A field used to categorize the transaction details. Values include: auth, decline, refund, adj, cb, split
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// The gross amount of the transaction
    #[serde(rename = "grossAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub gross_amount: Option<f64>,
    /// Chargeback amount deducted from transaction
    #[serde(rename = "chargeBackAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub charge_back_amount: Option<f64>,
    /// ACH return amount deducted from transaction
    #[serde(rename = "returnedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub returned_amount: Option<f64>,
    /// Refund amount deducted from transaction
    #[serde(rename = "refundAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub refund_amount: Option<f64>,
    /// Amount being held for fraud or risk concerns
    #[serde(rename = "holdAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub hold_amount: Option<f64>,
    /// Previously held funds that have been released after a risk review
    #[serde(rename = "releasedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub released_amount: Option<f64>,
    /// Charges applied for transactions and services
    #[serde(rename = "billingFeesAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub billing_fees_amount: Option<f64>,
    /// Payments captured in the batch cycle that are deposited separately. For example,  checks or cash payments recorded in the batch but not deposited via Payabli,  or card brands making a direct transfer in certain situations.
    #[serde(rename = "thirdPartyPaidAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub third_party_paid_amount: Option<f64>,
    /// Corrections applied to Billing & Fees charges
    #[serde(rename = "adjustmentsAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub adjustments_amount: Option<f64>,
    /// The net amount after all deductions
    #[serde(rename = "netTransferAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub net_transfer_amount: Option<f64>,
    /// Total amount directed to split funding destinations
    #[serde(rename = "splitFundingAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub split_funding_amount: Option<f64>,
    /// Total amount rejected by card networks or issuing banks after authorization or settling in this transaction
    #[serde(rename = "cardRejectedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub card_rejected_amount: Option<f64>,
    #[serde(rename = "billingFeesDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_fees_details: Option<Vec<BillingFeeDetail>>,
    #[serde(rename = "ParentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<OrgParentName>,
    #[serde(rename = "PaypointDbaname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dbaname: Option<Dbaname>,
    #[serde(rename = "PaypointLegalname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legalname: Option<Legalname>,
    /// The paypoint's entryname
    #[serde(rename = "PaypointEntryname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_entryname: Option<String>,
    /// The transaction ID for the payment
    #[serde(rename = "PaymentTransId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_trans_id: Option<String>,
    /// The payment connector used to process the transaction
    #[serde(rename = "ConnectorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_name: Option<String>,
    #[serde(rename = "ExternalProcessorInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_processor_information: Option<ExternalProcessorInformation>,
    /// Internal identifier used for processing
    #[serde(rename = "GatewayTransId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_trans_id: Option<String>,
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderId>,
    /// The payment method used for the transaction, for example card, ach, or device.
    #[serde(rename = "Method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(rename = "BatchNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_number: Option<BatchNumber>,
    /// The amount of the batch
    #[serde(rename = "BatchAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub batch_amount: Option<f64>,
    /// Unique ID for customer linked to the transaction
    #[serde(rename = "PayorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payor_id: Option<PayorId>,
    #[serde(rename = "PaymentData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_data: Option<QueryPaymentData>,
    /// Status of transaction. See [the
    /// docs](/developers/references/money-in-statuses#money-in-transaction-status) for a
    /// full reference.
    #[serde(rename = "TransStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_status: Option<i64>,
    #[serde(rename = "PaypointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_id: Option<PaypointId>,
    /// Transaction total amount (including service fee or sub-charge)
    #[serde(rename = "TotalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_amount: Option<f64>,
    /// Net amount paid
    #[serde(rename = "NetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<Netamountnullable>,
    #[serde(rename = "FeeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_amount: Option<FeeAmount>,
    /// Settlement status for transaction. See [the docs](/developers/references/money-in-statuses#payment-funding-status) for a full reference.
    #[serde(rename = "SettlementStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_status: Option<i64>,
    #[serde(rename = "Operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
    #[serde(rename = "ResponseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<QueryResponseData>,
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    /// Reference to the subscription or schedule that originated the transaction
    #[serde(rename = "ScheduleReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_reference: Option<i64>,
    #[serde(rename = "OrgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<Orgid>,
    #[serde(rename = "RefundId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_id: Option<RefundId>,
    #[serde(rename = "ReturnedId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_id: Option<ReturnedId>,
    #[serde(rename = "ChargebackId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chargeback_id: Option<ChargebackId>,
    #[serde(rename = "RetrievalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retrieval_id: Option<RetrievalId>,
    /// Additional transaction data
    #[serde(rename = "TransAdditionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_additional_data: Option<serde_json::Value>,
    /// Associated invoice data
    #[serde(rename = "invoiceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_data: Option<BillData>,
    #[serde(rename = "EntrypageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypage_id: Option<EntrypageId>,
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    /// Indicates whether the ACH account has been validated
    #[serde(rename = "IsValidatedACH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_validated_ach: Option<bool>,
    /// Transaction date and time, in UTC
    #[serde(rename = "TransactionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub transaction_time: Option<DateTime<Utc>>,
    #[serde(rename = "Customer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<QueryTransactionPayorData>,
    #[serde(rename = "splitFundingInstructions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_funding_instructions: Option<SplitFunding>,
    #[serde(rename = "CfeeTransactions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfee_transactions: Option<Vec<QueryCFeeTransaction>>,
    #[serde(rename = "TransactionEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_events: Option<Vec<QueryTransactionEvents>>,
    #[serde(rename = "PendingFeeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_fee_amount: Option<PendingFeeAmount>,
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
    #[serde(rename = "DeviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<Device>,
    #[serde(rename = "AchSecCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_sec_code: Option<AchSecCode>,
    #[serde(rename = "AchHolderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_holder_type: Option<AchHolderType>,
    #[serde(rename = "IpAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<IpAddress>,
    /// Indicates if this was a same-day ACH transaction.
    #[serde(rename = "IsSameDayACH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_same_day_ach: Option<bool>,
    /// Type of wallet used for the transaction (if applicable)
    #[serde(rename = "WalletType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_type: Option<String>,
}

impl TransferDetailRecord {
    pub fn builder() -> TransferDetailRecordBuilder {
        <TransferDetailRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferDetailRecordBuilder {
    transfer_detail_id: Option<i64>,
    transfer_id: Option<i64>,
    transaction_id: Option<String>,
    transaction_number: Option<String>,
    r#type: Option<String>,
    category: Option<String>,
    gross_amount: Option<f64>,
    charge_back_amount: Option<f64>,
    returned_amount: Option<f64>,
    refund_amount: Option<f64>,
    hold_amount: Option<f64>,
    released_amount: Option<f64>,
    billing_fees_amount: Option<f64>,
    third_party_paid_amount: Option<f64>,
    adjustments_amount: Option<f64>,
    net_transfer_amount: Option<f64>,
    split_funding_amount: Option<f64>,
    card_rejected_amount: Option<f64>,
    billing_fees_details: Option<Vec<BillingFeeDetail>>,
    parent_org_name: Option<OrgParentName>,
    paypoint_dbaname: Option<Dbaname>,
    paypoint_legalname: Option<Legalname>,
    paypoint_entryname: Option<String>,
    payment_trans_id: Option<String>,
    connector_name: Option<String>,
    external_processor_information: Option<ExternalProcessorInformation>,
    gateway_trans_id: Option<String>,
    order_id: Option<OrderId>,
    method: Option<String>,
    batch_number: Option<BatchNumber>,
    batch_amount: Option<f64>,
    payor_id: Option<PayorId>,
    payment_data: Option<QueryPaymentData>,
    trans_status: Option<i64>,
    paypoint_id: Option<PaypointId>,
    total_amount: Option<f64>,
    net_amount: Option<Netamountnullable>,
    fee_amount: Option<FeeAmount>,
    settlement_status: Option<i64>,
    operation: Option<Operation>,
    response_data: Option<QueryResponseData>,
    source: Option<Source>,
    schedule_reference: Option<i64>,
    org_id: Option<Orgid>,
    refund_id: Option<RefundId>,
    returned_id: Option<ReturnedId>,
    chargeback_id: Option<ChargebackId>,
    retrieval_id: Option<RetrievalId>,
    trans_additional_data: Option<serde_json::Value>,
    invoice_data: Option<BillData>,
    entrypage_id: Option<EntrypageId>,
    external_paypoint_id: Option<ExternalPaypointId>,
    is_validated_ach: Option<bool>,
    transaction_time: Option<DateTime<Utc>>,
    customer: Option<QueryTransactionPayorData>,
    split_funding_instructions: Option<SplitFunding>,
    cfee_transactions: Option<Vec<QueryCFeeTransaction>>,
    transaction_events: Option<Vec<QueryTransactionEvents>>,
    pending_fee_amount: Option<PendingFeeAmount>,
    risk_flagged: Option<RiskFlagged>,
    risk_flagged_on: Option<RiskFlaggedOn>,
    risk_status: Option<RiskStatus>,
    risk_reason: Option<RiskReason>,
    risk_action: Option<RiskAction>,
    risk_action_code: Option<RiskActionCode>,
    device_id: Option<Device>,
    ach_sec_code: Option<AchSecCode>,
    ach_holder_type: Option<AchHolderType>,
    ip_address: Option<IpAddress>,
    is_same_day_ach: Option<bool>,
    wallet_type: Option<String>,
}

impl TransferDetailRecordBuilder {
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

    pub fn transaction_number(mut self, value: impl Into<String>) -> Self {
        self.transaction_number = Some(value.into());
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

    pub fn charge_back_amount(mut self, value: f64) -> Self {
        self.charge_back_amount = Some(value);
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

    pub fn third_party_paid_amount(mut self, value: f64) -> Self {
        self.third_party_paid_amount = Some(value);
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

    pub fn split_funding_amount(mut self, value: f64) -> Self {
        self.split_funding_amount = Some(value);
        self
    }

    pub fn card_rejected_amount(mut self, value: f64) -> Self {
        self.card_rejected_amount = Some(value);
        self
    }

    pub fn billing_fees_details(mut self, value: Vec<BillingFeeDetail>) -> Self {
        self.billing_fees_details = Some(value);
        self
    }

    pub fn parent_org_name(mut self, value: OrgParentName) -> Self {
        self.parent_org_name = Some(value);
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

    pub fn paypoint_entryname(mut self, value: impl Into<String>) -> Self {
        self.paypoint_entryname = Some(value.into());
        self
    }

    pub fn payment_trans_id(mut self, value: impl Into<String>) -> Self {
        self.payment_trans_id = Some(value.into());
        self
    }

    pub fn connector_name(mut self, value: impl Into<String>) -> Self {
        self.connector_name = Some(value.into());
        self
    }

    pub fn external_processor_information(mut self, value: ExternalProcessorInformation) -> Self {
        self.external_processor_information = Some(value);
        self
    }

    pub fn gateway_trans_id(mut self, value: impl Into<String>) -> Self {
        self.gateway_trans_id = Some(value.into());
        self
    }

    pub fn order_id(mut self, value: OrderId) -> Self {
        self.order_id = Some(value);
        self
    }

    pub fn method(mut self, value: impl Into<String>) -> Self {
        self.method = Some(value.into());
        self
    }

    pub fn batch_number(mut self, value: BatchNumber) -> Self {
        self.batch_number = Some(value);
        self
    }

    pub fn batch_amount(mut self, value: f64) -> Self {
        self.batch_amount = Some(value);
        self
    }

    pub fn payor_id(mut self, value: PayorId) -> Self {
        self.payor_id = Some(value);
        self
    }

    pub fn payment_data(mut self, value: QueryPaymentData) -> Self {
        self.payment_data = Some(value);
        self
    }

    pub fn trans_status(mut self, value: i64) -> Self {
        self.trans_status = Some(value);
        self
    }

    pub fn paypoint_id(mut self, value: PaypointId) -> Self {
        self.paypoint_id = Some(value);
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

    pub fn settlement_status(mut self, value: i64) -> Self {
        self.settlement_status = Some(value);
        self
    }

    pub fn operation(mut self, value: Operation) -> Self {
        self.operation = Some(value);
        self
    }

    pub fn response_data(mut self, value: QueryResponseData) -> Self {
        self.response_data = Some(value);
        self
    }

    pub fn source(mut self, value: Source) -> Self {
        self.source = Some(value);
        self
    }

    pub fn schedule_reference(mut self, value: i64) -> Self {
        self.schedule_reference = Some(value);
        self
    }

    pub fn org_id(mut self, value: Orgid) -> Self {
        self.org_id = Some(value);
        self
    }

    pub fn refund_id(mut self, value: RefundId) -> Self {
        self.refund_id = Some(value);
        self
    }

    pub fn returned_id(mut self, value: ReturnedId) -> Self {
        self.returned_id = Some(value);
        self
    }

    pub fn chargeback_id(mut self, value: ChargebackId) -> Self {
        self.chargeback_id = Some(value);
        self
    }

    pub fn retrieval_id(mut self, value: RetrievalId) -> Self {
        self.retrieval_id = Some(value);
        self
    }

    pub fn trans_additional_data(mut self, value: serde_json::Value) -> Self {
        self.trans_additional_data = Some(value);
        self
    }

    pub fn invoice_data(mut self, value: BillData) -> Self {
        self.invoice_data = Some(value);
        self
    }

    pub fn entrypage_id(mut self, value: EntrypageId) -> Self {
        self.entrypage_id = Some(value);
        self
    }

    pub fn external_paypoint_id(mut self, value: ExternalPaypointId) -> Self {
        self.external_paypoint_id = Some(value);
        self
    }

    pub fn is_validated_ach(mut self, value: bool) -> Self {
        self.is_validated_ach = Some(value);
        self
    }

    pub fn transaction_time(mut self, value: DateTime<Utc>) -> Self {
        self.transaction_time = Some(value);
        self
    }

    pub fn customer(mut self, value: QueryTransactionPayorData) -> Self {
        self.customer = Some(value);
        self
    }

    pub fn split_funding_instructions(mut self, value: SplitFunding) -> Self {
        self.split_funding_instructions = Some(value);
        self
    }

    pub fn cfee_transactions(mut self, value: Vec<QueryCFeeTransaction>) -> Self {
        self.cfee_transactions = Some(value);
        self
    }

    pub fn transaction_events(mut self, value: Vec<QueryTransactionEvents>) -> Self {
        self.transaction_events = Some(value);
        self
    }

    pub fn pending_fee_amount(mut self, value: PendingFeeAmount) -> Self {
        self.pending_fee_amount = Some(value);
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

    pub fn device_id(mut self, value: Device) -> Self {
        self.device_id = Some(value);
        self
    }

    pub fn ach_sec_code(mut self, value: AchSecCode) -> Self {
        self.ach_sec_code = Some(value);
        self
    }

    pub fn ach_holder_type(mut self, value: AchHolderType) -> Self {
        self.ach_holder_type = Some(value);
        self
    }

    pub fn ip_address(mut self, value: IpAddress) -> Self {
        self.ip_address = Some(value);
        self
    }

    pub fn is_same_day_ach(mut self, value: bool) -> Self {
        self.is_same_day_ach = Some(value);
        self
    }

    pub fn wallet_type(mut self, value: impl Into<String>) -> Self {
        self.wallet_type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TransferDetailRecord`].
    pub fn build(self) -> Result<TransferDetailRecord, BuildError> {
        Ok(TransferDetailRecord {
            transfer_detail_id: self.transfer_detail_id,
            transfer_id: self.transfer_id,
            transaction_id: self.transaction_id,
            transaction_number: self.transaction_number,
            r#type: self.r#type,
            category: self.category,
            gross_amount: self.gross_amount,
            charge_back_amount: self.charge_back_amount,
            returned_amount: self.returned_amount,
            refund_amount: self.refund_amount,
            hold_amount: self.hold_amount,
            released_amount: self.released_amount,
            billing_fees_amount: self.billing_fees_amount,
            third_party_paid_amount: self.third_party_paid_amount,
            adjustments_amount: self.adjustments_amount,
            net_transfer_amount: self.net_transfer_amount,
            split_funding_amount: self.split_funding_amount,
            card_rejected_amount: self.card_rejected_amount,
            billing_fees_details: self.billing_fees_details,
            parent_org_name: self.parent_org_name,
            paypoint_dbaname: self.paypoint_dbaname,
            paypoint_legalname: self.paypoint_legalname,
            paypoint_entryname: self.paypoint_entryname,
            payment_trans_id: self.payment_trans_id,
            connector_name: self.connector_name,
            external_processor_information: self.external_processor_information,
            gateway_trans_id: self.gateway_trans_id,
            order_id: self.order_id,
            method: self.method,
            batch_number: self.batch_number,
            batch_amount: self.batch_amount,
            payor_id: self.payor_id,
            payment_data: self.payment_data,
            trans_status: self.trans_status,
            paypoint_id: self.paypoint_id,
            total_amount: self.total_amount,
            net_amount: self.net_amount,
            fee_amount: self.fee_amount,
            settlement_status: self.settlement_status,
            operation: self.operation,
            response_data: self.response_data,
            source: self.source,
            schedule_reference: self.schedule_reference,
            org_id: self.org_id,
            refund_id: self.refund_id,
            returned_id: self.returned_id,
            chargeback_id: self.chargeback_id,
            retrieval_id: self.retrieval_id,
            trans_additional_data: self.trans_additional_data,
            invoice_data: self.invoice_data,
            entrypage_id: self.entrypage_id,
            external_paypoint_id: self.external_paypoint_id,
            is_validated_ach: self.is_validated_ach,
            transaction_time: self.transaction_time,
            customer: self.customer,
            split_funding_instructions: self.split_funding_instructions,
            cfee_transactions: self.cfee_transactions,
            transaction_events: self.transaction_events,
            pending_fee_amount: self.pending_fee_amount,
            risk_flagged: self.risk_flagged,
            risk_flagged_on: self.risk_flagged_on,
            risk_status: self.risk_status,
            risk_reason: self.risk_reason,
            risk_action: self.risk_action,
            risk_action_code: self.risk_action_code,
            device_id: self.device_id,
            ach_sec_code: self.ach_sec_code,
            ach_holder_type: self.ach_holder_type,
            ip_address: self.ip_address,
            is_same_day_ach: self.is_same_day_ach,
            wallet_type: self.wallet_type,
        })
    }
}
