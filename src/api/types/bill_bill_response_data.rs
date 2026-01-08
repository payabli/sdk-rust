pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    pub bill_date: Option<Datenullable>,
    /// Due Date of bill. Accepted formats: YYYY-MM-DD, MM/DD/YYYY
    #[serde(rename = "DueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<Datenullable>,
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
    pub end_date: Option<Datenullable>,
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
