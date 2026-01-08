pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BillPayOutData {
    /// Bill ID in Payabli.
    #[serde(rename = "billId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_id: Option<i64>,
    /// Any comments about bill. **For managed payouts, this field has a limit of 100 characters**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Comments>,
    /// Bill due date in format YYYY-MM-DD or MM/DD/YYYY.
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<Datenullable>,
    /// Bill date in format YYYY-MM-DD or MM/DD/YYYY.
    #[serde(rename = "invoiceDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_date: Option<Datenullable>,
    /// Custom number identifying the bill. Must be unique in paypoint. **Required** for new bill and when `billId` isn't provided.
    #[serde(rename = "invoiceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_number: Option<InvoiceNumber>,
    /// Net Amount owed in bill. Required when adding a bill.
    #[serde(rename = "netAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<NetAmountstring>,
    /// Bill discount amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<String>,
    /// Description of payment terms.
    #[serde(rename = "Terms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms: Option<Terms>,
    #[serde(rename = "AccountingField1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_field_1: Option<AccountingField>,
    #[serde(rename = "AccountingField2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_field_2: Option<AccountingField>,
    #[serde(rename = "AdditionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalDataString>,
    /// Bill image attachment. Send the bill image as Base64-encoded string, or as a publicly accessible link. For full details on using this field with a payout authorization, see [the documentation](/developers/developer-guides/pay-out-manage-payouts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Attachments>,
}
