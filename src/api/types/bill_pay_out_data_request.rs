pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BillPayOutDataRequest {
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
    pub due_date: Option<NaiveDate>,
    /// Bill date in format YYYY-MM-DD or MM/DD/YYYY.
    #[serde(rename = "invoiceDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_date: Option<NaiveDate>,
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
    /// Bill image attachment. Send the bill image as Base64-encoded string, or as a publicly accessible link. For full details on using this field with a payout authorization, see [the documentation](/developers/developer-guides/pay-out-manage-payouts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Attachments>,
}

impl BillPayOutDataRequest {
    pub fn builder() -> BillPayOutDataRequestBuilder {
        <BillPayOutDataRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillPayOutDataRequestBuilder {
    bill_id: Option<i64>,
    comments: Option<Comments>,
    due_date: Option<NaiveDate>,
    invoice_date: Option<NaiveDate>,
    invoice_number: Option<InvoiceNumber>,
    net_amount: Option<NetAmountstring>,
    discount: Option<String>,
    terms: Option<Terms>,
    accounting_field_1: Option<AccountingField>,
    accounting_field_2: Option<AccountingField>,
    additional_data: Option<AdditionalDataString>,
    attachments: Option<Attachments>,
}

impl BillPayOutDataRequestBuilder {
    pub fn bill_id(mut self, value: i64) -> Self {
        self.bill_id = Some(value);
        self
    }

    pub fn comments(mut self, value: Comments) -> Self {
        self.comments = Some(value);
        self
    }

    pub fn due_date(mut self, value: NaiveDate) -> Self {
        self.due_date = Some(value);
        self
    }

    pub fn invoice_date(mut self, value: NaiveDate) -> Self {
        self.invoice_date = Some(value);
        self
    }

    pub fn invoice_number(mut self, value: InvoiceNumber) -> Self {
        self.invoice_number = Some(value);
        self
    }

    pub fn net_amount(mut self, value: NetAmountstring) -> Self {
        self.net_amount = Some(value);
        self
    }

    pub fn discount(mut self, value: impl Into<String>) -> Self {
        self.discount = Some(value.into());
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

    /// Consumes the builder and constructs a [`BillPayOutDataRequest`].
    pub fn build(self) -> Result<BillPayOutDataRequest, BuildError> {
        Ok(BillPayOutDataRequest {
            bill_id: self.bill_id,
            comments: self.comments,
            due_date: self.due_date,
            invoice_date: self.invoice_date,
            invoice_number: self.invoice_number,
            net_amount: self.net_amount,
            discount: self.discount,
            terms: self.terms,
            accounting_field_1: self.accounting_field_1,
            accounting_field_2: self.accounting_field_2,
            additional_data: self.additional_data,
            attachments: self.attachments,
        })
    }
}
