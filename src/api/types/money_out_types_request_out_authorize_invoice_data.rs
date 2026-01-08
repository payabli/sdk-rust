pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestOutAuthorizeInvoiceData {
    #[serde(rename = "invoiceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_number: Option<InvoiceNumber>,
    #[serde(rename = "netAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<NetAmountstring>,
    /// Invoice date in any of the accepted formats: YYYY-MM-DD, MM/DD/YYYY.
    #[serde(rename = "invoiceDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_date: Option<Datenullable>,
    /// Invoice due date in any of the accepted formats: YYYY-MM-DD, MM/DD/YYYY.
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<Datenullable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Comments>,
    #[serde(rename = "lotNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lot_number: Option<LotNumber>,
    #[serde(rename = "billId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_id: Option<BillId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<Discount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms: Option<Terms>,
    #[serde(rename = "accountingField1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_field_1: Option<AccountingField>,
    #[serde(rename = "accountingField2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_field_2: Option<AccountingField>,
    #[serde(rename = "additionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalDataString>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Attachments>,
}
