pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryInvoiceResponseRecordsItem {
    #[serde(rename = "invoiceId")]
    #[serde(default)]
    pub invoice_id: InvoiceId,
    #[serde(rename = "customerId")]
    #[serde(default)]
    pub customer_id: CustomerId,
    #[serde(rename = "paypointId")]
    #[serde(default)]
    pub paypoint_id: PaypointId,
    #[serde(rename = "invoiceNumber")]
    #[serde(default)]
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
    #[serde(default)]
    pub created_at: CreatedAt,
    #[serde(rename = "invoiceStatus")]
    #[serde(default)]
    pub invoice_status: Invoicestatus,
    #[serde(rename = "invoiceType")]
    #[serde(default)]
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
    #[serde(default)]
    pub invoice_amount: InvoiceAmount,
    #[serde(rename = "invoicePaidAmount")]
    #[serde(default)]
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
    #[serde(default)]
    pub first_name: String,
    /// Last name of the recipient of the invoice.
    #[serde(rename = "lastName")]
    #[serde(default)]
    pub last_name: String,
    /// Company name of the recipient of the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<String>,
    #[serde(rename = "shippingAddress1")]
    #[serde(default)]
    pub shipping_address_1: Shippingaddress,
    #[serde(rename = "shippingAddress2")]
    #[serde(default)]
    pub shipping_address_2: Shippingaddressadditional,
    #[serde(rename = "shippingCity")]
    #[serde(default)]
    pub shipping_city: Shippingcity,
    #[serde(rename = "shippingState")]
    #[serde(default)]
    pub shipping_state: Shippingstate,
    #[serde(rename = "shippingZip")]
    #[serde(default)]
    pub shipping_zip: Shippingzip,
    #[serde(rename = "shippingFromZip")]
    #[serde(default)]
    pub shipping_from_zip: ShippingFromZip,
    #[serde(rename = "shippingCountry")]
    #[serde(default)]
    pub shipping_country: Shippingcountry,
    /// Shipping recipient's contact email address.
    #[serde(rename = "shippingEmail")]
    #[serde(default)]
    pub shipping_email: Email,
    /// Recipient phone number.
    #[serde(rename = "shippingPhone")]
    #[serde(default)]
    pub shipping_phone: String,
    #[serde(rename = "summaryCommodityCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary_commodity_code: Option<SummaryCommodityCode>,
    /// Array of line items included in the invoice.
    #[serde(default)]
    pub items: Vec<BillItem>,
    #[serde(rename = "Customer")]
    #[serde(default)]
    pub customer: PayorDataResponse,
    #[serde(rename = "paylinkId")]
    #[serde(default)]
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
    #[serde(default)]
    pub paypoint_legalname: Legalname,
    /// Paypoint's DBA name.
    #[serde(rename = "PaypointDbaname")]
    #[serde(default)]
    pub paypoint_dbaname: Dbaname,
    /// Paypoint's entryname.
    #[serde(rename = "PaypointEntryname")]
    #[serde(default)]
    pub paypoint_entryname: Entrypointfield,
    #[serde(rename = "ParentOrgId")]
    #[serde(default)]
    pub parent_org_id: Orgid,
    #[serde(rename = "ParentOrgName")]
    #[serde(default)]
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
    #[serde(default)]
    pub external_paypoint_id: ExternalPaypointId,
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
}

impl QueryInvoiceResponseRecordsItem {
    pub fn builder() -> QueryInvoiceResponseRecordsItemBuilder {
        <QueryInvoiceResponseRecordsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryInvoiceResponseRecordsItemBuilder {
    invoice_id: Option<InvoiceId>,
    customer_id: Option<CustomerId>,
    paypoint_id: Option<PaypointId>,
    invoice_number: Option<InvoiceNumber>,
    invoice_date: Option<NaiveDate>,
    invoice_due_date: Option<NaiveDate>,
    invoice_sent_date: Option<NaiveDate>,
    invoice_end_date: Option<NaiveDate>,
    last_payment_date: Option<DateTime<Utc>>,
    created_at: Option<CreatedAt>,
    invoice_status: Option<Invoicestatus>,
    invoice_type: Option<InvoiceType>,
    frequency: Option<Frequency>,
    payment_terms: Option<Terms>,
    terms_conditions: Option<TermsConditions>,
    notes: Option<String>,
    tax: Option<Tax>,
    discount: Option<Discount>,
    invoice_amount: Option<InvoiceAmount>,
    invoice_paid_amount: Option<InvoicePaidAmount>,
    freight_amount: Option<FreightAmount>,
    duty_amount: Option<DutyAmount>,
    purchase_order: Option<PurchaseOrder>,
    first_name: Option<String>,
    last_name: Option<String>,
    company: Option<String>,
    shipping_address_1: Option<Shippingaddress>,
    shipping_address_2: Option<Shippingaddressadditional>,
    shipping_city: Option<Shippingcity>,
    shipping_state: Option<Shippingstate>,
    shipping_zip: Option<Shippingzip>,
    shipping_from_zip: Option<ShippingFromZip>,
    shipping_country: Option<Shippingcountry>,
    shipping_email: Option<Email>,
    shipping_phone: Option<String>,
    summary_commodity_code: Option<SummaryCommodityCode>,
    items: Option<Vec<BillItem>>,
    customer: Option<PayorDataResponse>,
    paylink_id: Option<String>,
    bill_events: Option<BillEvents>,
    scheduled_options: Option<BillOptions>,
    paypoint_legalname: Option<Legalname>,
    paypoint_dbaname: Option<Dbaname>,
    paypoint_entryname: Option<Entrypointfield>,
    parent_org_id: Option<Orgid>,
    parent_org_name: Option<OrgParentName>,
    additional_data: Option<HashMap<String, serde_json::Value>>,
    documents_ref: Option<DocumentsRef>,
    external_paypoint_id: Option<ExternalPaypointId>,
    page_identifier: Option<PageIdentifier>,
}

impl QueryInvoiceResponseRecordsItemBuilder {
    pub fn invoice_id(mut self, value: InvoiceId) -> Self {
        self.invoice_id = Some(value);
        self
    }

    pub fn customer_id(mut self, value: CustomerId) -> Self {
        self.customer_id = Some(value);
        self
    }

    pub fn paypoint_id(mut self, value: PaypointId) -> Self {
        self.paypoint_id = Some(value);
        self
    }

    pub fn invoice_number(mut self, value: InvoiceNumber) -> Self {
        self.invoice_number = Some(value);
        self
    }

    pub fn invoice_date(mut self, value: NaiveDate) -> Self {
        self.invoice_date = Some(value);
        self
    }

    pub fn invoice_due_date(mut self, value: NaiveDate) -> Self {
        self.invoice_due_date = Some(value);
        self
    }

    pub fn invoice_sent_date(mut self, value: NaiveDate) -> Self {
        self.invoice_sent_date = Some(value);
        self
    }

    pub fn invoice_end_date(mut self, value: NaiveDate) -> Self {
        self.invoice_end_date = Some(value);
        self
    }

    pub fn last_payment_date(mut self, value: DateTime<Utc>) -> Self {
        self.last_payment_date = Some(value);
        self
    }

    pub fn created_at(mut self, value: CreatedAt) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn invoice_status(mut self, value: Invoicestatus) -> Self {
        self.invoice_status = Some(value);
        self
    }

    pub fn invoice_type(mut self, value: InvoiceType) -> Self {
        self.invoice_type = Some(value);
        self
    }

    pub fn frequency(mut self, value: Frequency) -> Self {
        self.frequency = Some(value);
        self
    }

    pub fn payment_terms(mut self, value: Terms) -> Self {
        self.payment_terms = Some(value);
        self
    }

    pub fn terms_conditions(mut self, value: TermsConditions) -> Self {
        self.terms_conditions = Some(value);
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn tax(mut self, value: Tax) -> Self {
        self.tax = Some(value);
        self
    }

    pub fn discount(mut self, value: Discount) -> Self {
        self.discount = Some(value);
        self
    }

    pub fn invoice_amount(mut self, value: InvoiceAmount) -> Self {
        self.invoice_amount = Some(value);
        self
    }

    pub fn invoice_paid_amount(mut self, value: InvoicePaidAmount) -> Self {
        self.invoice_paid_amount = Some(value);
        self
    }

    pub fn freight_amount(mut self, value: FreightAmount) -> Self {
        self.freight_amount = Some(value);
        self
    }

    pub fn duty_amount(mut self, value: DutyAmount) -> Self {
        self.duty_amount = Some(value);
        self
    }

    pub fn purchase_order(mut self, value: PurchaseOrder) -> Self {
        self.purchase_order = Some(value);
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn company(mut self, value: impl Into<String>) -> Self {
        self.company = Some(value.into());
        self
    }

    pub fn shipping_address_1(mut self, value: Shippingaddress) -> Self {
        self.shipping_address_1 = Some(value);
        self
    }

    pub fn shipping_address_2(mut self, value: Shippingaddressadditional) -> Self {
        self.shipping_address_2 = Some(value);
        self
    }

    pub fn shipping_city(mut self, value: Shippingcity) -> Self {
        self.shipping_city = Some(value);
        self
    }

    pub fn shipping_state(mut self, value: Shippingstate) -> Self {
        self.shipping_state = Some(value);
        self
    }

    pub fn shipping_zip(mut self, value: Shippingzip) -> Self {
        self.shipping_zip = Some(value);
        self
    }

    pub fn shipping_from_zip(mut self, value: ShippingFromZip) -> Self {
        self.shipping_from_zip = Some(value);
        self
    }

    pub fn shipping_country(mut self, value: Shippingcountry) -> Self {
        self.shipping_country = Some(value);
        self
    }

    pub fn shipping_email(mut self, value: Email) -> Self {
        self.shipping_email = Some(value);
        self
    }

    pub fn shipping_phone(mut self, value: impl Into<String>) -> Self {
        self.shipping_phone = Some(value.into());
        self
    }

    pub fn summary_commodity_code(mut self, value: SummaryCommodityCode) -> Self {
        self.summary_commodity_code = Some(value);
        self
    }

    pub fn items(mut self, value: Vec<BillItem>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn customer(mut self, value: PayorDataResponse) -> Self {
        self.customer = Some(value);
        self
    }

    pub fn paylink_id(mut self, value: impl Into<String>) -> Self {
        self.paylink_id = Some(value.into());
        self
    }

    pub fn bill_events(mut self, value: BillEvents) -> Self {
        self.bill_events = Some(value);
        self
    }

    pub fn scheduled_options(mut self, value: BillOptions) -> Self {
        self.scheduled_options = Some(value);
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

    pub fn parent_org_id(mut self, value: Orgid) -> Self {
        self.parent_org_id = Some(value);
        self
    }

    pub fn parent_org_name(mut self, value: OrgParentName) -> Self {
        self.parent_org_name = Some(value);
        self
    }

    pub fn additional_data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.additional_data = Some(value);
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

    pub fn page_identifier(mut self, value: PageIdentifier) -> Self {
        self.page_identifier = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryInvoiceResponseRecordsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`invoice_id`](QueryInvoiceResponseRecordsItemBuilder::invoice_id)
    /// - [`customer_id`](QueryInvoiceResponseRecordsItemBuilder::customer_id)
    /// - [`paypoint_id`](QueryInvoiceResponseRecordsItemBuilder::paypoint_id)
    /// - [`invoice_number`](QueryInvoiceResponseRecordsItemBuilder::invoice_number)
    /// - [`created_at`](QueryInvoiceResponseRecordsItemBuilder::created_at)
    /// - [`invoice_status`](QueryInvoiceResponseRecordsItemBuilder::invoice_status)
    /// - [`invoice_type`](QueryInvoiceResponseRecordsItemBuilder::invoice_type)
    /// - [`frequency`](QueryInvoiceResponseRecordsItemBuilder::frequency)
    /// - [`payment_terms`](QueryInvoiceResponseRecordsItemBuilder::payment_terms)
    /// - [`invoice_amount`](QueryInvoiceResponseRecordsItemBuilder::invoice_amount)
    /// - [`invoice_paid_amount`](QueryInvoiceResponseRecordsItemBuilder::invoice_paid_amount)
    /// - [`first_name`](QueryInvoiceResponseRecordsItemBuilder::first_name)
    /// - [`last_name`](QueryInvoiceResponseRecordsItemBuilder::last_name)
    /// - [`shipping_address_1`](QueryInvoiceResponseRecordsItemBuilder::shipping_address_1)
    /// - [`shipping_address_2`](QueryInvoiceResponseRecordsItemBuilder::shipping_address_2)
    /// - [`shipping_city`](QueryInvoiceResponseRecordsItemBuilder::shipping_city)
    /// - [`shipping_state`](QueryInvoiceResponseRecordsItemBuilder::shipping_state)
    /// - [`shipping_zip`](QueryInvoiceResponseRecordsItemBuilder::shipping_zip)
    /// - [`shipping_from_zip`](QueryInvoiceResponseRecordsItemBuilder::shipping_from_zip)
    /// - [`shipping_country`](QueryInvoiceResponseRecordsItemBuilder::shipping_country)
    /// - [`shipping_email`](QueryInvoiceResponseRecordsItemBuilder::shipping_email)
    /// - [`shipping_phone`](QueryInvoiceResponseRecordsItemBuilder::shipping_phone)
    /// - [`items`](QueryInvoiceResponseRecordsItemBuilder::items)
    /// - [`customer`](QueryInvoiceResponseRecordsItemBuilder::customer)
    /// - [`paylink_id`](QueryInvoiceResponseRecordsItemBuilder::paylink_id)
    /// - [`paypoint_legalname`](QueryInvoiceResponseRecordsItemBuilder::paypoint_legalname)
    /// - [`paypoint_dbaname`](QueryInvoiceResponseRecordsItemBuilder::paypoint_dbaname)
    /// - [`paypoint_entryname`](QueryInvoiceResponseRecordsItemBuilder::paypoint_entryname)
    /// - [`parent_org_id`](QueryInvoiceResponseRecordsItemBuilder::parent_org_id)
    /// - [`parent_org_name`](QueryInvoiceResponseRecordsItemBuilder::parent_org_name)
    /// - [`external_paypoint_id`](QueryInvoiceResponseRecordsItemBuilder::external_paypoint_id)
    pub fn build(self) -> Result<QueryInvoiceResponseRecordsItem, BuildError> {
        Ok(QueryInvoiceResponseRecordsItem {
            invoice_id: self
                .invoice_id
                .ok_or_else(|| BuildError::missing_field("invoice_id"))?,
            customer_id: self
                .customer_id
                .ok_or_else(|| BuildError::missing_field("customer_id"))?,
            paypoint_id: self
                .paypoint_id
                .ok_or_else(|| BuildError::missing_field("paypoint_id"))?,
            invoice_number: self
                .invoice_number
                .ok_or_else(|| BuildError::missing_field("invoice_number"))?,
            invoice_date: self.invoice_date,
            invoice_due_date: self.invoice_due_date,
            invoice_sent_date: self.invoice_sent_date,
            invoice_end_date: self.invoice_end_date,
            last_payment_date: self.last_payment_date,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            invoice_status: self
                .invoice_status
                .ok_or_else(|| BuildError::missing_field("invoice_status"))?,
            invoice_type: self
                .invoice_type
                .ok_or_else(|| BuildError::missing_field("invoice_type"))?,
            frequency: self
                .frequency
                .ok_or_else(|| BuildError::missing_field("frequency"))?,
            payment_terms: self
                .payment_terms
                .ok_or_else(|| BuildError::missing_field("payment_terms"))?,
            terms_conditions: self.terms_conditions,
            notes: self.notes,
            tax: self.tax,
            discount: self.discount,
            invoice_amount: self
                .invoice_amount
                .ok_or_else(|| BuildError::missing_field("invoice_amount"))?,
            invoice_paid_amount: self
                .invoice_paid_amount
                .ok_or_else(|| BuildError::missing_field("invoice_paid_amount"))?,
            freight_amount: self.freight_amount,
            duty_amount: self.duty_amount,
            purchase_order: self.purchase_order,
            first_name: self
                .first_name
                .ok_or_else(|| BuildError::missing_field("first_name"))?,
            last_name: self
                .last_name
                .ok_or_else(|| BuildError::missing_field("last_name"))?,
            company: self.company,
            shipping_address_1: self
                .shipping_address_1
                .ok_or_else(|| BuildError::missing_field("shipping_address_1"))?,
            shipping_address_2: self
                .shipping_address_2
                .ok_or_else(|| BuildError::missing_field("shipping_address_2"))?,
            shipping_city: self
                .shipping_city
                .ok_or_else(|| BuildError::missing_field("shipping_city"))?,
            shipping_state: self
                .shipping_state
                .ok_or_else(|| BuildError::missing_field("shipping_state"))?,
            shipping_zip: self
                .shipping_zip
                .ok_or_else(|| BuildError::missing_field("shipping_zip"))?,
            shipping_from_zip: self
                .shipping_from_zip
                .ok_or_else(|| BuildError::missing_field("shipping_from_zip"))?,
            shipping_country: self
                .shipping_country
                .ok_or_else(|| BuildError::missing_field("shipping_country"))?,
            shipping_email: self
                .shipping_email
                .ok_or_else(|| BuildError::missing_field("shipping_email"))?,
            shipping_phone: self
                .shipping_phone
                .ok_or_else(|| BuildError::missing_field("shipping_phone"))?,
            summary_commodity_code: self.summary_commodity_code,
            items: self
                .items
                .ok_or_else(|| BuildError::missing_field("items"))?,
            customer: self
                .customer
                .ok_or_else(|| BuildError::missing_field("customer"))?,
            paylink_id: self
                .paylink_id
                .ok_or_else(|| BuildError::missing_field("paylink_id"))?,
            bill_events: self.bill_events,
            scheduled_options: self.scheduled_options,
            paypoint_legalname: self
                .paypoint_legalname
                .ok_or_else(|| BuildError::missing_field("paypoint_legalname"))?,
            paypoint_dbaname: self
                .paypoint_dbaname
                .ok_or_else(|| BuildError::missing_field("paypoint_dbaname"))?,
            paypoint_entryname: self
                .paypoint_entryname
                .ok_or_else(|| BuildError::missing_field("paypoint_entryname"))?,
            parent_org_id: self
                .parent_org_id
                .ok_or_else(|| BuildError::missing_field("parent_org_id"))?,
            parent_org_name: self
                .parent_org_name
                .ok_or_else(|| BuildError::missing_field("parent_org_name"))?,
            additional_data: self.additional_data,
            documents_ref: self.documents_ref,
            external_paypoint_id: self
                .external_paypoint_id
                .ok_or_else(|| BuildError::missing_field("external_paypoint_id"))?,
            page_identifier: self.page_identifier,
        })
    }
}
