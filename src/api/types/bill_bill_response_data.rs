pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BillResponseData {
    #[serde(rename = "IdBill")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_bill: Option<BillId>,
    /// Unique identifier for the bill.
    #[serde(rename = "BillNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_number: Option<String>,
    /// Net amount owed in bill.
    #[serde(rename = "NetAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<f64>,
    /// Bill discount amount.
    #[serde(rename = "Discount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<f64>,
    /// Total amount for the bill.
    #[serde(rename = "TotalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    /// Date of bill. Accepted formats: YYYY-MM-DD, MM/DD/YYYY
    #[serde(rename = "BillDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_date: Option<NaiveDate>,
    /// Due Date of bill. Accepted formats: YYYY-MM-DD, MM/DD/YYYY
    #[serde(rename = "DueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<NaiveDate>,
    /// Comments associated with the bill. For managed payables, the character limit is 200. For on demand payouts, the characters limit is 250.
    #[serde(rename = "Comments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// The batch number that the bill belongs to.
    #[serde(rename = "BatchNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub batch_number: Option<String>,
    /// Array of `LineItems` contained in bill.
    #[serde(rename = "BillItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_items: Option<Billitems>,
    /// Bill mode: value `0` for single/one-time bills, `1` for scheduled bills.
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i64>,
    /// Payment method used for the bill.
    #[serde(rename = "PaymentMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    /// Payment ID associated with the bill.
    #[serde(rename = "PaymentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<String>,
    #[serde(rename = "AccountingField1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_field_1: Option<AccountingField>,
    #[serde(rename = "AccountingField2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_field_2: Option<AccountingField>,
    #[serde(rename = "Terms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms: Option<Terms>,
    /// The source of the bill, such as "API" or "UI".
    #[serde(rename = "Source")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    #[serde(rename = "AdditionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalDataString>,
    #[serde(rename = "Vendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<VendorDataResponse>,
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Billstatus>,
    #[serde(rename = "CreatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    /// End date for scheduled bills. Applied only in `Mode` = 1.
    #[serde(rename = "EndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<NaiveDate>,
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<LastModified>,
    /// Frequency for scheduled bills. Applied only in `Mode` = 1.
    #[serde(rename = "Frequency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequency>,
    /// MoneyOut transaction associated to the bill
    #[serde(rename = "Transaction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<TransactionOutQueryRecord>,
    #[serde(rename = "billEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_events: Option<BillEvents>,
    #[serde(rename = "billApprovals")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_approvals: Option<BillApprovals>,
    #[serde(rename = "PaypointLegalname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legalname: Option<Legalname>,
    #[serde(rename = "PaypointDbaname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dbaname: Option<Dbaname>,
    #[serde(rename = "ParentOrgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_id: Option<OrgParentId>,
    #[serde(rename = "ParentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<OrgParentName>,
    #[serde(rename = "PaypointEntryname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_entryname: Option<Entrypointfield>,
    #[serde(rename = "paylinkId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paylink_id: Option<PaylinkId>,
    /// Object with the attached documents.
    #[serde(rename = "DocumentsRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents_ref: Option<DocumentsRef>,
    #[serde(rename = "externalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    /// Lot number of the bill.
    #[serde(rename = "LotNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lot_number: Option<String>,
    #[serde(rename = "EntityID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_id: Option<EntityId>,
}

impl BillResponseData {
    pub fn builder() -> BillResponseDataBuilder {
        <BillResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillResponseDataBuilder {
    id_bill: Option<BillId>,
    bill_number: Option<String>,
    net_amount: Option<f64>,
    discount: Option<f64>,
    total_amount: Option<f64>,
    bill_date: Option<NaiveDate>,
    due_date: Option<NaiveDate>,
    comments: Option<String>,
    batch_number: Option<String>,
    bill_items: Option<Billitems>,
    mode: Option<i64>,
    payment_method: Option<String>,
    payment_id: Option<String>,
    accounting_field_1: Option<AccountingField>,
    accounting_field_2: Option<AccountingField>,
    terms: Option<Terms>,
    source: Option<String>,
    additional_data: Option<AdditionalDataString>,
    vendor: Option<VendorDataResponse>,
    status: Option<Billstatus>,
    created_at: Option<CreatedAt>,
    end_date: Option<NaiveDate>,
    last_updated: Option<LastModified>,
    frequency: Option<Frequency>,
    transaction: Option<TransactionOutQueryRecord>,
    bill_events: Option<BillEvents>,
    bill_approvals: Option<BillApprovals>,
    paypoint_legalname: Option<Legalname>,
    paypoint_dbaname: Option<Dbaname>,
    parent_org_id: Option<OrgParentId>,
    parent_org_name: Option<OrgParentName>,
    paypoint_entryname: Option<Entrypointfield>,
    paylink_id: Option<PaylinkId>,
    documents_ref: Option<DocumentsRef>,
    external_paypoint_id: Option<ExternalPaypointId>,
    lot_number: Option<String>,
    entity_id: Option<EntityId>,
}

impl BillResponseDataBuilder {
    pub fn id_bill(mut self, value: BillId) -> Self {
        self.id_bill = Some(value);
        self
    }

    pub fn bill_number(mut self, value: impl Into<String>) -> Self {
        self.bill_number = Some(value.into());
        self
    }

    pub fn net_amount(mut self, value: f64) -> Self {
        self.net_amount = Some(value);
        self
    }

    pub fn discount(mut self, value: f64) -> Self {
        self.discount = Some(value);
        self
    }

    pub fn total_amount(mut self, value: f64) -> Self {
        self.total_amount = Some(value);
        self
    }

    pub fn bill_date(mut self, value: NaiveDate) -> Self {
        self.bill_date = Some(value);
        self
    }

    pub fn due_date(mut self, value: NaiveDate) -> Self {
        self.due_date = Some(value);
        self
    }

    pub fn comments(mut self, value: impl Into<String>) -> Self {
        self.comments = Some(value.into());
        self
    }

    pub fn batch_number(mut self, value: impl Into<String>) -> Self {
        self.batch_number = Some(value.into());
        self
    }

    pub fn bill_items(mut self, value: Billitems) -> Self {
        self.bill_items = Some(value);
        self
    }

    pub fn mode(mut self, value: i64) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn payment_method(mut self, value: impl Into<String>) -> Self {
        self.payment_method = Some(value.into());
        self
    }

    pub fn payment_id(mut self, value: impl Into<String>) -> Self {
        self.payment_id = Some(value.into());
        self
    }

    pub fn accounting_field_1(mut self, value: AccountingField) -> Self {
        self.accounting_field_1 = Some(value);
        self
    }

    pub fn accounting_field_2(mut self, value: AccountingField) -> Self {
        self.accounting_field_2 = Some(value);
        self
    }

    pub fn terms(mut self, value: Terms) -> Self {
        self.terms = Some(value);
        self
    }

    pub fn source(mut self, value: impl Into<String>) -> Self {
        self.source = Some(value.into());
        self
    }

    pub fn additional_data(mut self, value: AdditionalDataString) -> Self {
        self.additional_data = Some(value);
        self
    }

    pub fn vendor(mut self, value: VendorDataResponse) -> Self {
        self.vendor = Some(value);
        self
    }

    pub fn status(mut self, value: Billstatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn created_at(mut self, value: CreatedAt) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn end_date(mut self, value: NaiveDate) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn last_updated(mut self, value: LastModified) -> Self {
        self.last_updated = Some(value);
        self
    }

    pub fn frequency(mut self, value: Frequency) -> Self {
        self.frequency = Some(value);
        self
    }

    pub fn transaction(mut self, value: TransactionOutQueryRecord) -> Self {
        self.transaction = Some(value);
        self
    }

    pub fn bill_events(mut self, value: BillEvents) -> Self {
        self.bill_events = Some(value);
        self
    }

    pub fn bill_approvals(mut self, value: BillApprovals) -> Self {
        self.bill_approvals = Some(value);
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

    pub fn parent_org_id(mut self, value: OrgParentId) -> Self {
        self.parent_org_id = Some(value);
        self
    }

    pub fn parent_org_name(mut self, value: OrgParentName) -> Self {
        self.parent_org_name = Some(value);
        self
    }

    pub fn paypoint_entryname(mut self, value: Entrypointfield) -> Self {
        self.paypoint_entryname = Some(value);
        self
    }

    pub fn paylink_id(mut self, value: PaylinkId) -> Self {
        self.paylink_id = Some(value);
        self
    }

    pub fn documents_ref(mut self, value: DocumentsRef) -> Self {
        self.documents_ref = Some(value);
        self
    }

    pub fn external_paypoint_id(mut self, value: ExternalPaypointId) -> Self {
        self.external_paypoint_id = Some(value);
        self
    }

    pub fn lot_number(mut self, value: impl Into<String>) -> Self {
        self.lot_number = Some(value.into());
        self
    }

    pub fn entity_id(mut self, value: EntityId) -> Self {
        self.entity_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BillResponseData`].
    pub fn build(self) -> Result<BillResponseData, BuildError> {
        Ok(BillResponseData {
            id_bill: self.id_bill,
            bill_number: self.bill_number,
            net_amount: self.net_amount,
            discount: self.discount,
            total_amount: self.total_amount,
            bill_date: self.bill_date,
            due_date: self.due_date,
            comments: self.comments,
            batch_number: self.batch_number,
            bill_items: self.bill_items,
            mode: self.mode,
            payment_method: self.payment_method,
            payment_id: self.payment_id,
            accounting_field_1: self.accounting_field_1,
            accounting_field_2: self.accounting_field_2,
            terms: self.terms,
            source: self.source,
            additional_data: self.additional_data,
            vendor: self.vendor,
            status: self.status,
            created_at: self.created_at,
            end_date: self.end_date,
            last_updated: self.last_updated,
            frequency: self.frequency,
            transaction: self.transaction,
            bill_events: self.bill_events,
            bill_approvals: self.bill_approvals,
            paypoint_legalname: self.paypoint_legalname,
            paypoint_dbaname: self.paypoint_dbaname,
            parent_org_id: self.parent_org_id,
            parent_org_name: self.parent_org_name,
            paypoint_entryname: self.paypoint_entryname,
            paylink_id: self.paylink_id,
            documents_ref: self.documents_ref,
            external_paypoint_id: self.external_paypoint_id,
            lot_number: self.lot_number,
            entity_id: self.entity_id,
        })
    }
}
