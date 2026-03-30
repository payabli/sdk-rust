pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TransactionQueryRecordsCustomer {
    #[serde(rename = "AchHolderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_holder_type: Option<AchHolderType>,
    #[serde(rename = "AchSecCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_sec_code: Option<AchSecCode>,
    /// Batch amount.
    #[serde(rename = "BatchAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_amount: Option<f64>,
    #[serde(rename = "BatchNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_number: Option<BatchNumber>,
    /// Service Fee or sub-charge transaction associated to the main transaction.
    #[serde(rename = "CfeeTransactions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cfee_transactions: Option<Vec<QueryCFeeTransaction>>,
    /// Connector used for transaction.
    #[serde(rename = "ConnectorName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connector_name: Option<String>,
    #[serde(rename = "Customer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<QueryTransactionPayorDataCustomer>,
    #[serde(rename = "DeviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<Device>,
    #[serde(rename = "EntrypageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypage_id: Option<EntrypageId>,
    #[serde(rename = "ExternalProcessorInformation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_processor_information: Option<ExternalProcessorInformation>,
    #[serde(rename = "FeeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_amount: Option<FeeAmount>,
    /// Internal identifier used for processing.
    #[serde(rename = "GatewayTransId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_trans_id: Option<String>,
    #[serde(rename = "invoiceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_data: Option<BillData>,
    /// Payment method used: card, ach, or wallet.
    #[serde(rename = "Method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Net amount paid.
    #[serde(rename = "NetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<Netamountnullable>,
    #[serde(rename = "Operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<Operation>,
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderId>,
    /// ID of immediate parent organization.
    #[serde(rename = "OrgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<Orgid>,
    #[serde(rename = "ParentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<OrgParentName>,
    #[serde(rename = "PaymentData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_data: Option<QueryPaymentData>,
    /// Unique Transaction ID.
    #[serde(rename = "PaymentTransId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_trans_id: Option<String>,
    #[serde(rename = "PayorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payor_id: Option<PayorId>,
    /// Paypoint's DBA name.
    #[serde(rename = "PaypointDbaname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dbaname: Option<Dbaname>,
    /// Paypoint's entryname.
    #[serde(rename = "PaypointEntryname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_entryname: Option<Entrypointfield>,
    /// InternalId for paypoint.
    #[serde(rename = "PaypointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_id: Option<i64>,
    /// Paypoint's legal name.
    #[serde(rename = "PaypointLegalname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legalname: Option<Legalname>,
    #[serde(rename = "PendingFeeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_fee_amount: Option<PendingFeeAmount>,
    #[serde(rename = "RefundId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_id: Option<RefundId>,
    #[serde(rename = "ResponseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<QueryResponseData>,
    #[serde(rename = "ReturnedId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_id: Option<ReturnedId>,
    /// Reference to the subscription that originated the transaction.
    #[serde(rename = "ScheduleReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_reference: Option<i64>,
    /// Settlement status for transaction. See [the docs](/developers/references/money-in-statuses#payment-funding-status) for a full reference.
    #[serde(rename = "SettlementStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_status: Option<i64>,
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    #[serde(rename = "splitFundingInstructions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_funding_instructions: Option<SplitFunding>,
    /// Transaction total amount (including service fee or sub-charge)
    #[serde(rename = "TotalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    /// Events associated with this transaction.
    #[serde(rename = "TransactionEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_events: Option<Vec<QueryTransactionEvents>>,
    /// Transaction date and time, in UTC.
    #[serde(rename = "TransactionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub transaction_time: Option<DateTime<Utc>>,
    #[serde(rename = "TransAdditionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_additional_data: Option<serde_json::Value>,
    /// Status of transaction. See [the docs](/developers/references/money-in-statuses#money-in-transaction-status) for a full reference.
    #[serde(rename = "TransStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_status: Option<i64>,
}

impl TransactionQueryRecordsCustomer {
    pub fn builder() -> TransactionQueryRecordsCustomerBuilder {
        <TransactionQueryRecordsCustomerBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionQueryRecordsCustomerBuilder {
    ach_holder_type: Option<AchHolderType>,
    ach_sec_code: Option<AchSecCode>,
    batch_amount: Option<f64>,
    batch_number: Option<BatchNumber>,
    cfee_transactions: Option<Vec<QueryCFeeTransaction>>,
    connector_name: Option<String>,
    customer: Option<QueryTransactionPayorDataCustomer>,
    device_id: Option<Device>,
    entrypage_id: Option<EntrypageId>,
    external_processor_information: Option<ExternalProcessorInformation>,
    fee_amount: Option<FeeAmount>,
    gateway_trans_id: Option<String>,
    invoice_data: Option<BillData>,
    method: Option<String>,
    net_amount: Option<Netamountnullable>,
    operation: Option<Operation>,
    order_id: Option<OrderId>,
    org_id: Option<Orgid>,
    parent_org_name: Option<OrgParentName>,
    payment_data: Option<QueryPaymentData>,
    payment_trans_id: Option<String>,
    payor_id: Option<PayorId>,
    paypoint_dbaname: Option<Dbaname>,
    paypoint_entryname: Option<Entrypointfield>,
    paypoint_id: Option<i64>,
    paypoint_legalname: Option<Legalname>,
    pending_fee_amount: Option<PendingFeeAmount>,
    refund_id: Option<RefundId>,
    response_data: Option<QueryResponseData>,
    returned_id: Option<ReturnedId>,
    schedule_reference: Option<i64>,
    settlement_status: Option<i64>,
    source: Option<Source>,
    split_funding_instructions: Option<SplitFunding>,
    total_amount: Option<f64>,
    transaction_events: Option<Vec<QueryTransactionEvents>>,
    transaction_time: Option<DateTime<Utc>>,
    trans_additional_data: Option<serde_json::Value>,
    trans_status: Option<i64>,
}

impl TransactionQueryRecordsCustomerBuilder {
    pub fn ach_holder_type(mut self, value: AchHolderType) -> Self {
        self.ach_holder_type = Some(value);
        self
    }

    pub fn ach_sec_code(mut self, value: AchSecCode) -> Self {
        self.ach_sec_code = Some(value);
        self
    }

    pub fn batch_amount(mut self, value: f64) -> Self {
        self.batch_amount = Some(value);
        self
    }

    pub fn batch_number(mut self, value: BatchNumber) -> Self {
        self.batch_number = Some(value);
        self
    }

    pub fn cfee_transactions(mut self, value: Vec<QueryCFeeTransaction>) -> Self {
        self.cfee_transactions = Some(value);
        self
    }

    pub fn connector_name(mut self, value: impl Into<String>) -> Self {
        self.connector_name = Some(value.into());
        self
    }

    pub fn customer(mut self, value: QueryTransactionPayorDataCustomer) -> Self {
        self.customer = Some(value);
        self
    }

    pub fn device_id(mut self, value: Device) -> Self {
        self.device_id = Some(value);
        self
    }

    pub fn entrypage_id(mut self, value: EntrypageId) -> Self {
        self.entrypage_id = Some(value);
        self
    }

    pub fn external_processor_information(mut self, value: ExternalProcessorInformation) -> Self {
        self.external_processor_information = Some(value);
        self
    }

    pub fn fee_amount(mut self, value: FeeAmount) -> Self {
        self.fee_amount = Some(value);
        self
    }

    pub fn gateway_trans_id(mut self, value: impl Into<String>) -> Self {
        self.gateway_trans_id = Some(value.into());
        self
    }

    pub fn invoice_data(mut self, value: BillData) -> Self {
        self.invoice_data = Some(value);
        self
    }

    pub fn method(mut self, value: impl Into<String>) -> Self {
        self.method = Some(value.into());
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

    pub fn order_id(mut self, value: OrderId) -> Self {
        self.order_id = Some(value);
        self
    }

    pub fn org_id(mut self, value: Orgid) -> Self {
        self.org_id = Some(value);
        self
    }

    pub fn parent_org_name(mut self, value: OrgParentName) -> Self {
        self.parent_org_name = Some(value);
        self
    }

    pub fn payment_data(mut self, value: QueryPaymentData) -> Self {
        self.payment_data = Some(value);
        self
    }

    pub fn payment_trans_id(mut self, value: impl Into<String>) -> Self {
        self.payment_trans_id = Some(value.into());
        self
    }

    pub fn payor_id(mut self, value: PayorId) -> Self {
        self.payor_id = Some(value);
        self
    }

    pub fn paypoint_dbaname(mut self, value: Dbaname) -> Self {
        self.paypoint_dbaname = Some(value);
        self
    }

    pub fn paypoint_entryname(mut self, value: Entrypointfield) -> Self {
        self.paypoint_entryname = Some(value);
        self
    }

    pub fn paypoint_id(mut self, value: i64) -> Self {
        self.paypoint_id = Some(value);
        self
    }

    pub fn paypoint_legalname(mut self, value: Legalname) -> Self {
        self.paypoint_legalname = Some(value);
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

    pub fn response_data(mut self, value: QueryResponseData) -> Self {
        self.response_data = Some(value);
        self
    }

    pub fn returned_id(mut self, value: ReturnedId) -> Self {
        self.returned_id = Some(value);
        self
    }

    pub fn schedule_reference(mut self, value: i64) -> Self {
        self.schedule_reference = Some(value);
        self
    }

    pub fn settlement_status(mut self, value: i64) -> Self {
        self.settlement_status = Some(value);
        self
    }

    pub fn source(mut self, value: Source) -> Self {
        self.source = Some(value);
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

    pub fn transaction_events(mut self, value: Vec<QueryTransactionEvents>) -> Self {
        self.transaction_events = Some(value);
        self
    }

    pub fn transaction_time(mut self, value: DateTime<Utc>) -> Self {
        self.transaction_time = Some(value);
        self
    }

    pub fn trans_additional_data(mut self, value: serde_json::Value) -> Self {
        self.trans_additional_data = Some(value);
        self
    }

    pub fn trans_status(mut self, value: i64) -> Self {
        self.trans_status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransactionQueryRecordsCustomer`].
    pub fn build(self) -> Result<TransactionQueryRecordsCustomer, BuildError> {
        Ok(TransactionQueryRecordsCustomer {
            ach_holder_type: self.ach_holder_type,
            ach_sec_code: self.ach_sec_code,
            batch_amount: self.batch_amount,
            batch_number: self.batch_number,
            cfee_transactions: self.cfee_transactions,
            connector_name: self.connector_name,
            customer: self.customer,
            device_id: self.device_id,
            entrypage_id: self.entrypage_id,
            external_processor_information: self.external_processor_information,
            fee_amount: self.fee_amount,
            gateway_trans_id: self.gateway_trans_id,
            invoice_data: self.invoice_data,
            method: self.method,
            net_amount: self.net_amount,
            operation: self.operation,
            order_id: self.order_id,
            org_id: self.org_id,
            parent_org_name: self.parent_org_name,
            payment_data: self.payment_data,
            payment_trans_id: self.payment_trans_id,
            payor_id: self.payor_id,
            paypoint_dbaname: self.paypoint_dbaname,
            paypoint_entryname: self.paypoint_entryname,
            paypoint_id: self.paypoint_id,
            paypoint_legalname: self.paypoint_legalname,
            pending_fee_amount: self.pending_fee_amount,
            refund_id: self.refund_id,
            response_data: self.response_data,
            returned_id: self.returned_id,
            schedule_reference: self.schedule_reference,
            settlement_status: self.settlement_status,
            source: self.source,
            split_funding_instructions: self.split_funding_instructions,
            total_amount: self.total_amount,
            transaction_events: self.transaction_events,
            transaction_time: self.transaction_time,
            trans_additional_data: self.trans_additional_data,
            trans_status: self.trans_status,
        })
    }
}
