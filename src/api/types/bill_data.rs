pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BillData {
    #[serde(rename = "AdditionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalDataMap>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Attachments>,
    /// Company name of the recipient of the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<Discount>,
    #[serde(rename = "dutyAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duty_amount: Option<DutyAmount>,
    /// First name of the recipient of the invoice.
    #[serde(rename = "firstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(rename = "freightAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freight_amount: Option<FreightAmount>,
    /// Frequency of scheduled invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequency>,
    #[serde(rename = "invoiceAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_amount: Option<InvoiceAmount>,
    /// Invoice date in any of the accepted formats: YYYY-MM-DD, MM/DD/YYYY.
    #[serde(rename = "invoiceDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_date: Option<Datenullable>,
    /// Invoice due date in one of the accepted formats: YYYY-MM-DD, MM/DD/YYYY.
    #[serde(rename = "invoiceDueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_due_date: Option<Datenullable>,
    /// Indicate the date to finish a scheduled invoice cycle (`invoiceType`` = 1) in any of the accepted formats: YYYY-MM-DD, MM/DD/YYYY.
    #[serde(rename = "invoiceEndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_end_date: Option<Datenullable>,
    /// Invoice number. Identifies the invoice under a paypoint.
    #[serde(rename = "invoiceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_number: Option<InvoiceNumber>,
    #[serde(rename = "invoiceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_status: Option<Invoicestatus>,
    #[serde(rename = "invoiceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_type: Option<InvoiceType>,
    /// Array of line items included in the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<BillItem>>,
    /// Last name of the recipient of the invoice.
    #[serde(rename = "lastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// Notes included in the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(rename = "paymentTerms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_terms: Option<BillDataPaymentTerms>,
    #[serde(rename = "purchaseOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_order: Option<PurchaseOrder>,
    #[serde(rename = "shippingAddress1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_1: Option<Shippingaddress>,
    #[serde(rename = "shippingAddress2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_2: Option<Shippingaddressadditional>,
    #[serde(rename = "shippingCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_city: Option<Shippingcity>,
    #[serde(rename = "shippingCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_country: Option<Shippingcountry>,
    /// Shipping recipient's contact email address.
    #[serde(rename = "shippingEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_email: Option<Email>,
    #[serde(rename = "shippingFromZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_from_zip: Option<ShippingFromZip>,
    /// Recipient phone number.
    #[serde(rename = "shippingPhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_phone: Option<String>,
    #[serde(rename = "shippingState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_state: Option<Shippingstate>,
    #[serde(rename = "shippingZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_zip: Option<Shippingzip>,
    #[serde(rename = "summaryCommodityCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_commodity_code: Option<SummaryCommodityCode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<Tax>,
    #[serde(rename = "termsConditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_conditions: Option<TermsConditions>,
}
