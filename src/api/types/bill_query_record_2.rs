pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BillQueryRecord2 {
    #[serde(rename = "AccountingField1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_field_1: Option<AccountingField>,
    #[serde(rename = "AccountingField2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_field_2: Option<AccountingField>,
    /// Additional data associated with the bill.
    #[serde(rename = "AdditionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<HashMap<String, Option<String>>>,
    /// Batch number associated with the bill.
    #[serde(rename = "BatchNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_number: Option<String>,
    #[serde(rename = "billApprovals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_approvals: Option<Vec<BillQueryRecord2BillApprovalsItem>>,
    /// Bill creation date in one of the accepted formats: YYYY-MM-DD, MM/DD/YYYY.
    #[serde(rename = "BillDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_date: Option<NaiveDate>,
    /// Events associated with the bill.
    #[serde(rename = "billEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_events: Option<Vec<GeneralEvents>>,
    /// Array of items included in the bill.
    #[serde(rename = "BillItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_items: Option<Vec<BillItem>>,
    /// Bill number.
    #[serde(rename = "BillNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_number: Option<String>,
    /// Additional comments on the bill.
    #[serde(rename = "Comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Timestamp of when bill was created, in UTC.
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    /// Discount amount applied to the bill.
    #[serde(rename = "Discount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub discount: Option<f64>,
    /// Reference to documents associated with the bill.
    #[serde(rename = "DocumentsRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents_ref: Option<String>,
    /// Bill due date in one of the accepted formats: YYYY-MM-DD, MM/DD/YYYY.
    #[serde(rename = "DueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<NaiveDate>,
    /// End date for the bill.
    #[serde(rename = "EndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<NaiveDate>,
    /// Entity identifier associated with the bill.
    #[serde(rename = "EntityID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<String>,
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    /// Frequency for scheduled bills. Applied only in `Mode` = 1.
    #[serde(rename = "Frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequency>,
    /// Identifier of the bill.
    #[serde(rename = "IdBill")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_bill: Option<i64>,
    /// Timestamp of when bill was last updated, in UTC.
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub last_updated: Option<DateTime<Utc>>,
    /// Lot number associated with the bill.
    #[serde(rename = "LotNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lot_number: Option<String>,
    /// Bill mode: value `0` for single/one-time bills, `1` for scheduled bills.
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i64>,
    /// Net amount of the bill.
    #[serde(rename = "NetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub net_amount: Option<f64>,
    /// Parent organization identifier.
    #[serde(rename = "ParentOrgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_id: Option<i64>,
    #[serde(rename = "ParentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<OrgParentName>,
    #[serde(rename = "PaymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<PaymentIdString>,
    /// Preferred payment method used.
    #[serde(rename = "PaymentMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<BillQueryRecord2PaymentMethod>,
    /// Paylink identifier associated with the bill.
    #[serde(rename = "paylinkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paylink_id: Option<String>,
    /// The paypoint's DBA name.
    #[serde(rename = "PaypointDbaname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dbaname: Option<Dbaname>,
    /// Entry name of the paypoint.
    #[serde(rename = "PaypointEntryname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_entryname: Option<String>,
    /// The paypoint's legal name.
    #[serde(rename = "PaypointLegalname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legalname: Option<Legalname>,
    /// Source of the bill.
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Billstatus>,
    /// The payment terms for invoice. If no terms were defined initially, then response data for this field will default to `N30`.
    #[serde(rename = "Terms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms: Option<Terms>,
    /// Total amount of the bill including taxes and fees.
    #[serde(rename = "TotalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_amount: Option<f64>,
    /// MoneyOut transaction associated to the bill.
    #[serde(rename = "Transaction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<TransactionOutQueryRecord>,
    #[serde(rename = "Vendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<VendorOutData>,
}

impl BillQueryRecord2 {
    pub fn builder() -> BillQueryRecord2Builder {
        <BillQueryRecord2Builder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillQueryRecord2Builder {
    accounting_field_1: Option<AccountingField>,
    accounting_field_2: Option<AccountingField>,
    additional_data: Option<HashMap<String, Option<String>>>,
    batch_number: Option<String>,
    bill_approvals: Option<Vec<BillQueryRecord2BillApprovalsItem>>,
    bill_date: Option<NaiveDate>,
    bill_events: Option<Vec<GeneralEvents>>,
    bill_items: Option<Vec<BillItem>>,
    bill_number: Option<String>,
    comments: Option<String>,
    created_at: Option<CreatedAt>,
    discount: Option<f64>,
    documents_ref: Option<String>,
    due_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    entity_id: Option<String>,
    external_paypoint_id: Option<ExternalPaypointId>,
    frequency: Option<Frequency>,
    id_bill: Option<i64>,
    last_updated: Option<DateTime<Utc>>,
    lot_number: Option<String>,
    mode: Option<i64>,
    net_amount: Option<f64>,
    parent_org_id: Option<i64>,
    parent_org_name: Option<OrgParentName>,
    payment_id: Option<PaymentIdString>,
    payment_method: Option<BillQueryRecord2PaymentMethod>,
    paylink_id: Option<String>,
    paypoint_dbaname: Option<Dbaname>,
    paypoint_entryname: Option<String>,
    paypoint_legalname: Option<Legalname>,
    source: Option<String>,
    status: Option<Billstatus>,
    terms: Option<Terms>,
    total_amount: Option<f64>,
    transaction: Option<TransactionOutQueryRecord>,
    vendor: Option<VendorOutData>,
}

impl BillQueryRecord2Builder {
    pub fn accounting_field_1(mut self, value: AccountingField) -> Self {
        self.accounting_field_1 = Some(value);
        self
    }

    pub fn accounting_field_2(mut self, value: AccountingField) -> Self {
        self.accounting_field_2 = Some(value);
        self
    }

    pub fn additional_data(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.additional_data = Some(value);
        self
    }

    pub fn batch_number(mut self, value: impl Into<String>) -> Self {
        self.batch_number = Some(value.into());
        self
    }

    pub fn bill_approvals(mut self, value: Vec<BillQueryRecord2BillApprovalsItem>) -> Self {
        self.bill_approvals = Some(value);
        self
    }

    pub fn bill_date(mut self, value: NaiveDate) -> Self {
        self.bill_date = Some(value);
        self
    }

    pub fn bill_events(mut self, value: Vec<GeneralEvents>) -> Self {
        self.bill_events = Some(value);
        self
    }

    pub fn bill_items(mut self, value: Vec<BillItem>) -> Self {
        self.bill_items = Some(value);
        self
    }

    pub fn bill_number(mut self, value: impl Into<String>) -> Self {
        self.bill_number = Some(value.into());
        self
    }

    pub fn comments(mut self, value: impl Into<String>) -> Self {
        self.comments = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: CreatedAt) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn discount(mut self, value: f64) -> Self {
        self.discount = Some(value);
        self
    }

    pub fn documents_ref(mut self, value: impl Into<String>) -> Self {
        self.documents_ref = Some(value.into());
        self
    }

    pub fn due_date(mut self, value: NaiveDate) -> Self {
        self.due_date = Some(value);
        self
    }

    pub fn end_date(mut self, value: NaiveDate) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn entity_id(mut self, value: impl Into<String>) -> Self {
        self.entity_id = Some(value.into());
        self
    }

    pub fn external_paypoint_id(mut self, value: ExternalPaypointId) -> Self {
        self.external_paypoint_id = Some(value);
        self
    }

    pub fn frequency(mut self, value: Frequency) -> Self {
        self.frequency = Some(value);
        self
    }

    pub fn id_bill(mut self, value: i64) -> Self {
        self.id_bill = Some(value);
        self
    }

    pub fn last_updated(mut self, value: DateTime<Utc>) -> Self {
        self.last_updated = Some(value);
        self
    }

    pub fn lot_number(mut self, value: impl Into<String>) -> Self {
        self.lot_number = Some(value.into());
        self
    }

    pub fn mode(mut self, value: i64) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn net_amount(mut self, value: f64) -> Self {
        self.net_amount = Some(value);
        self
    }

    pub fn parent_org_id(mut self, value: i64) -> Self {
        self.parent_org_id = Some(value);
        self
    }

    pub fn parent_org_name(mut self, value: OrgParentName) -> Self {
        self.parent_org_name = Some(value);
        self
    }

    pub fn payment_id(mut self, value: PaymentIdString) -> Self {
        self.payment_id = Some(value);
        self
    }

    pub fn payment_method(mut self, value: BillQueryRecord2PaymentMethod) -> Self {
        self.payment_method = Some(value);
        self
    }

    pub fn paylink_id(mut self, value: impl Into<String>) -> Self {
        self.paylink_id = Some(value.into());
        self
    }

    pub fn paypoint_dbaname(mut self, value: Dbaname) -> Self {
        self.paypoint_dbaname = Some(value);
        self
    }

    pub fn paypoint_entryname(mut self, value: impl Into<String>) -> Self {
        self.paypoint_entryname = Some(value.into());
        self
    }

    pub fn paypoint_legalname(mut self, value: Legalname) -> Self {
        self.paypoint_legalname = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn status(mut self, value: Billstatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn terms(mut self, value: Terms) -> Self {
        self.terms = Some(value);
        self
    }

    pub fn total_amount(mut self, value: f64) -> Self {
        self.total_amount = Some(value);
        self
    }

    pub fn transaction(mut self, value: TransactionOutQueryRecord) -> Self {
        self.transaction = Some(value);
        self
    }

    pub fn vendor(mut self, value: VendorOutData) -> Self {
        self.vendor = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BillQueryRecord2`].
    pub fn build(self) -> Result<BillQueryRecord2, BuildError> {
        Ok(BillQueryRecord2 {
            accounting_field_1: self.accounting_field_1,
            accounting_field_2: self.accounting_field_2,
            additional_data: self.additional_data,
            batch_number: self.batch_number,
            bill_approvals: self.bill_approvals,
            bill_date: self.bill_date,
            bill_events: self.bill_events,
            bill_items: self.bill_items,
            bill_number: self.bill_number,
            comments: self.comments,
            created_at: self.created_at,
            discount: self.discount,
            documents_ref: self.documents_ref,
            due_date: self.due_date,
            end_date: self.end_date,
            entity_id: self.entity_id,
            external_paypoint_id: self.external_paypoint_id,
            frequency: self.frequency,
            id_bill: self.id_bill,
            last_updated: self.last_updated,
            lot_number: self.lot_number,
            mode: self.mode,
            net_amount: self.net_amount,
            parent_org_id: self.parent_org_id,
            parent_org_name: self.parent_org_name,
            payment_id: self.payment_id,
            payment_method: self.payment_method,
            paylink_id: self.paylink_id,
            paypoint_dbaname: self.paypoint_dbaname,
            paypoint_entryname: self.paypoint_entryname,
            paypoint_legalname: self.paypoint_legalname,
            source: self.source,
            status: self.status,
            terms: self.terms,
            total_amount: self.total_amount,
            transaction: self.transaction,
            vendor: self.vendor,
        })
    }
}
