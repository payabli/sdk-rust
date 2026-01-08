pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    pub additional_data: Option<HashMap<String, String>>,
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
    pub bill_date: Option<Datenullable>,
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
    pub discount: Option<f64>,
    /// Reference to documents associated with the bill.
    #[serde(rename = "DocumentsRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents_ref: Option<String>,
    /// Bill due date in one of the accepted formats: YYYY-MM-DD, MM/DD/YYYY.
    #[serde(rename = "DueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<Datenullable>,
    /// End date for the bill.
    #[serde(rename = "EndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Datenullable>,
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
    pub last_updated: Option<DatetimeNullable>,
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
    pub total_amount: Option<f64>,
    /// MoneyOut transaction associated to the bill.
    #[serde(rename = "Transaction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<TransactionOutQueryRecord>,
    #[serde(rename = "Vendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<VendorOutData>,
}
