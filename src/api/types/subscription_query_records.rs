pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SubscriptionQueryRecords {
    /// Timestamp of when the subscription ws created, in UTC.
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    #[serde(rename = "Customer")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<QueryTransactionPayorData>,
    /// The subscription's end date.
    #[serde(rename = "EndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub end_date: Option<DateTime<Utc>>,
    #[serde(rename = "EntrypageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypage_id: Option<EntrypageId>,
    #[serde(rename = "ExternalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    /// Fee applied to the subscription.
    #[serde(rename = "FeeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_amount: Option<f64>,
    /// The subscription's frequency.
    #[serde(rename = "Frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    /// The subscription's ID.
    #[serde(rename = "IdSub")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_sub: Option<i64>,
    #[serde(rename = "invoiceData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_data: Option<BillData>,
    /// The last time the subscription was processed.
    #[serde(rename = "LastRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub last_run: Option<DateTime<Utc>>,
    /// The last date and time the subscription was updated.
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<LastModified>,
    /// The number of cycles the subscription has left.
    #[serde(rename = "LeftCycles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_cycles: Option<i64>,
    /// The subscription's payment method.
    #[serde(rename = "Method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// The subscription amount, minus any fees.
    #[serde(rename = "NetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<Netamountnullable>,
    /// The next date the subscription will be processed.
    #[serde(rename = "NextDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub next_date: Option<DateTime<Utc>>,
    #[serde(rename = "ParentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<OrgParentName>,
    #[serde(rename = "PaymentData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_data: Option<QueryPaymentData>,
    /// The paypoint's DBA name.
    #[serde(rename = "PaypointDbaname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dbaname: Option<Dbaname>,
    /// The paypoint's entryname.
    #[serde(rename = "PaypointEntryname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_entryname: Option<Entrypointfield>,
    #[serde(rename = "PaypointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_id: Option<PaypointId>,
    /// The paypoint's legal name.
    #[serde(rename = "PaypointLegalname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legalname: Option<Legalname>,
    /// Payment plan ID.
    #[serde(rename = "PlanId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<i64>,
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
    /// The subscription start date.
    #[serde(rename = "StartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub start_date: Option<DateTime<Utc>>,
    /// The full stored payment method record linked to the subscription
    /// and charged on each billing cycle. Returned as `null` for legacy
    /// subscriptions that don't have a linked stored method.
    ///
    /// The shape is the same across payment vehicles (card, ACH, check).
    /// Only the populated fields differ. For example, `ABA` is populated
    /// for ACH, while `ExpDate` and `binData` are populated for card.
    #[serde(rename = "StoredMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_method: Option<VendorResponseStoredMethod>,
    /// Events associated with the subscription.
    #[serde(rename = "SubEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_events: Option<Vec<GeneralEvents>>,
    /// The subscription's status.
    /// - 0: Paused
    /// - 1: Active
    #[serde(rename = "SubStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_status: Option<i64>,
    /// The subscription amount, including any fees.
    #[serde(rename = "TotalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    /// The total number of cycles the subscription is set to run.
    #[serde(rename = "TotalCycles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cycles: Option<i64>,
    /// When `true`, the subscription has no explicit end date and will run until canceled.
    #[serde(rename = "UntilCancelled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_cancelled: Option<bool>,
}

impl SubscriptionQueryRecords {
    pub fn builder() -> SubscriptionQueryRecordsBuilder {
        <SubscriptionQueryRecordsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubscriptionQueryRecordsBuilder {
    created_at: Option<CreatedAt>,
    customer: Option<QueryTransactionPayorData>,
    end_date: Option<DateTime<Utc>>,
    entrypage_id: Option<EntrypageId>,
    external_paypoint_id: Option<ExternalPaypointId>,
    fee_amount: Option<f64>,
    frequency: Option<String>,
    id_sub: Option<i64>,
    invoice_data: Option<BillData>,
    last_run: Option<DateTime<Utc>>,
    last_updated: Option<LastModified>,
    left_cycles: Option<i64>,
    method: Option<String>,
    net_amount: Option<Netamountnullable>,
    next_date: Option<DateTime<Utc>>,
    parent_org_name: Option<OrgParentName>,
    payment_data: Option<QueryPaymentData>,
    paypoint_dbaname: Option<Dbaname>,
    paypoint_entryname: Option<Entrypointfield>,
    paypoint_id: Option<PaypointId>,
    paypoint_legalname: Option<Legalname>,
    plan_id: Option<i64>,
    source: Option<Source>,
    start_date: Option<DateTime<Utc>>,
    stored_method: Option<VendorResponseStoredMethod>,
    sub_events: Option<Vec<GeneralEvents>>,
    sub_status: Option<i64>,
    total_amount: Option<f64>,
    total_cycles: Option<i64>,
    until_cancelled: Option<bool>,
}

impl SubscriptionQueryRecordsBuilder {
    pub fn created_at(mut self, value: CreatedAt) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn customer(mut self, value: QueryTransactionPayorData) -> Self {
        self.customer = Some(value);
        self
    }

    pub fn end_date(mut self, value: DateTime<Utc>) -> Self {
        self.end_date = Some(value);
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

    pub fn fee_amount(mut self, value: f64) -> Self {
        self.fee_amount = Some(value);
        self
    }

    pub fn frequency(mut self, value: impl Into<String>) -> Self {
        self.frequency = Some(value.into());
        self
    }

    pub fn id_sub(mut self, value: i64) -> Self {
        self.id_sub = Some(value);
        self
    }

    pub fn invoice_data(mut self, value: BillData) -> Self {
        self.invoice_data = Some(value);
        self
    }

    pub fn last_run(mut self, value: DateTime<Utc>) -> Self {
        self.last_run = Some(value);
        self
    }

    pub fn last_updated(mut self, value: LastModified) -> Self {
        self.last_updated = Some(value);
        self
    }

    pub fn left_cycles(mut self, value: i64) -> Self {
        self.left_cycles = Some(value);
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

    pub fn next_date(mut self, value: DateTime<Utc>) -> Self {
        self.next_date = Some(value);
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

    pub fn paypoint_dbaname(mut self, value: Dbaname) -> Self {
        self.paypoint_dbaname = Some(value);
        self
    }

    pub fn paypoint_entryname(mut self, value: Entrypointfield) -> Self {
        self.paypoint_entryname = Some(value);
        self
    }

    pub fn paypoint_id(mut self, value: PaypointId) -> Self {
        self.paypoint_id = Some(value);
        self
    }

    pub fn paypoint_legalname(mut self, value: Legalname) -> Self {
        self.paypoint_legalname = Some(value);
        self
    }

    pub fn plan_id(mut self, value: i64) -> Self {
        self.plan_id = Some(value);
        self
    }

    pub fn source(mut self, value: Source) -> Self {
        self.source = Some(value);
        self
    }

    pub fn start_date(mut self, value: DateTime<Utc>) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn stored_method(mut self, value: VendorResponseStoredMethod) -> Self {
        self.stored_method = Some(value);
        self
    }

    pub fn sub_events(mut self, value: Vec<GeneralEvents>) -> Self {
        self.sub_events = Some(value);
        self
    }

    pub fn sub_status(mut self, value: i64) -> Self {
        self.sub_status = Some(value);
        self
    }

    pub fn total_amount(mut self, value: f64) -> Self {
        self.total_amount = Some(value);
        self
    }

    pub fn total_cycles(mut self, value: i64) -> Self {
        self.total_cycles = Some(value);
        self
    }

    pub fn until_cancelled(mut self, value: bool) -> Self {
        self.until_cancelled = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubscriptionQueryRecords`].
    pub fn build(self) -> Result<SubscriptionQueryRecords, BuildError> {
        Ok(SubscriptionQueryRecords {
            created_at: self.created_at,
            customer: self.customer,
            end_date: self.end_date,
            entrypage_id: self.entrypage_id,
            external_paypoint_id: self.external_paypoint_id,
            fee_amount: self.fee_amount,
            frequency: self.frequency,
            id_sub: self.id_sub,
            invoice_data: self.invoice_data,
            last_run: self.last_run,
            last_updated: self.last_updated,
            left_cycles: self.left_cycles,
            method: self.method,
            net_amount: self.net_amount,
            next_date: self.next_date,
            parent_org_name: self.parent_org_name,
            payment_data: self.payment_data,
            paypoint_dbaname: self.paypoint_dbaname,
            paypoint_entryname: self.paypoint_entryname,
            paypoint_id: self.paypoint_id,
            paypoint_legalname: self.paypoint_legalname,
            plan_id: self.plan_id,
            source: self.source,
            start_date: self.start_date,
            stored_method: self.stored_method,
            sub_events: self.sub_events,
            sub_status: self.sub_status,
            total_amount: self.total_amount,
            total_cycles: self.total_cycles,
            until_cancelled: self.until_cancelled,
        })
    }
}
