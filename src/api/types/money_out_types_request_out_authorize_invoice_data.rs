pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
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
    pub invoice_date: Option<NaiveDate>,
    /// Invoice due date in any of the accepted formats: YYYY-MM-DD, MM/DD/YYYY.
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<NaiveDate>,
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

impl RequestOutAuthorizeInvoiceData {
    pub fn builder() -> RequestOutAuthorizeInvoiceDataBuilder {
        <RequestOutAuthorizeInvoiceDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequestOutAuthorizeInvoiceDataBuilder {
    invoice_number: Option<InvoiceNumber>,
    net_amount: Option<NetAmountstring>,
    invoice_date: Option<NaiveDate>,
    due_date: Option<NaiveDate>,
    comments: Option<Comments>,
    lot_number: Option<LotNumber>,
    bill_id: Option<BillId>,
    discount: Option<Discount>,
    terms: Option<Terms>,
    accounting_field_1: Option<AccountingField>,
    accounting_field_2: Option<AccountingField>,
    additional_data: Option<AdditionalDataString>,
    attachments: Option<Attachments>,
}

impl RequestOutAuthorizeInvoiceDataBuilder {
    pub fn invoice_number(mut self, value: InvoiceNumber) -> Self {
        self.invoice_number = Some(value);
        self
    }

    pub fn net_amount(mut self, value: NetAmountstring) -> Self {
        self.net_amount = Some(value);
        self
    }

    pub fn invoice_date(mut self, value: NaiveDate) -> Self {
        self.invoice_date = Some(value);
        self
    }

    pub fn due_date(mut self, value: NaiveDate) -> Self {
        self.due_date = Some(value);
        self
    }

    pub fn comments(mut self, value: Comments) -> Self {
        self.comments = Some(value);
        self
    }

    pub fn lot_number(mut self, value: LotNumber) -> Self {
        self.lot_number = Some(value);
        self
    }

    pub fn bill_id(mut self, value: BillId) -> Self {
        self.bill_id = Some(value);
        self
    }

    pub fn discount(mut self, value: Discount) -> Self {
        self.discount = Some(value);
        self
    }

    pub fn terms(mut self, value: Terms) -> Self {
        self.terms = Some(value);
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

    pub fn additional_data(mut self, value: AdditionalDataString) -> Self {
        self.additional_data = Some(value);
        self
    }

    pub fn attachments(mut self, value: Attachments) -> Self {
        self.attachments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RequestOutAuthorizeInvoiceData`].
    pub fn build(self) -> Result<RequestOutAuthorizeInvoiceData, BuildError> {
        Ok(RequestOutAuthorizeInvoiceData {
            invoice_number: self.invoice_number,
            net_amount: self.net_amount,
            invoice_date: self.invoice_date,
            due_date: self.due_date,
            comments: self.comments,
            lot_number: self.lot_number,
            bill_id: self.bill_id,
            discount: self.discount,
            terms: self.terms,
            accounting_field_1: self.accounting_field_1,
            accounting_field_2: self.accounting_field_2,
            additional_data: self.additional_data,
            attachments: self.attachments,
        })
    }
}
