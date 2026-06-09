pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ChargebackQueryRecords {
    /// Identifier of chargeback or return.
    #[serde(rename = "Id")]
    #[serde(default)]
    pub id: i64,
    /// Date of chargeback in format YYYY-MM-DD or MM/DD/YYYY.
    #[serde(rename = "ChargebackDate")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub chargeback_date: DateTime<Utc>,
    /// Number of case assigned to the chargeback.
    #[serde(rename = "CaseNumber")]
    #[serde(default)]
    pub case_number: String,
    /// R code for returned ACH or custom code identifying the reason.
    #[serde(rename = "ReasonCode")]
    #[serde(default)]
    pub reason_code: String,
    /// Text describing the chargeback or ACH return reason.
    #[serde(rename = "Reason")]
    #[serde(default)]
    pub reason: String,
    /// Processor reference number to the chargeback.
    #[serde(rename = "ReferenceNumber")]
    #[serde(default)]
    pub reference_number: String,
    /// Last 4 digits of card or bank account involved in chargeback or return.
    #[serde(rename = "LastFour")]
    #[serde(default)]
    pub last_four: String,
    #[serde(rename = "AccountType")]
    #[serde(default)]
    pub account_type: Accounttype,
    /// Status for chargeback or ACH return
    ///
    /// - 0: Open (chargebacks only)
    /// - 1: Pending (chargebacks only)
    /// - 2: Closed-Won (chargebacks only)
    /// - 3: Closed-Lost (chargebacks only)
    /// - 4: ACH Return (ACH only)
    /// - 5: ACH Dispute, Not Authorized (ACH only)
    #[serde(rename = "Status")]
    #[serde(default)]
    pub status: i64,
    /// Type of payment vehicle: **ach** or **card**.
    #[serde(rename = "Method")]
    #[serde(default)]
    pub method: String,
    /// Timestamp when the register was created, in UTC.
    #[serde(rename = "CreatedAt")]
    #[serde(default)]
    pub created_at: CreatedAt,
    #[serde(rename = "ReplyBy")]
    #[serde(default)]
    pub reply_by: Replyby,
    /// ReferenceId of the transaction in Payabli.
    #[serde(rename = "PaymentTransId")]
    #[serde(default)]
    pub payment_trans_id: String,
    /// Reference to the subscription originating the transaction.
    #[serde(rename = "ScheduleReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule_reference: Option<i64>,
    #[serde(rename = "OrderId")]
    #[serde(default)]
    pub order_id: OrderId,
    /// Net amount in chargeback or ACH return.
    #[serde(rename = "NetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<Netamountnullable>,
    #[serde(rename = "TransactionTime")]
    #[serde(default)]
    pub transaction_time: TransactionTime,
    #[serde(rename = "Customer")]
    #[serde(default)]
    pub customer: QueryTransactionPayorData,
    #[serde(rename = "PaymentData")]
    #[serde(default)]
    pub payment_data: QueryPaymentData,
    /// The paypoint's legal name.
    #[serde(rename = "PaypointLegalname")]
    #[serde(default)]
    pub paypoint_legalname: Legalname,
    /// The paypoint's DBA name.
    #[serde(rename = "PaypointDbaname")]
    #[serde(default)]
    pub paypoint_dbaname: Dbaname,
    #[serde(rename = "ParentOrgName")]
    #[serde(default)]
    pub parent_org_name: OrgParentName,
    /// The ID of the parent organization.
    #[serde(rename = "ParentOrgId")]
    #[serde(default)]
    pub parent_org_id: i64,
    /// The paypoint's entryname.
    #[serde(rename = "PaypointEntryname")]
    #[serde(default)]
    pub paypoint_entryname: Entrypointfield,
    /// Chargeback response records.
    #[serde(rename = "Responses")]
    #[serde(default)]
    pub responses: Vec<ChargeBackResponse>,
    #[serde(rename = "Transaction")]
    #[serde(default)]
    pub transaction: TransactionQueryRecords,
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageidentifier: Option<PageIdentifier>,
    /// Messages related to the chargeback.
    #[serde(default)]
    pub messages: Vec<ChargebackMessage>,
    /// Service group classification.
    #[serde(rename = "ServiceGroup")]
    #[serde(default)]
    pub service_group: String,
    /// Type of dispute classification.
    #[serde(rename = "DisputeType")]
    #[serde(default)]
    pub dispute_type: String,
    /// Name of the payment processor.
    #[serde(rename = "ProcessorName")]
    #[serde(default)]
    pub processor_name: String,
}

impl ChargebackQueryRecords {
    pub fn builder() -> ChargebackQueryRecordsBuilder {
        <ChargebackQueryRecordsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ChargebackQueryRecordsBuilder {
    id: Option<i64>,
    chargeback_date: Option<DateTime<Utc>>,
    case_number: Option<String>,
    reason_code: Option<String>,
    reason: Option<String>,
    reference_number: Option<String>,
    last_four: Option<String>,
    account_type: Option<Accounttype>,
    status: Option<i64>,
    method: Option<String>,
    created_at: Option<CreatedAt>,
    reply_by: Option<Replyby>,
    payment_trans_id: Option<String>,
    schedule_reference: Option<i64>,
    order_id: Option<OrderId>,
    net_amount: Option<Netamountnullable>,
    transaction_time: Option<TransactionTime>,
    customer: Option<QueryTransactionPayorData>,
    payment_data: Option<QueryPaymentData>,
    paypoint_legalname: Option<Legalname>,
    paypoint_dbaname: Option<Dbaname>,
    parent_org_name: Option<OrgParentName>,
    parent_org_id: Option<i64>,
    paypoint_entryname: Option<Entrypointfield>,
    responses: Option<Vec<ChargeBackResponse>>,
    transaction: Option<TransactionQueryRecords>,
    external_paypoint_id: Option<ExternalPaypointId>,
    pageidentifier: Option<PageIdentifier>,
    messages: Option<Vec<ChargebackMessage>>,
    service_group: Option<String>,
    dispute_type: Option<String>,
    processor_name: Option<String>,
}

impl ChargebackQueryRecordsBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn chargeback_date(mut self, value: DateTime<Utc>) -> Self {
        self.chargeback_date = Some(value);
        self
    }

    pub fn case_number(mut self, value: impl Into<String>) -> Self {
        self.case_number = Some(value.into());
        self
    }

    pub fn reason_code(mut self, value: impl Into<String>) -> Self {
        self.reason_code = Some(value.into());
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn reference_number(mut self, value: impl Into<String>) -> Self {
        self.reference_number = Some(value.into());
        self
    }

    pub fn last_four(mut self, value: impl Into<String>) -> Self {
        self.last_four = Some(value.into());
        self
    }

    pub fn account_type(mut self, value: Accounttype) -> Self {
        self.account_type = Some(value);
        self
    }

    pub fn status(mut self, value: i64) -> Self {
        self.status = Some(value);
        self
    }

    pub fn method(mut self, value: impl Into<String>) -> Self {
        self.method = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: CreatedAt) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn reply_by(mut self, value: Replyby) -> Self {
        self.reply_by = Some(value);
        self
    }

    pub fn payment_trans_id(mut self, value: impl Into<String>) -> Self {
        self.payment_trans_id = Some(value.into());
        self
    }

    pub fn schedule_reference(mut self, value: i64) -> Self {
        self.schedule_reference = Some(value);
        self
    }

    pub fn order_id(mut self, value: OrderId) -> Self {
        self.order_id = Some(value);
        self
    }

    pub fn net_amount(mut self, value: Netamountnullable) -> Self {
        self.net_amount = Some(value);
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

    pub fn payment_data(mut self, value: QueryPaymentData) -> Self {
        self.payment_data = Some(value);
        self
    }

    pub fn paypoint_legalname(mut self, value: Legalname) -> Self {
        self.paypoint_legalname = Some(value);
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

    pub fn responses(mut self, value: Vec<ChargeBackResponse>) -> Self {
        self.responses = Some(value);
        self
    }

    pub fn transaction(mut self, value: TransactionQueryRecords) -> Self {
        self.transaction = Some(value);
        self
    }

    pub fn external_paypoint_id(mut self, value: ExternalPaypointId) -> Self {
        self.external_paypoint_id = Some(value);
        self
    }

    pub fn pageidentifier(mut self, value: PageIdentifier) -> Self {
        self.pageidentifier = Some(value);
        self
    }

    pub fn messages(mut self, value: Vec<ChargebackMessage>) -> Self {
        self.messages = Some(value);
        self
    }

    pub fn service_group(mut self, value: impl Into<String>) -> Self {
        self.service_group = Some(value.into());
        self
    }

    pub fn dispute_type(mut self, value: impl Into<String>) -> Self {
        self.dispute_type = Some(value.into());
        self
    }

    pub fn processor_name(mut self, value: impl Into<String>) -> Self {
        self.processor_name = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ChargebackQueryRecords`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](ChargebackQueryRecordsBuilder::id)
    /// - [`chargeback_date`](ChargebackQueryRecordsBuilder::chargeback_date)
    /// - [`case_number`](ChargebackQueryRecordsBuilder::case_number)
    /// - [`reason_code`](ChargebackQueryRecordsBuilder::reason_code)
    /// - [`reason`](ChargebackQueryRecordsBuilder::reason)
    /// - [`reference_number`](ChargebackQueryRecordsBuilder::reference_number)
    /// - [`last_four`](ChargebackQueryRecordsBuilder::last_four)
    /// - [`account_type`](ChargebackQueryRecordsBuilder::account_type)
    /// - [`status`](ChargebackQueryRecordsBuilder::status)
    /// - [`method`](ChargebackQueryRecordsBuilder::method)
    /// - [`created_at`](ChargebackQueryRecordsBuilder::created_at)
    /// - [`reply_by`](ChargebackQueryRecordsBuilder::reply_by)
    /// - [`payment_trans_id`](ChargebackQueryRecordsBuilder::payment_trans_id)
    /// - [`order_id`](ChargebackQueryRecordsBuilder::order_id)
    /// - [`transaction_time`](ChargebackQueryRecordsBuilder::transaction_time)
    /// - [`customer`](ChargebackQueryRecordsBuilder::customer)
    /// - [`payment_data`](ChargebackQueryRecordsBuilder::payment_data)
    /// - [`paypoint_legalname`](ChargebackQueryRecordsBuilder::paypoint_legalname)
    /// - [`paypoint_dbaname`](ChargebackQueryRecordsBuilder::paypoint_dbaname)
    /// - [`parent_org_name`](ChargebackQueryRecordsBuilder::parent_org_name)
    /// - [`parent_org_id`](ChargebackQueryRecordsBuilder::parent_org_id)
    /// - [`paypoint_entryname`](ChargebackQueryRecordsBuilder::paypoint_entryname)
    /// - [`responses`](ChargebackQueryRecordsBuilder::responses)
    /// - [`transaction`](ChargebackQueryRecordsBuilder::transaction)
    /// - [`messages`](ChargebackQueryRecordsBuilder::messages)
    /// - [`service_group`](ChargebackQueryRecordsBuilder::service_group)
    /// - [`dispute_type`](ChargebackQueryRecordsBuilder::dispute_type)
    /// - [`processor_name`](ChargebackQueryRecordsBuilder::processor_name)
    pub fn build(self) -> Result<ChargebackQueryRecords, BuildError> {
        Ok(ChargebackQueryRecords {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            chargeback_date: self
                .chargeback_date
                .ok_or_else(|| BuildError::missing_field("chargeback_date"))?,
            case_number: self
                .case_number
                .ok_or_else(|| BuildError::missing_field("case_number"))?,
            reason_code: self
                .reason_code
                .ok_or_else(|| BuildError::missing_field("reason_code"))?,
            reason: self
                .reason
                .ok_or_else(|| BuildError::missing_field("reason"))?,
            reference_number: self
                .reference_number
                .ok_or_else(|| BuildError::missing_field("reference_number"))?,
            last_four: self
                .last_four
                .ok_or_else(|| BuildError::missing_field("last_four"))?,
            account_type: self
                .account_type
                .ok_or_else(|| BuildError::missing_field("account_type"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            reply_by: self
                .reply_by
                .ok_or_else(|| BuildError::missing_field("reply_by"))?,
            payment_trans_id: self
                .payment_trans_id
                .ok_or_else(|| BuildError::missing_field("payment_trans_id"))?,
            schedule_reference: self.schedule_reference,
            order_id: self
                .order_id
                .ok_or_else(|| BuildError::missing_field("order_id"))?,
            net_amount: self.net_amount,
            transaction_time: self
                .transaction_time
                .ok_or_else(|| BuildError::missing_field("transaction_time"))?,
            customer: self
                .customer
                .ok_or_else(|| BuildError::missing_field("customer"))?,
            payment_data: self
                .payment_data
                .ok_or_else(|| BuildError::missing_field("payment_data"))?,
            paypoint_legalname: self
                .paypoint_legalname
                .ok_or_else(|| BuildError::missing_field("paypoint_legalname"))?,
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
            responses: self
                .responses
                .ok_or_else(|| BuildError::missing_field("responses"))?,
            transaction: self
                .transaction
                .ok_or_else(|| BuildError::missing_field("transaction"))?,
            external_paypoint_id: self.external_paypoint_id,
            pageidentifier: self.pageidentifier,
            messages: self
                .messages
                .ok_or_else(|| BuildError::missing_field("messages"))?,
            service_group: self
                .service_group
                .ok_or_else(|| BuildError::missing_field("service_group"))?,
            dispute_type: self
                .dispute_type
                .ok_or_else(|| BuildError::missing_field("dispute_type"))?,
            processor_name: self
                .processor_name
                .ok_or_else(|| BuildError::missing_field("processor_name"))?,
        })
    }
}
