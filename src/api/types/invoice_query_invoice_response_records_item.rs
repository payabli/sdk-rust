pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryInvoiceResponseRecordsItem {
    #[serde(rename = "invoiceId")]
    pub invoice_id: InvoiceId,
    #[serde(rename = "customerId")]
    pub customer_id: CustomerId,
    #[serde(rename = "paypointId")]
    pub paypoint_id: PaypointId,
    #[serde(rename = "invoiceNumber")]
    pub invoice_number: InvoiceNumber,
    /// Invoice date in any of the accepted formats: YYYY-MM-DD, MM/DD/YYYY.
    #[serde(rename = "invoiceDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_date: Option<NaiveDate>,
    /// Invoice due date in any of the accepted formats: YYYY-MM-DD, MM/DD/YYYY.
    #[serde(rename = "invoiceDueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_due_date: Option<NaiveDate>,
    /// Invoice sent date in any of the accepted formats: YYYY-MM-DD, MM/DD/YYYY.
    #[serde(rename = "invoiceSentDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_sent_date: Option<NaiveDate>,
    /// The end date for a scheduled invoice cycle (`invoiceType` = 1).
    #[serde(rename = "invoiceEndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_end_date: Option<NaiveDate>,
    /// Timestamp of last payment.
    #[serde(rename = "lastPaymentDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub last_payment_date: Option<DateTime<Utc>>,
    #[serde(rename = "createdAt")]
    pub created_at: CreatedAt,
    #[serde(rename = "invoiceStatus")]
    pub invoice_status: Invoicestatus,
    #[serde(rename = "invoiceType")]
    pub invoice_type: InvoiceType,
    /// Frequency of scheduled invoice.
    pub frequency: Frequency,
    #[serde(rename = "paymentTerms")]
    pub payment_terms: Terms,
    #[serde(rename = "termsConditions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_conditions: Option<TermsConditions>,
    /// Invoice notes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<Tax>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<Discount>,
    #[serde(rename = "invoiceAmount")]
    pub invoice_amount: InvoiceAmount,
    #[serde(rename = "invoicePaidAmount")]
    pub invoice_paid_amount: InvoicePaidAmount,
    #[serde(rename = "freightAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub freight_amount: Option<FreightAmount>,
    #[serde(rename = "dutyAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duty_amount: Option<DutyAmount>,
    #[serde(rename = "purchaseOrder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub purchase_order: Option<PurchaseOrder>,
    /// First name of the recipient of the invoice.
    #[serde(rename = "firstName")]
    pub first_name: String,
    /// Last name of the recipient of the invoice.
    #[serde(rename = "lastName")]
    pub last_name: String,
    /// Company name of the recipient of the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(rename = "shippingAddress1")]
    pub shipping_address_1: Shippingaddress,
    #[serde(rename = "shippingAddress2")]
    pub shipping_address_2: Shippingaddressadditional,
    #[serde(rename = "shippingCity")]
    pub shipping_city: Shippingcity,
    #[serde(rename = "shippingState")]
    pub shipping_state: Shippingstate,
    #[serde(rename = "shippingZip")]
    pub shipping_zip: Shippingzip,
    #[serde(rename = "shippingFromZip")]
    pub shipping_from_zip: ShippingFromZip,
    #[serde(rename = "shippingCountry")]
    pub shipping_country: Shippingcountry,
    /// Shipping recipient's contact email address.
    #[serde(rename = "shippingEmail")]
    pub shipping_email: Email,
    /// Recipient phone number.
    #[serde(rename = "shippingPhone")]
    pub shipping_phone: String,
    #[serde(rename = "summaryCommodityCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_commodity_code: Option<SummaryCommodityCode>,
    /// Array of line items included in the invoice.
    pub items: Vec<BillItem>,
    #[serde(rename = "Customer")]
    pub customer: PayorDataResponse,
    #[serde(rename = "paylinkId")]
    pub paylink_id: String,
    #[serde(rename = "billEvents")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_events: Option<BillEvents>,
    /// Object with options for scheduled invoices.
    #[serde(rename = "scheduledOptions")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled_options: Option<BillOptions>,
    /// Paypoint's legal name.
    #[serde(rename = "PaypointLegalname")]
    pub paypoint_legalname: Legalname,
    /// Paypoint's DBA name.
    #[serde(rename = "PaypointDbaname")]
    pub paypoint_dbaname: Dbaname,
    /// Paypoint's entryname.
    #[serde(rename = "PaypointEntryname")]
    pub paypoint_entryname: Entrypointfield,
    #[serde(rename = "ParentOrgId")]
    pub parent_org_id: Orgid,
    #[serde(rename = "ParentOrgName")]
    pub parent_org_name: OrgParentName,
    /// Custom list of key:value pairs. This field is used to store any data related to the invoice or for your system.
    #[serde(rename = "AdditionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<HashMap<String, serde_json::Value>>,
    /// Object containing attachments associated to the invoice.
    #[serde(rename = "DocumentsRef")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub documents_ref: Option<DocumentsRef>,
    #[serde(rename = "externalPaypointID")]
    pub external_paypoint_id: ExternalPaypointId,
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
}