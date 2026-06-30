pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryResponseSettlementsRecordsItem {
    /// The batch amount.
    #[serde(rename = "BatchAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub batch_amount: Option<f64>,
    #[serde(rename = "BatchNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_number: Option<BatchNumber>,
    #[serde(rename = "Category")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    #[serde(rename = "Customer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<QueryTransactionPayorData>,
    #[serde(rename = "DepositDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_date: Option<DepositDate>,
    #[serde(rename = "ExpectedDepositDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_deposit_date: Option<ExpectedDepositDate>,
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    /// Internal identifier used for processing.
    #[serde(rename = "GatewayTransId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_trans_id: Option<String>,
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "invoiceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_data: Option<BillData>,
    /// Describes whether the transaction is being held or not.
    /// 1 - Transaction is held
    /// 0 - Transaction isn't being held
    #[serde(rename = "isHold")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_hold: Option<i64>,
    #[serde(rename = "MaskedAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masked_account: Option<Maskedaccount>,
    /// The payment method.
    #[serde(rename = "Method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Net amount paid.
    #[serde(rename = "NetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<Netamountnullable>,
    /// The operation performed.
    #[serde(rename = "Operation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderId>,
    #[serde(rename = "ParentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<OrgParentName>,
    #[serde(rename = "PaymentData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_data: Option<QueryPaymentData>,
    /// The transaction ID for the payment.
    #[serde(rename = "PaymentTransId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_trans_id: Option<String>,
    #[serde(rename = "PaymentTransStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_trans_status: Option<TransStatus>,
    /// Paypoint DBA name.
    #[serde(rename = "PaypointDbaname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dbaname: Option<String>,
    /// Paypoint entryname.
    #[serde(rename = "PaypointEntryname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_entryname: Option<String>,
    /// Paypoint legal name.
    #[serde(rename = "PaypointLegalname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legalname: Option<String>,
    #[serde(rename = "ResponseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<QueryResponseData>,
    /// Reference to the subscription originating the transaction.
    #[serde(rename = "ScheduleReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_reference: Option<i64>,
    /// The transaction amount.
    #[serde(rename = "SettledAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub settled_amount: Option<f64>,
    /// The date and time when the transaction was settled. This field is null when the transaction's `SettlementStatus` is -1, -5, or -6 (Exception, Held, or Released).
    #[serde(rename = "SettlementDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub settlement_date: Option<DateTime<Utc>>,
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    /// Split funding instructions for the settled transaction, each enriched with the batch and transfer that paid out the split. Null when the transaction has no splits.
    #[serde(rename = "splitFundingInstructions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub split_funding_instructions: Option<Vec<SettlementSplitFundingDetail>>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<SettlementStatus>,
    /// Events associated with this transaction.
    #[serde(rename = "TransactionEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_events: Option<Vec<QueryTransactionEvents>>,
    #[serde(rename = "TransactionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_time: Option<TransactionTime>,
    /// Payment method used: card or ach.
    #[serde(rename = "TransMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_method: Option<String>,
    /// The transaction type: credit or debit.
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl QueryResponseSettlementsRecordsItem {
    pub fn builder() -> QueryResponseSettlementsRecordsItemBuilder {
        <QueryResponseSettlementsRecordsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryResponseSettlementsRecordsItemBuilder {
    batch_amount: Option<f64>,
    batch_number: Option<BatchNumber>,
    category: Option<Category>,
    created_at: Option<CreatedAt>,
    customer: Option<QueryTransactionPayorData>,
    deposit_date: Option<DepositDate>,
    expected_deposit_date: Option<ExpectedDepositDate>,
    external_paypoint_id: Option<ExternalPaypointId>,
    gateway_trans_id: Option<String>,
    id: Option<i64>,
    invoice_data: Option<BillData>,
    is_hold: Option<i64>,
    masked_account: Option<Maskedaccount>,
    method: Option<String>,
    net_amount: Option<Netamountnullable>,
    operation: Option<String>,
    order_id: Option<OrderId>,
    parent_org_name: Option<OrgParentName>,
    payment_data: Option<QueryPaymentData>,
    payment_trans_id: Option<String>,
    payment_trans_status: Option<TransStatus>,
    paypoint_dbaname: Option<String>,
    paypoint_entryname: Option<String>,
    paypoint_legalname: Option<String>,
    response_data: Option<QueryResponseData>,
    schedule_reference: Option<i64>,
    settled_amount: Option<f64>,
    settlement_date: Option<DateTime<Utc>>,
    source: Option<Source>,
    split_funding_instructions: Option<Vec<SettlementSplitFundingDetail>>,
    status: Option<SettlementStatus>,
    transaction_events: Option<Vec<QueryTransactionEvents>>,
    transaction_time: Option<TransactionTime>,
    trans_method: Option<String>,
    r#type: Option<String>,
}

impl QueryResponseSettlementsRecordsItemBuilder {
    pub fn batch_amount(mut self, value: f64) -> Self {
        self.batch_amount = Some(value);
        self
    }

    pub fn batch_number(mut self, value: BatchNumber) -> Self {
        self.batch_number = Some(value);
        self
    }

    pub fn category(mut self, value: Category) -> Self {
        self.category = Some(value);
        self
    }

    pub fn created_at(mut self, value: CreatedAt) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn customer(mut self, value: QueryTransactionPayorData) -> Self {
        self.customer = Some(value);
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

    pub fn external_paypoint_id(mut self, value: ExternalPaypointId) -> Self {
        self.external_paypoint_id = Some(value);
        self
    }

    pub fn gateway_trans_id(mut self, value: impl Into<String>) -> Self {
        self.gateway_trans_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn invoice_data(mut self, value: BillData) -> Self {
        self.invoice_data = Some(value);
        self
    }

    pub fn is_hold(mut self, value: i64) -> Self {
        self.is_hold = Some(value);
        self
    }

    pub fn masked_account(mut self, value: Maskedaccount) -> Self {
        self.masked_account = Some(value);
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

    pub fn operation(mut self, value: impl Into<String>) -> Self {
        self.operation = Some(value.into());
        self
    }

    pub fn order_id(mut self, value: OrderId) -> Self {
        self.order_id = Some(value);
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

    pub fn payment_trans_status(mut self, value: TransStatus) -> Self {
        self.payment_trans_status = Some(value);
        self
    }

    pub fn paypoint_dbaname(mut self, value: impl Into<String>) -> Self {
        self.paypoint_dbaname = Some(value.into());
        self
    }

    pub fn paypoint_entryname(mut self, value: impl Into<String>) -> Self {
        self.paypoint_entryname = Some(value.into());
        self
    }

    pub fn paypoint_legalname(mut self, value: impl Into<String>) -> Self {
        self.paypoint_legalname = Some(value.into());
        self
    }

    pub fn response_data(mut self, value: QueryResponseData) -> Self {
        self.response_data = Some(value);
        self
    }

    pub fn schedule_reference(mut self, value: i64) -> Self {
        self.schedule_reference = Some(value);
        self
    }

    pub fn settled_amount(mut self, value: f64) -> Self {
        self.settled_amount = Some(value);
        self
    }

    pub fn settlement_date(mut self, value: DateTime<Utc>) -> Self {
        self.settlement_date = Some(value);
        self
    }

    pub fn source(mut self, value: Source) -> Self {
        self.source = Some(value);
        self
    }

    pub fn split_funding_instructions(mut self, value: Vec<SettlementSplitFundingDetail>) -> Self {
        self.split_funding_instructions = Some(value);
        self
    }

    pub fn status(mut self, value: SettlementStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn transaction_events(mut self, value: Vec<QueryTransactionEvents>) -> Self {
        self.transaction_events = Some(value);
        self
    }

    pub fn transaction_time(mut self, value: TransactionTime) -> Self {
        self.transaction_time = Some(value);
        self
    }

    pub fn trans_method(mut self, value: impl Into<String>) -> Self {
        self.trans_method = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryResponseSettlementsRecordsItem`].
    pub fn build(self) -> Result<QueryResponseSettlementsRecordsItem, BuildError> {
        Ok(QueryResponseSettlementsRecordsItem {
            batch_amount: self.batch_amount,
            batch_number: self.batch_number,
            category: self.category,
            created_at: self.created_at,
            customer: self.customer,
            deposit_date: self.deposit_date,
            expected_deposit_date: self.expected_deposit_date,
            external_paypoint_id: self.external_paypoint_id,
            gateway_trans_id: self.gateway_trans_id,
            id: self.id,
            invoice_data: self.invoice_data,
            is_hold: self.is_hold,
            masked_account: self.masked_account,
            method: self.method,
            net_amount: self.net_amount,
            operation: self.operation,
            order_id: self.order_id,
            parent_org_name: self.parent_org_name,
            payment_data: self.payment_data,
            payment_trans_id: self.payment_trans_id,
            payment_trans_status: self.payment_trans_status,
            paypoint_dbaname: self.paypoint_dbaname,
            paypoint_entryname: self.paypoint_entryname,
            paypoint_legalname: self.paypoint_legalname,
            response_data: self.response_data,
            schedule_reference: self.schedule_reference,
            settled_amount: self.settled_amount,
            settlement_date: self.settlement_date,
            source: self.source,
            split_funding_instructions: self.split_funding_instructions,
            status: self.status,
            transaction_events: self.transaction_events,
            transaction_time: self.transaction_time,
            trans_method: self.trans_method,
            r#type: self.r#type,
        })
    }
}
