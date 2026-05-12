pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PayoutSubscriptionQueryRecordPascal {
    /// The payout subscription's ID.
    #[serde(rename = "IdOutSubscription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_out_subscription: Option<i64>,
    /// The payout subscription's status.
    /// - 0: Paused
    /// - 1: Active
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    /// Events associated with the payout subscription.
    #[serde(rename = "Events")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub events: Option<Vec<GeneralEvents>>,
    #[serde(rename = "Vendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<VendorQueryRecord>,
    /// Bills associated with the payout subscription.
    #[serde(rename = "BillData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_data: Option<Vec<BillPayOutData>>,
    #[serde(rename = "ExternalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    /// The payout subscription's payment method.
    #[serde(rename = "Method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(rename = "PaypointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_id: Option<PaypointId>,
    /// The payout subscription amount, including any fees.
    #[serde(rename = "TotalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_amount: Option<f64>,
    /// The payout subscription amount, minus any fees.
    #[serde(rename = "NetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<Netamountnullable>,
    /// Fee applied to the payout subscription.
    #[serde(rename = "FeeAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub fee_amount: Option<f64>,
    #[serde(rename = "PaymentData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_data: Option<QueryPaymentData>,
    /// The payout subscription start date.
    #[serde(rename = "StartDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub start_date: Option<DateTime<Utc>>,
    /// The payout subscription's end date.
    #[serde(rename = "EndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub end_date: Option<DateTime<Utc>>,
    /// The next date the payout subscription will be processed.
    #[serde(rename = "NextDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub next_date: Option<DateTime<Utc>>,
    /// The payout subscription's frequency.
    #[serde(rename = "Frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    /// The total number of cycles the payout subscription is set to run.
    #[serde(rename = "TotalCycles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_cycles: Option<i64>,
    /// The number of cycles the payout subscription has left.
    #[serde(rename = "LeftCycles")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub left_cycles: Option<i64>,
    /// The last time the payout subscription was processed.
    #[serde(rename = "LastRun")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub last_run: Option<DateTime<Utc>>,
    #[serde(rename = "EntrypageId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entrypage_id: Option<EntrypageId>,
    /// When `true`, the payout subscription has no explicit end date and runs until canceled.
    #[serde(rename = "UntilCancelled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub until_cancelled: Option<bool>,
    /// The last date and time the payout subscription was updated.
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<LastModified>,
    /// Timestamp of when the payout subscription was created, in UTC.
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    /// The paypoint's legal name.
    #[serde(rename = "PaypointLegalname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legalname: Option<Legalname>,
    /// The paypoint's DBA name.
    #[serde(rename = "PaypointDbaname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dbaname: Option<Dbaname>,
    /// The paypoint's entryname.
    #[serde(rename = "PaypointEntryname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_entryname: Option<Entrypointfield>,
    #[serde(rename = "ParentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<OrgParentName>,
    #[serde(rename = "ParentOrgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_id: Option<OrgParentId>,
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<Source>,
}

impl PayoutSubscriptionQueryRecordPascal {
    pub fn builder() -> PayoutSubscriptionQueryRecordPascalBuilder {
        <PayoutSubscriptionQueryRecordPascalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayoutSubscriptionQueryRecordPascalBuilder {
    id_out_subscription: Option<i64>,
    status: Option<i64>,
    events: Option<Vec<GeneralEvents>>,
    vendor: Option<VendorQueryRecord>,
    bill_data: Option<Vec<BillPayOutData>>,
    external_paypoint_id: Option<ExternalPaypointId>,
    method: Option<String>,
    paypoint_id: Option<PaypointId>,
    total_amount: Option<f64>,
    net_amount: Option<Netamountnullable>,
    fee_amount: Option<f64>,
    payment_data: Option<QueryPaymentData>,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
    next_date: Option<DateTime<Utc>>,
    frequency: Option<String>,
    total_cycles: Option<i64>,
    left_cycles: Option<i64>,
    last_run: Option<DateTime<Utc>>,
    entrypage_id: Option<EntrypageId>,
    until_cancelled: Option<bool>,
    last_updated: Option<LastModified>,
    created_at: Option<CreatedAt>,
    paypoint_legalname: Option<Legalname>,
    paypoint_dbaname: Option<Dbaname>,
    paypoint_entryname: Option<Entrypointfield>,
    parent_org_name: Option<OrgParentName>,
    parent_org_id: Option<OrgParentId>,
    source: Option<Source>,
}

impl PayoutSubscriptionQueryRecordPascalBuilder {
    pub fn id_out_subscription(mut self, value: i64) -> Self {
        self.id_out_subscription = Some(value);
        self
    }

    pub fn status(mut self, value: i64) -> Self {
        self.status = Some(value);
        self
    }

    pub fn events(mut self, value: Vec<GeneralEvents>) -> Self {
        self.events = Some(value);
        self
    }

    pub fn vendor(mut self, value: VendorQueryRecord) -> Self {
        self.vendor = Some(value);
        self
    }

    pub fn bill_data(mut self, value: Vec<BillPayOutData>) -> Self {
        self.bill_data = Some(value);
        self
    }

    pub fn external_paypoint_id(mut self, value: ExternalPaypointId) -> Self {
        self.external_paypoint_id = Some(value);
        self
    }

    pub fn method(mut self, value: impl Into<String>) -> Self {
        self.method = Some(value.into());
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

    pub fn fee_amount(mut self, value: f64) -> Self {
        self.fee_amount = Some(value);
        self
    }

    pub fn payment_data(mut self, value: QueryPaymentData) -> Self {
        self.payment_data = Some(value);
        self
    }

    pub fn start_date(mut self, value: DateTime<Utc>) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn end_date(mut self, value: DateTime<Utc>) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn next_date(mut self, value: DateTime<Utc>) -> Self {
        self.next_date = Some(value);
        self
    }

    pub fn frequency(mut self, value: impl Into<String>) -> Self {
        self.frequency = Some(value.into());
        self
    }

    pub fn total_cycles(mut self, value: i64) -> Self {
        self.total_cycles = Some(value);
        self
    }

    pub fn left_cycles(mut self, value: i64) -> Self {
        self.left_cycles = Some(value);
        self
    }

    pub fn last_run(mut self, value: DateTime<Utc>) -> Self {
        self.last_run = Some(value);
        self
    }

    pub fn entrypage_id(mut self, value: EntrypageId) -> Self {
        self.entrypage_id = Some(value);
        self
    }

    pub fn until_cancelled(mut self, value: bool) -> Self {
        self.until_cancelled = Some(value);
        self
    }

    pub fn last_updated(mut self, value: LastModified) -> Self {
        self.last_updated = Some(value);
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

    pub fn paypoint_dbaname(mut self, value: Dbaname) -> Self {
        self.paypoint_dbaname = Some(value);
        self
    }

    pub fn paypoint_entryname(mut self, value: Entrypointfield) -> Self {
        self.paypoint_entryname = Some(value);
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

    pub fn source(mut self, value: Source) -> Self {
        self.source = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayoutSubscriptionQueryRecordPascal`].
    pub fn build(self) -> Result<PayoutSubscriptionQueryRecordPascal, BuildError> {
        Ok(PayoutSubscriptionQueryRecordPascal {
            id_out_subscription: self.id_out_subscription,
            status: self.status,
            events: self.events,
            vendor: self.vendor,
            bill_data: self.bill_data,
            external_paypoint_id: self.external_paypoint_id,
            method: self.method,
            paypoint_id: self.paypoint_id,
            total_amount: self.total_amount,
            net_amount: self.net_amount,
            fee_amount: self.fee_amount,
            payment_data: self.payment_data,
            start_date: self.start_date,
            end_date: self.end_date,
            next_date: self.next_date,
            frequency: self.frequency,
            total_cycles: self.total_cycles,
            left_cycles: self.left_cycles,
            last_run: self.last_run,
            entrypage_id: self.entrypage_id,
            until_cancelled: self.until_cancelled,
            last_updated: self.last_updated,
            created_at: self.created_at,
            paypoint_legalname: self.paypoint_legalname,
            paypoint_dbaname: self.paypoint_dbaname,
            paypoint_entryname: self.paypoint_entryname,
            parent_org_name: self.parent_org_name,
            parent_org_id: self.parent_org_id,
            source: self.source,
        })
    }
}
