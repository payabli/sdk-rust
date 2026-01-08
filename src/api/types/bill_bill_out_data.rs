pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BillOutData {
    #[serde(rename = "accountingField1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_field_1: Option<AccountingField>,
    #[serde(rename = "accountingField2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_field_2: Option<AccountingField>,
    #[serde(rename = "additionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalDataString>,
    /// An array of bill images. Attachments aren't required, but we strongly recommend including them. Including a bill image can make payouts smoother and prevent delays. You can include either the Base64-encoded file content, or you can include an fURL to a public file. The maximum file size for image uploads is 30 MB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Attachments>,
    /// Date of bill. Accepted formats: YYYY-MM-DD, MM/DD/YYYY.
    #[serde(rename = "billDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_date: Option<Datenullable>,
    #[serde(rename = "billItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_items: Option<Billitems>,
    /// Unique identifier for the bill. Required when adding a bill.
    #[serde(rename = "billNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Comments>,
    /// Discount amount applied to the bill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<f64>,
    /// Due date of bill. Accepted formats: YYYY-MM-DD, MM/DD/YYYY.
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<Datenullable>,
    /// End Date for scheduled bills. Applied only in `Mode` = 1. Accepted formats: YYYY-MM-DD, MM/DD/YYYY
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<Datenullable>,
    /// Frequency for scheduled bills. Applied only in `Mode` = 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequency>,
    /// Lot number associated with the bill.
    #[serde(rename = "lotNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lot_number: Option<String>,
    /// Bill mode: value `0` for one-time bills, `1` for scheduled bills.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i64>,
    /// Net Amount owed in bill. Required when adding a bill.
    #[serde(rename = "netAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<f64>,
    /// Options for scheduled bills.
    #[serde(rename = "scheduledOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_options: Option<BillOutDataScheduledOptions>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Billstatus>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms: Option<Terms>,
    /// Total amount of the bill.
    #[serde(rename = "totalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<f64>,
    /// The vendor associated with the bill. Although you can create a vendor in a create bill request, Payabli recommends creating a vendor separately and passing a valid `vendorNumber` here. At minimum, the `vendorNumber` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<VendorData>,
}
