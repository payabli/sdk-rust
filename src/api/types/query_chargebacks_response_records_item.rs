pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryChargebacksResponseRecordsItem {
    /// Type of account.
    #[serde(rename = "AccountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// Case number of the chargeback.
    #[serde(rename = "CaseNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub case_number: Option<String>,
    /// Date of the chargeback.
    #[serde(rename = "ChargebackDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub chargeback_date: Option<DateTime<Utc>>,
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    #[serde(rename = "Customer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<QueryTransactionPayorData>,
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    /// Unique identifier of the record.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Last four digits of the account number.
    #[serde(rename = "LastFour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_four: Option<String>,
    /// Method of payment.
    #[serde(rename = "Method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Net amount after deductions.
    #[serde(rename = "NetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<f64>,
    #[serde(rename = "OrderId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<OrderId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageidentifier: Option<PageIdentifier>,
    #[serde(rename = "ParentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<OrgParentName>,
    /// Payment data associated with the transaction.
    #[serde(rename = "PaymentData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_data: Option<QueryPaymentData>,
    /// Transaction ID for the payment.
    #[serde(rename = "PaymentTransId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_trans_id: Option<String>,
    /// The 'Doing Business As' (DBA) name of the paypoint.
    #[serde(rename = "PaypointDbaname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dbaname: Option<Dbaname>,
    /// Entryname for the paypoint.
    #[serde(rename = "PaypointEntryname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_entryname: Option<Entrypointfield>,
    /// Legal name of the paypoint.
    #[serde(rename = "PaypointLegalname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legalname: Option<Legalname>,
    /// Description of the reason for chargeback.
    #[serde(rename = "Reason")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// Code representing the reason for chargeback.
    #[serde(rename = "ReasonCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_code: Option<String>,
    /// Reference number for the transaction.
    #[serde(rename = "ReferenceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_number: Option<String>,
    #[serde(rename = "ReplyBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_by: Option<Replyby>,
    /// Responses related to the transaction.
    #[serde(rename = "Responses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<String>,
    /// Reference for any scheduled transactions.
    #[serde(rename = "ScheduleReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_reference: Option<i64>,
    /// Status of the transaction.
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    #[serde(rename = "Transaction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<TransactionQueryRecords>,
    #[serde(rename = "TransactionTime")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_time: Option<TransactionTime>,
}

impl QueryChargebacksResponseRecordsItem {
    pub fn builder() -> QueryChargebacksResponseRecordsItemBuilder {
        <QueryChargebacksResponseRecordsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryChargebacksResponseRecordsItemBuilder {
    account_type: Option<String>,
    case_number: Option<String>,
    chargeback_date: Option<DateTime<Utc>>,
    created_at: Option<CreatedAt>,
    customer: Option<QueryTransactionPayorData>,
    external_paypoint_id: Option<ExternalPaypointId>,
    id: Option<i64>,
    last_four: Option<String>,
    method: Option<String>,
    net_amount: Option<f64>,
    order_id: Option<OrderId>,
    pageidentifier: Option<PageIdentifier>,
    parent_org_name: Option<OrgParentName>,
    payment_data: Option<QueryPaymentData>,
    payment_trans_id: Option<String>,
    paypoint_dbaname: Option<Dbaname>,
    paypoint_entryname: Option<Entrypointfield>,
    paypoint_legalname: Option<Legalname>,
    reason: Option<String>,
    reason_code: Option<String>,
    reference_number: Option<String>,
    reply_by: Option<Replyby>,
    responses: Option<String>,
    schedule_reference: Option<i64>,
    status: Option<i64>,
    transaction: Option<TransactionQueryRecords>,
    transaction_time: Option<TransactionTime>,
}

impl QueryChargebacksResponseRecordsItemBuilder {
    pub fn account_type(mut self, value: impl Into<String>) -> Self {
        self.account_type = Some(value.into());
        self
    }

    pub fn case_number(mut self, value: impl Into<String>) -> Self {
        self.case_number = Some(value.into());
        self
    }

    pub fn chargeback_date(mut self, value: DateTime<Utc>) -> Self {
        self.chargeback_date = Some(value);
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

    pub fn external_paypoint_id(mut self, value: ExternalPaypointId) -> Self {
        self.external_paypoint_id = Some(value);
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn last_four(mut self, value: impl Into<String>) -> Self {
        self.last_four = Some(value.into());
        self
    }

    pub fn method(mut self, value: impl Into<String>) -> Self {
        self.method = Some(value.into());
        self
    }

    pub fn net_amount(mut self, value: f64) -> Self {
        self.net_amount = Some(value);
        self
    }

    pub fn order_id(mut self, value: OrderId) -> Self {
        self.order_id = Some(value);
        self
    }

    pub fn pageidentifier(mut self, value: PageIdentifier) -> Self {
        self.pageidentifier = Some(value);
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

    pub fn paypoint_dbaname(mut self, value: Dbaname) -> Self {
        self.paypoint_dbaname = Some(value);
        self
    }

    pub fn paypoint_entryname(mut self, value: Entrypointfield) -> Self {
        self.paypoint_entryname = Some(value);
        self
    }

    pub fn paypoint_legalname(mut self, value: Legalname) -> Self {
        self.paypoint_legalname = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn reason_code(mut self, value: impl Into<String>) -> Self {
        self.reason_code = Some(value.into());
        self
    }

    pub fn reference_number(mut self, value: impl Into<String>) -> Self {
        self.reference_number = Some(value.into());
        self
    }

    pub fn reply_by(mut self, value: Replyby) -> Self {
        self.reply_by = Some(value);
        self
    }

    pub fn responses(mut self, value: impl Into<String>) -> Self {
        self.responses = Some(value.into());
        self
    }

    pub fn schedule_reference(mut self, value: i64) -> Self {
        self.schedule_reference = Some(value);
        self
    }

    pub fn status(mut self, value: i64) -> Self {
        self.status = Some(value);
        self
    }

    pub fn transaction(mut self, value: TransactionQueryRecords) -> Self {
        self.transaction = Some(value);
        self
    }

    pub fn transaction_time(mut self, value: TransactionTime) -> Self {
        self.transaction_time = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryChargebacksResponseRecordsItem`].
    pub fn build(self) -> Result<QueryChargebacksResponseRecordsItem, BuildError> {
        Ok(QueryChargebacksResponseRecordsItem {
            account_type: self.account_type,
            case_number: self.case_number,
            chargeback_date: self.chargeback_date,
            created_at: self.created_at,
            customer: self.customer,
            external_paypoint_id: self.external_paypoint_id,
            id: self.id,
            last_four: self.last_four,
            method: self.method,
            net_amount: self.net_amount,
            order_id: self.order_id,
            pageidentifier: self.pageidentifier,
            parent_org_name: self.parent_org_name,
            payment_data: self.payment_data,
            payment_trans_id: self.payment_trans_id,
            paypoint_dbaname: self.paypoint_dbaname,
            paypoint_entryname: self.paypoint_entryname,
            paypoint_legalname: self.paypoint_legalname,
            reason: self.reason,
            reason_code: self.reason_code,
            reference_number: self.reference_number,
            reply_by: self.reply_by,
            responses: self.responses,
            schedule_reference: self.schedule_reference,
            status: self.status,
            transaction: self.transaction,
            transaction_time: self.transaction_time,
        })
    }
}
