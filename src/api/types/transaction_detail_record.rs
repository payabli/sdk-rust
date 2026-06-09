pub use crate::prelude::*;

/// Complete transaction details including payment information, customer data, and processing metadata. This is returned when includeDetails=true.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionDetailRecord {
    #[serde(rename = "parentOrgName")]
    #[serde(default)]
    pub parent_org_name: OrgParentName,
    #[serde(rename = "paypointDbaname")]
    #[serde(default)]
    pub paypoint_dbaname: Dbaname,
    #[serde(rename = "paypointLegalname")]
    #[serde(default)]
    pub paypoint_legalname: Legalname,
    #[serde(rename = "paypointEntryname")]
    #[serde(default)]
    pub paypoint_entryname: Entrypointfield,
    #[serde(rename = "paymentTransId")]
    #[serde(default)]
    pub payment_trans_id: String,
    #[serde(rename = "connectorName")]
    #[serde(default)]
    pub connector_name: String,
    #[serde(rename = "externalProcessorInformation")]
    #[serde(default)]
    pub external_processor_information: ExternalProcessorInformation,
    #[serde(rename = "gatewayTransId")]
    #[serde(default)]
    pub gateway_trans_id: String,
    #[serde(rename = "orderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderId>,
    pub method: TransactionDetailRecordMethod,
    #[serde(rename = "batchNumber")]
    #[serde(default)]
    pub batch_number: BatchNumber,
    #[serde(rename = "batchAmount")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub batch_amount: f64,
    #[serde(rename = "payorId")]
    #[serde(default)]
    pub payor_id: PayorId,
    #[serde(rename = "paymentData")]
    #[serde(default)]
    pub payment_data: TransactionDetailPaymentData,
    #[serde(rename = "transStatus")]
    #[serde(default)]
    pub trans_status: TransStatus,
    #[serde(rename = "paypointId")]
    #[serde(default)]
    pub paypoint_id: PaypointId,
    #[serde(rename = "totalAmount")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_amount: f64,
    #[serde(rename = "netAmount")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub net_amount: f64,
    #[serde(rename = "feeAmount")]
    #[serde(default)]
    pub fee_amount: FeeAmount,
    #[serde(rename = "settlementStatus")]
    #[serde(default)]
    pub settlement_status: SettlementStatus,
    #[serde(default)]
    pub operation: Operation,
    #[serde(rename = "responseData")]
    #[serde(default)]
    pub response_data: TransactionDetailResponseData,
    #[serde(default)]
    pub source: Source,
    #[serde(rename = "scheduleReference")]
    #[serde(default)]
    pub schedule_reference: i64,
    #[serde(rename = "orgId")]
    #[serde(default)]
    pub org_id: Orgid,
    #[serde(rename = "refundId")]
    #[serde(default)]
    pub refund_id: RefundId,
    #[serde(rename = "returnedId")]
    #[serde(default)]
    pub returned_id: ReturnedId,
    #[serde(rename = "chargebackId")]
    #[serde(default)]
    pub chargeback_id: ChargebackId,
    #[serde(rename = "retrievalId")]
    #[serde(default)]
    pub retrieval_id: RetrievalId,
    #[serde(rename = "transAdditionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_additional_data: Option<serde_json::Value>,
    #[serde(rename = "invoiceData")]
    #[serde(default)]
    pub invoice_data: TransactionDetailInvoiceData,
    #[serde(rename = "entrypageId")]
    #[serde(default)]
    pub entrypage_id: EntrypageId,
    #[serde(rename = "externalPaypointID")]
    #[serde(default)]
    pub external_paypoint_id: ExternalPaypointId,
    #[serde(rename = "isValidatedACH")]
    #[serde(default)]
    pub is_validated_ach: bool,
    #[serde(rename = "transactionTime")]
    #[serde(default)]
    pub transaction_time: String,
    #[serde(default)]
    pub customer: TransactionDetailCustomer,
    #[serde(rename = "splitFundingInstructions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_funding_instructions: Option<SplitFunding>,
    #[serde(rename = "cfeeTransactions")]
    #[serde(default)]
    pub cfee_transactions: Vec<QueryCFeeTransaction>,
    #[serde(rename = "transactionEvents")]
    #[serde(default)]
    pub transaction_events: Vec<TransactionDetailEvent>,
    #[serde(rename = "pendingFeeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_fee_amount: Option<PendingFeeAmount>,
    #[serde(rename = "riskFlagged")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_flagged: Option<RiskFlagged>,
    #[serde(rename = "riskFlaggedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_flagged_on: Option<RiskFlaggedOn>,
    #[serde(rename = "riskStatus")]
    #[serde(default)]
    pub risk_status: RiskStatus,
    #[serde(rename = "riskReason")]
    #[serde(default)]
    pub risk_reason: RiskReason,
    #[serde(rename = "riskAction")]
    #[serde(default)]
    pub risk_action: RiskAction,
    #[serde(rename = "riskActionCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_action_code: Option<RiskActionCode>,
    #[serde(rename = "deviceId")]
    #[serde(default)]
    pub device_id: Device,
    #[serde(rename = "achSecCode")]
    #[serde(default)]
    pub ach_sec_code: AchSecCode,
    #[serde(rename = "achHolderType")]
    pub ach_holder_type: AchHolderType,
    #[serde(rename = "ipAddress")]
    #[serde(default)]
    pub ip_address: IpAddress,
    #[serde(rename = "isSameDayACH")]
    #[serde(default)]
    pub is_same_day_ach: bool,
    #[serde(rename = "walletType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_type: Option<String>,
}

impl TransactionDetailRecord {
    pub fn builder() -> TransactionDetailRecordBuilder {
        <TransactionDetailRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionDetailRecordBuilder {
    parent_org_name: Option<OrgParentName>,
    paypoint_dbaname: Option<Dbaname>,
    paypoint_legalname: Option<Legalname>,
    paypoint_entryname: Option<Entrypointfield>,
    payment_trans_id: Option<String>,
    connector_name: Option<String>,
    external_processor_information: Option<ExternalProcessorInformation>,
    gateway_trans_id: Option<String>,
    order_id: Option<OrderId>,
    method: Option<TransactionDetailRecordMethod>,
    batch_number: Option<BatchNumber>,
    batch_amount: Option<f64>,
    payor_id: Option<PayorId>,
    payment_data: Option<TransactionDetailPaymentData>,
    trans_status: Option<TransStatus>,
    paypoint_id: Option<PaypointId>,
    total_amount: Option<f64>,
    net_amount: Option<f64>,
    fee_amount: Option<FeeAmount>,
    settlement_status: Option<SettlementStatus>,
    operation: Option<Operation>,
    response_data: Option<TransactionDetailResponseData>,
    source: Option<Source>,
    schedule_reference: Option<i64>,
    org_id: Option<Orgid>,
    refund_id: Option<RefundId>,
    returned_id: Option<ReturnedId>,
    chargeback_id: Option<ChargebackId>,
    retrieval_id: Option<RetrievalId>,
    trans_additional_data: Option<serde_json::Value>,
    invoice_data: Option<TransactionDetailInvoiceData>,
    entrypage_id: Option<EntrypageId>,
    external_paypoint_id: Option<ExternalPaypointId>,
    is_validated_ach: Option<bool>,
    transaction_time: Option<String>,
    customer: Option<TransactionDetailCustomer>,
    split_funding_instructions: Option<SplitFunding>,
    cfee_transactions: Option<Vec<QueryCFeeTransaction>>,
    transaction_events: Option<Vec<TransactionDetailEvent>>,
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

impl TransactionDetailRecordBuilder {
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

    pub fn paypoint_entryname(mut self, value: Entrypointfield) -> Self {
        self.paypoint_entryname = Some(value);
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

    pub fn method(mut self, value: TransactionDetailRecordMethod) -> Self {
        self.method = Some(value);
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

    pub fn payment_data(mut self, value: TransactionDetailPaymentData) -> Self {
        self.payment_data = Some(value);
        self
    }

    pub fn trans_status(mut self, value: TransStatus) -> Self {
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

    pub fn net_amount(mut self, value: f64) -> Self {
        self.net_amount = Some(value);
        self
    }

    pub fn fee_amount(mut self, value: FeeAmount) -> Self {
        self.fee_amount = Some(value);
        self
    }

    pub fn settlement_status(mut self, value: SettlementStatus) -> Self {
        self.settlement_status = Some(value);
        self
    }

    pub fn operation(mut self, value: Operation) -> Self {
        self.operation = Some(value);
        self
    }

    pub fn response_data(mut self, value: TransactionDetailResponseData) -> Self {
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

    pub fn invoice_data(mut self, value: TransactionDetailInvoiceData) -> Self {
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

    pub fn transaction_time(mut self, value: impl Into<String>) -> Self {
        self.transaction_time = Some(value.into());
        self
    }

    pub fn customer(mut self, value: TransactionDetailCustomer) -> Self {
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

    pub fn transaction_events(mut self, value: Vec<TransactionDetailEvent>) -> Self {
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

    /// Consumes the builder and constructs a [`TransactionDetailRecord`].
    /// This method will fail if any of the following fields are not set:
    /// - [`parent_org_name`](TransactionDetailRecordBuilder::parent_org_name)
    /// - [`paypoint_dbaname`](TransactionDetailRecordBuilder::paypoint_dbaname)
    /// - [`paypoint_legalname`](TransactionDetailRecordBuilder::paypoint_legalname)
    /// - [`paypoint_entryname`](TransactionDetailRecordBuilder::paypoint_entryname)
    /// - [`payment_trans_id`](TransactionDetailRecordBuilder::payment_trans_id)
    /// - [`connector_name`](TransactionDetailRecordBuilder::connector_name)
    /// - [`external_processor_information`](TransactionDetailRecordBuilder::external_processor_information)
    /// - [`gateway_trans_id`](TransactionDetailRecordBuilder::gateway_trans_id)
    /// - [`method`](TransactionDetailRecordBuilder::method)
    /// - [`batch_number`](TransactionDetailRecordBuilder::batch_number)
    /// - [`batch_amount`](TransactionDetailRecordBuilder::batch_amount)
    /// - [`payor_id`](TransactionDetailRecordBuilder::payor_id)
    /// - [`payment_data`](TransactionDetailRecordBuilder::payment_data)
    /// - [`trans_status`](TransactionDetailRecordBuilder::trans_status)
    /// - [`paypoint_id`](TransactionDetailRecordBuilder::paypoint_id)
    /// - [`total_amount`](TransactionDetailRecordBuilder::total_amount)
    /// - [`net_amount`](TransactionDetailRecordBuilder::net_amount)
    /// - [`fee_amount`](TransactionDetailRecordBuilder::fee_amount)
    /// - [`settlement_status`](TransactionDetailRecordBuilder::settlement_status)
    /// - [`operation`](TransactionDetailRecordBuilder::operation)
    /// - [`response_data`](TransactionDetailRecordBuilder::response_data)
    /// - [`source`](TransactionDetailRecordBuilder::source)
    /// - [`schedule_reference`](TransactionDetailRecordBuilder::schedule_reference)
    /// - [`org_id`](TransactionDetailRecordBuilder::org_id)
    /// - [`refund_id`](TransactionDetailRecordBuilder::refund_id)
    /// - [`returned_id`](TransactionDetailRecordBuilder::returned_id)
    /// - [`chargeback_id`](TransactionDetailRecordBuilder::chargeback_id)
    /// - [`retrieval_id`](TransactionDetailRecordBuilder::retrieval_id)
    /// - [`invoice_data`](TransactionDetailRecordBuilder::invoice_data)
    /// - [`entrypage_id`](TransactionDetailRecordBuilder::entrypage_id)
    /// - [`external_paypoint_id`](TransactionDetailRecordBuilder::external_paypoint_id)
    /// - [`is_validated_ach`](TransactionDetailRecordBuilder::is_validated_ach)
    /// - [`transaction_time`](TransactionDetailRecordBuilder::transaction_time)
    /// - [`customer`](TransactionDetailRecordBuilder::customer)
    /// - [`cfee_transactions`](TransactionDetailRecordBuilder::cfee_transactions)
    /// - [`transaction_events`](TransactionDetailRecordBuilder::transaction_events)
    /// - [`risk_status`](TransactionDetailRecordBuilder::risk_status)
    /// - [`risk_reason`](TransactionDetailRecordBuilder::risk_reason)
    /// - [`risk_action`](TransactionDetailRecordBuilder::risk_action)
    /// - [`device_id`](TransactionDetailRecordBuilder::device_id)
    /// - [`ach_sec_code`](TransactionDetailRecordBuilder::ach_sec_code)
    /// - [`ach_holder_type`](TransactionDetailRecordBuilder::ach_holder_type)
    /// - [`ip_address`](TransactionDetailRecordBuilder::ip_address)
    /// - [`is_same_day_ach`](TransactionDetailRecordBuilder::is_same_day_ach)
    pub fn build(self) -> Result<TransactionDetailRecord, BuildError> {
        Ok(TransactionDetailRecord {
            parent_org_name: self
                .parent_org_name
                .ok_or_else(|| BuildError::missing_field("parent_org_name"))?,
            paypoint_dbaname: self
                .paypoint_dbaname
                .ok_or_else(|| BuildError::missing_field("paypoint_dbaname"))?,
            paypoint_legalname: self
                .paypoint_legalname
                .ok_or_else(|| BuildError::missing_field("paypoint_legalname"))?,
            paypoint_entryname: self
                .paypoint_entryname
                .ok_or_else(|| BuildError::missing_field("paypoint_entryname"))?,
            payment_trans_id: self
                .payment_trans_id
                .ok_or_else(|| BuildError::missing_field("payment_trans_id"))?,
            connector_name: self
                .connector_name
                .ok_or_else(|| BuildError::missing_field("connector_name"))?,
            external_processor_information: self
                .external_processor_information
                .ok_or_else(|| BuildError::missing_field("external_processor_information"))?,
            gateway_trans_id: self
                .gateway_trans_id
                .ok_or_else(|| BuildError::missing_field("gateway_trans_id"))?,
            order_id: self.order_id,
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
            batch_number: self
                .batch_number
                .ok_or_else(|| BuildError::missing_field("batch_number"))?,
            batch_amount: self
                .batch_amount
                .ok_or_else(|| BuildError::missing_field("batch_amount"))?,
            payor_id: self
                .payor_id
                .ok_or_else(|| BuildError::missing_field("payor_id"))?,
            payment_data: self
                .payment_data
                .ok_or_else(|| BuildError::missing_field("payment_data"))?,
            trans_status: self
                .trans_status
                .ok_or_else(|| BuildError::missing_field("trans_status"))?,
            paypoint_id: self
                .paypoint_id
                .ok_or_else(|| BuildError::missing_field("paypoint_id"))?,
            total_amount: self
                .total_amount
                .ok_or_else(|| BuildError::missing_field("total_amount"))?,
            net_amount: self
                .net_amount
                .ok_or_else(|| BuildError::missing_field("net_amount"))?,
            fee_amount: self
                .fee_amount
                .ok_or_else(|| BuildError::missing_field("fee_amount"))?,
            settlement_status: self
                .settlement_status
                .ok_or_else(|| BuildError::missing_field("settlement_status"))?,
            operation: self
                .operation
                .ok_or_else(|| BuildError::missing_field("operation"))?,
            response_data: self
                .response_data
                .ok_or_else(|| BuildError::missing_field("response_data"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            schedule_reference: self
                .schedule_reference
                .ok_or_else(|| BuildError::missing_field("schedule_reference"))?,
            org_id: self
                .org_id
                .ok_or_else(|| BuildError::missing_field("org_id"))?,
            refund_id: self
                .refund_id
                .ok_or_else(|| BuildError::missing_field("refund_id"))?,
            returned_id: self
                .returned_id
                .ok_or_else(|| BuildError::missing_field("returned_id"))?,
            chargeback_id: self
                .chargeback_id
                .ok_or_else(|| BuildError::missing_field("chargeback_id"))?,
            retrieval_id: self
                .retrieval_id
                .ok_or_else(|| BuildError::missing_field("retrieval_id"))?,
            trans_additional_data: self.trans_additional_data,
            invoice_data: self
                .invoice_data
                .ok_or_else(|| BuildError::missing_field("invoice_data"))?,
            entrypage_id: self
                .entrypage_id
                .ok_or_else(|| BuildError::missing_field("entrypage_id"))?,
            external_paypoint_id: self
                .external_paypoint_id
                .ok_or_else(|| BuildError::missing_field("external_paypoint_id"))?,
            is_validated_ach: self
                .is_validated_ach
                .ok_or_else(|| BuildError::missing_field("is_validated_ach"))?,
            transaction_time: self
                .transaction_time
                .ok_or_else(|| BuildError::missing_field("transaction_time"))?,
            customer: self
                .customer
                .ok_or_else(|| BuildError::missing_field("customer"))?,
            split_funding_instructions: self.split_funding_instructions,
            cfee_transactions: self
                .cfee_transactions
                .ok_or_else(|| BuildError::missing_field("cfee_transactions"))?,
            transaction_events: self
                .transaction_events
                .ok_or_else(|| BuildError::missing_field("transaction_events"))?,
            pending_fee_amount: self.pending_fee_amount,
            risk_flagged: self.risk_flagged,
            risk_flagged_on: self.risk_flagged_on,
            risk_status: self
                .risk_status
                .ok_or_else(|| BuildError::missing_field("risk_status"))?,
            risk_reason: self
                .risk_reason
                .ok_or_else(|| BuildError::missing_field("risk_reason"))?,
            risk_action: self
                .risk_action
                .ok_or_else(|| BuildError::missing_field("risk_action"))?,
            risk_action_code: self.risk_action_code,
            device_id: self
                .device_id
                .ok_or_else(|| BuildError::missing_field("device_id"))?,
            ach_sec_code: self
                .ach_sec_code
                .ok_or_else(|| BuildError::missing_field("ach_sec_code"))?,
            ach_holder_type: self
                .ach_holder_type
                .ok_or_else(|| BuildError::missing_field("ach_holder_type"))?,
            ip_address: self
                .ip_address
                .ok_or_else(|| BuildError::missing_field("ip_address"))?,
            is_same_day_ach: self
                .is_same_day_ach
                .ok_or_else(|| BuildError::missing_field("is_same_day_ach"))?,
            wallet_type: self.wallet_type,
        })
    }
}
