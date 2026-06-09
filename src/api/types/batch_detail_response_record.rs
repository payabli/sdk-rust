pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchDetailResponseRecord {
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: i64,
    #[serde(rename = "Method")]
    #[serde(default)]
    pub method: String,
    #[serde(rename = "WalletType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_type: Option<String>,
    #[serde(rename = "SettledAmount")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub settled_amount: f64,
    #[serde(rename = "Type")]
    #[serde(default)]
    pub r#type: String,
    #[serde(rename = "BatchNumber")]
    #[serde(default)]
    pub batch_number: BatchNumber,
    #[serde(rename = "BatchAmount")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub batch_amount: f64,
    #[serde(rename = "PaymentTransId")]
    #[serde(default)]
    pub payment_trans_id: String,
    #[serde(rename = "PaymentTransStatus")]
    #[serde(default)]
    pub payment_trans_status: i64,
    #[serde(rename = "ScheduleReference")]
    #[serde(default)]
    pub schedule_reference: i64,
    #[serde(rename = "GatewayTransId")]
    #[serde(default)]
    pub gateway_trans_id: String,
    #[serde(rename = "OrderId")]
    #[serde(default)]
    pub order_id: OrderId,
    #[serde(rename = "TransMethod")]
    #[serde(default)]
    pub trans_method: String,
    #[serde(rename = "PaymentData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_data: Option<QueryPaymentData>,
    #[serde(rename = "NetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<Netamountnullable>,
    #[serde(rename = "Operation")]
    #[serde(default)]
    pub operation: Operation,
    #[serde(rename = "Category")]
    #[serde(default)]
    pub category: Category,
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: i64,
    #[serde(rename = "TransactionTime")]
    #[serde(default)]
    pub transaction_time: TransactionTime,
    #[serde(rename = "Customer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<QueryTransactionPayorData>,
    #[serde(rename = "SettlementDate")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub settlement_date: DateTime<Utc>,
    #[serde(rename = "PaymentSettlementStatus")]
    #[serde(default)]
    pub payment_settlement_status: i64,
    #[serde(rename = "BatchStatus")]
    #[serde(default)]
    pub batch_status: i64,
    #[serde(rename = "DepositDate")]
    #[serde(default)]
    pub deposit_date: DepositDate,
    #[serde(rename = "ExpectedDepositDate")]
    #[serde(default)]
    pub expected_deposit_date: ExpectedDepositDate,
    #[serde(rename = "MaskedAccount")]
    #[serde(default)]
    pub masked_account: Maskedaccount,
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    pub created_at: CreatedAt,
    #[serde(rename = "PaypointLegalname")]
    #[serde(default)]
    pub paypoint_legalname: Legalname,
    #[serde(rename = "ResponseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<QueryResponseData>,
    #[serde(rename = "PaypointDbaname")]
    #[serde(default)]
    pub paypoint_dbaname: Dbaname,
    #[serde(rename = "ParentOrgName")]
    #[serde(default)]
    pub parent_org_name: OrgParentName,
    #[serde(rename = "ParentOrgId")]
    #[serde(default)]
    pub parent_org_id: i64,
    #[serde(rename = "PaypointEntryname")]
    #[serde(default)]
    pub paypoint_entryname: Entrypointfield,
    #[serde(rename = "DeviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<Device>,
    #[serde(rename = "RetrievalId")]
    #[serde(default)]
    pub retrieval_id: RetrievalId,
    #[serde(rename = "ChargebackId")]
    #[serde(default)]
    pub chargeback_id: ChargebackId,
    #[serde(rename = "AchHolderType")]
    pub ach_holder_type: AchHolderType,
    #[serde(rename = "AchSecCode")]
    #[serde(default)]
    pub ach_sec_code: AchSecCode,
    #[serde(rename = "ConnectorName")]
    #[serde(default)]
    pub connector_name: String,
    #[serde(rename = "EntrypageId")]
    #[serde(default)]
    pub entrypage_id: EntrypageId,
    #[serde(rename = "FeeAmount")]
    #[serde(default)]
    pub fee_amount: FeeAmount,
    #[serde(rename = "OrgId")]
    #[serde(default)]
    pub org_id: Orgid,
    #[serde(rename = "PayorId")]
    #[serde(default)]
    pub payor_id: PayorId,
    #[serde(rename = "PaypointId")]
    #[serde(default)]
    pub paypoint_id: PaypointId,
    #[serde(rename = "PendingFeeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_fee_amount: Option<PendingFeeAmount>,
    #[serde(rename = "RefundId")]
    #[serde(default)]
    pub refund_id: RefundId,
    #[serde(rename = "ReturnedId")]
    #[serde(default)]
    pub returned_id: ReturnedId,
    #[serde(rename = "splitFundingInstructions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_funding_instructions: Option<SplitFunding>,
    #[serde(rename = "TotalAmount")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub total_amount: f64,
    #[serde(rename = "CfeeTransactions")]
    #[serde(default)]
    pub cfee_transactions: Vec<QueryCFeeTransaction>,
    #[serde(rename = "invoiceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_data: Option<BillData>,
    #[serde(rename = "TransactionEvents")]
    #[serde(default)]
    pub transaction_events: Vec<QueryTransactionEvents>,
    #[serde(rename = "externalPaypointID")]
    #[serde(default)]
    pub external_paypoint_id: ExternalPaypointId,
    #[serde(rename = "isHold")]
    #[serde(default)]
    pub is_hold: i64,
}

impl BatchDetailResponseRecord {
    pub fn builder() -> BatchDetailResponseRecordBuilder {
        <BatchDetailResponseRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BatchDetailResponseRecordBuilder {
    id: Option<i64>,
    method: Option<String>,
    wallet_type: Option<String>,
    settled_amount: Option<f64>,
    r#type: Option<String>,
    batch_number: Option<BatchNumber>,
    batch_amount: Option<f64>,
    payment_trans_id: Option<String>,
    payment_trans_status: Option<i64>,
    schedule_reference: Option<i64>,
    gateway_trans_id: Option<String>,
    order_id: Option<OrderId>,
    trans_method: Option<String>,
    payment_data: Option<QueryPaymentData>,
    net_amount: Option<Netamountnullable>,
    operation: Option<Operation>,
    category: Option<Category>,
    source: Option<Source>,
    status: Option<i64>,
    transaction_time: Option<TransactionTime>,
    customer: Option<QueryTransactionPayorData>,
    settlement_date: Option<DateTime<Utc>>,
    payment_settlement_status: Option<i64>,
    batch_status: Option<i64>,
    deposit_date: Option<DepositDate>,
    expected_deposit_date: Option<ExpectedDepositDate>,
    masked_account: Option<Maskedaccount>,
    created_at: Option<CreatedAt>,
    paypoint_legalname: Option<Legalname>,
    response_data: Option<QueryResponseData>,
    paypoint_dbaname: Option<Dbaname>,
    parent_org_name: Option<OrgParentName>,
    parent_org_id: Option<i64>,
    paypoint_entryname: Option<Entrypointfield>,
    device_id: Option<Device>,
    retrieval_id: Option<RetrievalId>,
    chargeback_id: Option<ChargebackId>,
    ach_holder_type: Option<AchHolderType>,
    ach_sec_code: Option<AchSecCode>,
    connector_name: Option<String>,
    entrypage_id: Option<EntrypageId>,
    fee_amount: Option<FeeAmount>,
    org_id: Option<Orgid>,
    payor_id: Option<PayorId>,
    paypoint_id: Option<PaypointId>,
    pending_fee_amount: Option<PendingFeeAmount>,
    refund_id: Option<RefundId>,
    returned_id: Option<ReturnedId>,
    split_funding_instructions: Option<SplitFunding>,
    total_amount: Option<f64>,
    cfee_transactions: Option<Vec<QueryCFeeTransaction>>,
    invoice_data: Option<BillData>,
    transaction_events: Option<Vec<QueryTransactionEvents>>,
    external_paypoint_id: Option<ExternalPaypointId>,
    is_hold: Option<i64>,
}

impl BatchDetailResponseRecordBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn method(mut self, value: impl Into<String>) -> Self {
        self.method = Some(value.into());
        self
    }

    pub fn wallet_type(mut self, value: impl Into<String>) -> Self {
        self.wallet_type = Some(value.into());
        self
    }

    pub fn settled_amount(mut self, value: f64) -> Self {
        self.settled_amount = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
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

    pub fn payment_trans_id(mut self, value: impl Into<String>) -> Self {
        self.payment_trans_id = Some(value.into());
        self
    }

    pub fn payment_trans_status(mut self, value: i64) -> Self {
        self.payment_trans_status = Some(value);
        self
    }

    pub fn schedule_reference(mut self, value: i64) -> Self {
        self.schedule_reference = Some(value);
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

    pub fn trans_method(mut self, value: impl Into<String>) -> Self {
        self.trans_method = Some(value.into());
        self
    }

    pub fn payment_data(mut self, value: QueryPaymentData) -> Self {
        self.payment_data = Some(value);
        self
    }

    pub fn net_amount(mut self, value: Netamountnullable) -> Self {
        self.net_amount = Some(value);
        self
    }

    pub fn operation(mut self, value: Operation) -> Self {
        self.operation = Some(value);
        self
    }

    pub fn category(mut self, value: Category) -> Self {
        self.category = Some(value);
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

    pub fn transaction_time(mut self, value: TransactionTime) -> Self {
        self.transaction_time = Some(value);
        self
    }

    pub fn customer(mut self, value: QueryTransactionPayorData) -> Self {
        self.customer = Some(value);
        self
    }

    pub fn settlement_date(mut self, value: DateTime<Utc>) -> Self {
        self.settlement_date = Some(value);
        self
    }

    pub fn payment_settlement_status(mut self, value: i64) -> Self {
        self.payment_settlement_status = Some(value);
        self
    }

    pub fn batch_status(mut self, value: i64) -> Self {
        self.batch_status = Some(value);
        self
    }

    pub fn deposit_date(mut self, value: DepositDate) -> Self {
        self.deposit_date = Some(value);
        self
    }

    pub fn expected_deposit_date(mut self, value: ExpectedDepositDate) -> Self {
        self.expected_deposit_date = Some(value);
        self
    }

    pub fn masked_account(mut self, value: Maskedaccount) -> Self {
        self.masked_account = Some(value);
        self
    }

    pub fn created_at(mut self, value: CreatedAt) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn paypoint_legalname(mut self, value: Legalname) -> Self {
        self.paypoint_legalname = Some(value);
        self
    }

    pub fn response_data(mut self, value: QueryResponseData) -> Self {
        self.response_data = Some(value);
        self
    }

    pub fn paypoint_dbaname(mut self, value: Dbaname) -> Self {
        self.paypoint_dbaname = Some(value);
        self
    }

    pub fn parent_org_name(mut self, value: OrgParentName) -> Self {
        self.parent_org_name = Some(value);
        self
    }

    pub fn parent_org_id(mut self, value: i64) -> Self {
        self.parent_org_id = Some(value);
        self
    }

    pub fn paypoint_entryname(mut self, value: Entrypointfield) -> Self {
        self.paypoint_entryname = Some(value);
        self
    }

    pub fn device_id(mut self, value: Device) -> Self {
        self.device_id = Some(value);
        self
    }

    pub fn retrieval_id(mut self, value: RetrievalId) -> Self {
        self.retrieval_id = Some(value);
        self
    }

    pub fn chargeback_id(mut self, value: ChargebackId) -> Self {
        self.chargeback_id = Some(value);
        self
    }

    pub fn ach_holder_type(mut self, value: AchHolderType) -> Self {
        self.ach_holder_type = Some(value);
        self
    }

    pub fn ach_sec_code(mut self, value: AchSecCode) -> Self {
        self.ach_sec_code = Some(value);
        self
    }

    pub fn connector_name(mut self, value: impl Into<String>) -> Self {
        self.connector_name = Some(value.into());
        self
    }

    pub fn entrypage_id(mut self, value: EntrypageId) -> Self {
        self.entrypage_id = Some(value);
        self
    }

    pub fn fee_amount(mut self, value: FeeAmount) -> Self {
        self.fee_amount = Some(value);
        self
    }

    pub fn org_id(mut self, value: Orgid) -> Self {
        self.org_id = Some(value);
        self
    }

    pub fn payor_id(mut self, value: PayorId) -> Self {
        self.payor_id = Some(value);
        self
    }

    pub fn paypoint_id(mut self, value: PaypointId) -> Self {
        self.paypoint_id = Some(value);
        self
    }

    pub fn pending_fee_amount(mut self, value: PendingFeeAmount) -> Self {
        self.pending_fee_amount = Some(value);
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

    pub fn split_funding_instructions(mut self, value: SplitFunding) -> Self {
        self.split_funding_instructions = Some(value);
        self
    }

    pub fn total_amount(mut self, value: f64) -> Self {
        self.total_amount = Some(value);
        self
    }

    pub fn cfee_transactions(mut self, value: Vec<QueryCFeeTransaction>) -> Self {
        self.cfee_transactions = Some(value);
        self
    }

    pub fn invoice_data(mut self, value: BillData) -> Self {
        self.invoice_data = Some(value);
        self
    }

    pub fn transaction_events(mut self, value: Vec<QueryTransactionEvents>) -> Self {
        self.transaction_events = Some(value);
        self
    }

    pub fn external_paypoint_id(mut self, value: ExternalPaypointId) -> Self {
        self.external_paypoint_id = Some(value);
        self
    }

    pub fn is_hold(mut self, value: i64) -> Self {
        self.is_hold = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BatchDetailResponseRecord`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](BatchDetailResponseRecordBuilder::id)
    /// - [`method`](BatchDetailResponseRecordBuilder::method)
    /// - [`settled_amount`](BatchDetailResponseRecordBuilder::settled_amount)
    /// - [`r#type`](BatchDetailResponseRecordBuilder::r#type)
    /// - [`batch_number`](BatchDetailResponseRecordBuilder::batch_number)
    /// - [`batch_amount`](BatchDetailResponseRecordBuilder::batch_amount)
    /// - [`payment_trans_id`](BatchDetailResponseRecordBuilder::payment_trans_id)
    /// - [`payment_trans_status`](BatchDetailResponseRecordBuilder::payment_trans_status)
    /// - [`schedule_reference`](BatchDetailResponseRecordBuilder::schedule_reference)
    /// - [`gateway_trans_id`](BatchDetailResponseRecordBuilder::gateway_trans_id)
    /// - [`order_id`](BatchDetailResponseRecordBuilder::order_id)
    /// - [`trans_method`](BatchDetailResponseRecordBuilder::trans_method)
    /// - [`operation`](BatchDetailResponseRecordBuilder::operation)
    /// - [`category`](BatchDetailResponseRecordBuilder::category)
    /// - [`status`](BatchDetailResponseRecordBuilder::status)
    /// - [`transaction_time`](BatchDetailResponseRecordBuilder::transaction_time)
    /// - [`settlement_date`](BatchDetailResponseRecordBuilder::settlement_date)
    /// - [`payment_settlement_status`](BatchDetailResponseRecordBuilder::payment_settlement_status)
    /// - [`batch_status`](BatchDetailResponseRecordBuilder::batch_status)
    /// - [`deposit_date`](BatchDetailResponseRecordBuilder::deposit_date)
    /// - [`expected_deposit_date`](BatchDetailResponseRecordBuilder::expected_deposit_date)
    /// - [`masked_account`](BatchDetailResponseRecordBuilder::masked_account)
    /// - [`created_at`](BatchDetailResponseRecordBuilder::created_at)
    /// - [`paypoint_legalname`](BatchDetailResponseRecordBuilder::paypoint_legalname)
    /// - [`paypoint_dbaname`](BatchDetailResponseRecordBuilder::paypoint_dbaname)
    /// - [`parent_org_name`](BatchDetailResponseRecordBuilder::parent_org_name)
    /// - [`parent_org_id`](BatchDetailResponseRecordBuilder::parent_org_id)
    /// - [`paypoint_entryname`](BatchDetailResponseRecordBuilder::paypoint_entryname)
    /// - [`retrieval_id`](BatchDetailResponseRecordBuilder::retrieval_id)
    /// - [`chargeback_id`](BatchDetailResponseRecordBuilder::chargeback_id)
    /// - [`ach_holder_type`](BatchDetailResponseRecordBuilder::ach_holder_type)
    /// - [`ach_sec_code`](BatchDetailResponseRecordBuilder::ach_sec_code)
    /// - [`connector_name`](BatchDetailResponseRecordBuilder::connector_name)
    /// - [`entrypage_id`](BatchDetailResponseRecordBuilder::entrypage_id)
    /// - [`fee_amount`](BatchDetailResponseRecordBuilder::fee_amount)
    /// - [`org_id`](BatchDetailResponseRecordBuilder::org_id)
    /// - [`payor_id`](BatchDetailResponseRecordBuilder::payor_id)
    /// - [`paypoint_id`](BatchDetailResponseRecordBuilder::paypoint_id)
    /// - [`refund_id`](BatchDetailResponseRecordBuilder::refund_id)
    /// - [`returned_id`](BatchDetailResponseRecordBuilder::returned_id)
    /// - [`total_amount`](BatchDetailResponseRecordBuilder::total_amount)
    /// - [`cfee_transactions`](BatchDetailResponseRecordBuilder::cfee_transactions)
    /// - [`transaction_events`](BatchDetailResponseRecordBuilder::transaction_events)
    /// - [`external_paypoint_id`](BatchDetailResponseRecordBuilder::external_paypoint_id)
    /// - [`is_hold`](BatchDetailResponseRecordBuilder::is_hold)
    pub fn build(self) -> Result<BatchDetailResponseRecord, BuildError> {
        Ok(BatchDetailResponseRecord {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
            wallet_type: self.wallet_type,
            settled_amount: self
                .settled_amount
                .ok_or_else(|| BuildError::missing_field("settled_amount"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            batch_number: self
                .batch_number
                .ok_or_else(|| BuildError::missing_field("batch_number"))?,
            batch_amount: self
                .batch_amount
                .ok_or_else(|| BuildError::missing_field("batch_amount"))?,
            payment_trans_id: self
                .payment_trans_id
                .ok_or_else(|| BuildError::missing_field("payment_trans_id"))?,
            payment_trans_status: self
                .payment_trans_status
                .ok_or_else(|| BuildError::missing_field("payment_trans_status"))?,
            schedule_reference: self
                .schedule_reference
                .ok_or_else(|| BuildError::missing_field("schedule_reference"))?,
            gateway_trans_id: self
                .gateway_trans_id
                .ok_or_else(|| BuildError::missing_field("gateway_trans_id"))?,
            order_id: self
                .order_id
                .ok_or_else(|| BuildError::missing_field("order_id"))?,
            trans_method: self
                .trans_method
                .ok_or_else(|| BuildError::missing_field("trans_method"))?,
            payment_data: self.payment_data,
            net_amount: self.net_amount,
            operation: self
                .operation
                .ok_or_else(|| BuildError::missing_field("operation"))?,
            category: self
                .category
                .ok_or_else(|| BuildError::missing_field("category"))?,
            source: self.source,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            transaction_time: self
                .transaction_time
                .ok_or_else(|| BuildError::missing_field("transaction_time"))?,
            customer: self.customer,
            settlement_date: self
                .settlement_date
                .ok_or_else(|| BuildError::missing_field("settlement_date"))?,
            payment_settlement_status: self
                .payment_settlement_status
                .ok_or_else(|| BuildError::missing_field("payment_settlement_status"))?,
            batch_status: self
                .batch_status
                .ok_or_else(|| BuildError::missing_field("batch_status"))?,
            deposit_date: self
                .deposit_date
                .ok_or_else(|| BuildError::missing_field("deposit_date"))?,
            expected_deposit_date: self
                .expected_deposit_date
                .ok_or_else(|| BuildError::missing_field("expected_deposit_date"))?,
            masked_account: self
                .masked_account
                .ok_or_else(|| BuildError::missing_field("masked_account"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            paypoint_legalname: self
                .paypoint_legalname
                .ok_or_else(|| BuildError::missing_field("paypoint_legalname"))?,
            response_data: self.response_data,
            paypoint_dbaname: self
                .paypoint_dbaname
                .ok_or_else(|| BuildError::missing_field("paypoint_dbaname"))?,
            parent_org_name: self
                .parent_org_name
                .ok_or_else(|| BuildError::missing_field("parent_org_name"))?,
            parent_org_id: self
                .parent_org_id
                .ok_or_else(|| BuildError::missing_field("parent_org_id"))?,
            paypoint_entryname: self
                .paypoint_entryname
                .ok_or_else(|| BuildError::missing_field("paypoint_entryname"))?,
            device_id: self.device_id,
            retrieval_id: self
                .retrieval_id
                .ok_or_else(|| BuildError::missing_field("retrieval_id"))?,
            chargeback_id: self
                .chargeback_id
                .ok_or_else(|| BuildError::missing_field("chargeback_id"))?,
            ach_holder_type: self
                .ach_holder_type
                .ok_or_else(|| BuildError::missing_field("ach_holder_type"))?,
            ach_sec_code: self
                .ach_sec_code
                .ok_or_else(|| BuildError::missing_field("ach_sec_code"))?,
            connector_name: self
                .connector_name
                .ok_or_else(|| BuildError::missing_field("connector_name"))?,
            entrypage_id: self
                .entrypage_id
                .ok_or_else(|| BuildError::missing_field("entrypage_id"))?,
            fee_amount: self
                .fee_amount
                .ok_or_else(|| BuildError::missing_field("fee_amount"))?,
            org_id: self
                .org_id
                .ok_or_else(|| BuildError::missing_field("org_id"))?,
            payor_id: self
                .payor_id
                .ok_or_else(|| BuildError::missing_field("payor_id"))?,
            paypoint_id: self
                .paypoint_id
                .ok_or_else(|| BuildError::missing_field("paypoint_id"))?,
            pending_fee_amount: self.pending_fee_amount,
            refund_id: self
                .refund_id
                .ok_or_else(|| BuildError::missing_field("refund_id"))?,
            returned_id: self
                .returned_id
                .ok_or_else(|| BuildError::missing_field("returned_id"))?,
            split_funding_instructions: self.split_funding_instructions,
            total_amount: self
                .total_amount
                .ok_or_else(|| BuildError::missing_field("total_amount"))?,
            cfee_transactions: self
                .cfee_transactions
                .ok_or_else(|| BuildError::missing_field("cfee_transactions"))?,
            invoice_data: self.invoice_data,
            transaction_events: self
                .transaction_events
                .ok_or_else(|| BuildError::missing_field("transaction_events"))?,
            external_paypoint_id: self
                .external_paypoint_id
                .ok_or_else(|| BuildError::missing_field("external_paypoint_id"))?,
            is_hold: self
                .is_hold
                .ok_or_else(|| BuildError::missing_field("is_hold"))?,
        })
    }
}
