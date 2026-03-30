pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
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
    pub invoice_date: Option<NaiveDate>,
    /// Invoice due date in one of the accepted formats: YYYY-MM-DD, MM/DD/YYYY.
    #[serde(rename = "invoiceDueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_due_date: Option<NaiveDate>,
    /// Indicate the date to finish a scheduled invoice cycle (`invoiceType`` = 1) in any of the accepted formats: YYYY-MM-DD, MM/DD/YYYY.
    #[serde(rename = "invoiceEndDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_end_date: Option<NaiveDate>,
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

impl BillData {
    pub fn builder() -> BillDataBuilder {
        <BillDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillDataBuilder {
    additional_data: Option<AdditionalDataMap>,
    attachments: Option<Attachments>,
    company: Option<String>,
    discount: Option<Discount>,
    duty_amount: Option<DutyAmount>,
    first_name: Option<String>,
    freight_amount: Option<FreightAmount>,
    frequency: Option<Frequency>,
    invoice_amount: Option<InvoiceAmount>,
    invoice_date: Option<NaiveDate>,
    invoice_due_date: Option<NaiveDate>,
    invoice_end_date: Option<NaiveDate>,
    invoice_number: Option<InvoiceNumber>,
    invoice_status: Option<Invoicestatus>,
    invoice_type: Option<InvoiceType>,
    items: Option<Vec<BillItem>>,
    last_name: Option<String>,
    notes: Option<String>,
    payment_terms: Option<BillDataPaymentTerms>,
    purchase_order: Option<PurchaseOrder>,
    shipping_address_1: Option<Shippingaddress>,
    shipping_address_2: Option<Shippingaddressadditional>,
    shipping_city: Option<Shippingcity>,
    shipping_country: Option<Shippingcountry>,
    shipping_email: Option<Email>,
    shipping_from_zip: Option<ShippingFromZip>,
    shipping_phone: Option<String>,
    shipping_state: Option<Shippingstate>,
    shipping_zip: Option<Shippingzip>,
    summary_commodity_code: Option<SummaryCommodityCode>,
    tax: Option<Tax>,
    terms_conditions: Option<TermsConditions>,
}

impl BillDataBuilder {
    pub fn additional_data(mut self, value: AdditionalDataMap) -> Self {
        self.additional_data = Some(value);
        self
    }

    pub fn attachments(mut self, value: Attachments) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn company(mut self, value: impl Into<String>) -> Self {
        self.company = Some(value.into());
        self
    }

    pub fn discount(mut self, value: Discount) -> Self {
        self.discount = Some(value);
        self
    }

    pub fn duty_amount(mut self, value: DutyAmount) -> Self {
        self.duty_amount = Some(value);
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn freight_amount(mut self, value: FreightAmount) -> Self {
        self.freight_amount = Some(value);
        self
    }

    pub fn frequency(mut self, value: Frequency) -> Self {
        self.frequency = Some(value);
        self
    }

    pub fn invoice_amount(mut self, value: InvoiceAmount) -> Self {
        self.invoice_amount = Some(value);
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

    pub fn invoice_end_date(mut self, value: NaiveDate) -> Self {
        self.invoice_end_date = Some(value);
        self
    }

    pub fn invoice_number(mut self, value: InvoiceNumber) -> Self {
        self.invoice_number = Some(value);
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

    pub fn items(mut self, value: Vec<BillItem>) -> Self {
        self.items = Some(value);
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn notes(mut self, value: impl Into<String>) -> Self {
        self.notes = Some(value.into());
        self
    }

    pub fn payment_terms(mut self, value: BillDataPaymentTerms) -> Self {
        self.payment_terms = Some(value);
        self
    }

    pub fn purchase_order(mut self, value: PurchaseOrder) -> Self {
        self.purchase_order = Some(value);
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

    pub fn shipping_country(mut self, value: Shippingcountry) -> Self {
        self.shipping_country = Some(value);
        self
    }

    pub fn shipping_email(mut self, value: Email) -> Self {
        self.shipping_email = Some(value);
        self
    }

    pub fn shipping_from_zip(mut self, value: ShippingFromZip) -> Self {
        self.shipping_from_zip = Some(value);
        self
    }

    pub fn shipping_phone(mut self, value: impl Into<String>) -> Self {
        self.shipping_phone = Some(value.into());
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

    pub fn summary_commodity_code(mut self, value: SummaryCommodityCode) -> Self {
        self.summary_commodity_code = Some(value);
        self
    }

    pub fn tax(mut self, value: Tax) -> Self {
        self.tax = Some(value);
        self
    }

    pub fn terms_conditions(mut self, value: TermsConditions) -> Self {
        self.terms_conditions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BillData`].
    pub fn build(self) -> Result<BillData, BuildError> {
        Ok(BillData {
            additional_data: self.additional_data,
            attachments: self.attachments,
            company: self.company,
            discount: self.discount,
            duty_amount: self.duty_amount,
            first_name: self.first_name,
            freight_amount: self.freight_amount,
            frequency: self.frequency,
            invoice_amount: self.invoice_amount,
            invoice_date: self.invoice_date,
            invoice_due_date: self.invoice_due_date,
            invoice_end_date: self.invoice_end_date,
            invoice_number: self.invoice_number,
            invoice_status: self.invoice_status,
            invoice_type: self.invoice_type,
            items: self.items,
            last_name: self.last_name,
            notes: self.notes,
            payment_terms: self.payment_terms,
            purchase_order: self.purchase_order,
            shipping_address_1: self.shipping_address_1,
            shipping_address_2: self.shipping_address_2,
            shipping_city: self.shipping_city,
            shipping_country: self.shipping_country,
            shipping_email: self.shipping_email,
            shipping_from_zip: self.shipping_from_zip,
            shipping_phone: self.shipping_phone,
            shipping_state: self.shipping_state,
            shipping_zip: self.shipping_zip,
            summary_commodity_code: self.summary_commodity_code,
            tax: self.tax,
            terms_conditions: self.terms_conditions,
        })
    }
}
