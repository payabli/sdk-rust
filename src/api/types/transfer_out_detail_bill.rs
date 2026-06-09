pub use crate::prelude::*;

/// Bill information for an outbound transfer detail.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TransferOutDetailBill {
    /// Unique identifier for the bill.
    #[serde(rename = "billId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_id: Option<i64>,
    /// Lot number.
    #[serde(rename = "LotNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lot_number: Option<String>,
    /// Accounting field 1.
    #[serde(rename = "AccountingField1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_field_1: Option<String>,
    /// Accounting field 2.
    #[serde(rename = "AccountingField2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_field_2: Option<String>,
    /// Payment terms.
    #[serde(rename = "Terms")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms: Option<String>,
    /// Additional data for the bill.
    #[serde(rename = "AdditionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<HashMap<String, serde_json::Value>>,
    /// Attachments for the bill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<TransferOutDetailBillAttachment>>,
    /// Invoice number.
    #[serde(rename = "invoiceNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_number: Option<String>,
    /// Net amount of the bill.
    #[serde(rename = "netAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_amount: Option<String>,
    /// Date of the invoice.
    #[serde(rename = "invoiceDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_date: Option<String>,
    /// Due date for the bill.
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<String>,
    /// Comments on the bill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    /// Identifier for the bill.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Discount applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub discount: Option<f64>,
    /// Total amount of the bill.
    #[serde(rename = "totalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_amount: Option<f64>,
}

impl TransferOutDetailBill {
    pub fn builder() -> TransferOutDetailBillBuilder {
        <TransferOutDetailBillBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferOutDetailBillBuilder {
    bill_id: Option<i64>,
    lot_number: Option<String>,
    accounting_field_1: Option<String>,
    accounting_field_2: Option<String>,
    terms: Option<String>,
    additional_data: Option<HashMap<String, serde_json::Value>>,
    attachments: Option<Vec<TransferOutDetailBillAttachment>>,
    invoice_number: Option<String>,
    net_amount: Option<String>,
    invoice_date: Option<String>,
    due_date: Option<String>,
    comments: Option<String>,
    identifier: Option<String>,
    discount: Option<f64>,
    total_amount: Option<f64>,
}

impl TransferOutDetailBillBuilder {
    pub fn bill_id(mut self, value: i64) -> Self {
        self.bill_id = Some(value);
        self
    }

    pub fn lot_number(mut self, value: impl Into<String>) -> Self {
        self.lot_number = Some(value.into());
        self
    }

    pub fn accounting_field_1(mut self, value: impl Into<String>) -> Self {
        self.accounting_field_1 = Some(value.into());
        self
    }

    pub fn accounting_field_2(mut self, value: impl Into<String>) -> Self {
        self.accounting_field_2 = Some(value.into());
        self
    }

    pub fn terms(mut self, value: impl Into<String>) -> Self {
        self.terms = Some(value.into());
        self
    }

    pub fn additional_data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.additional_data = Some(value);
        self
    }

    pub fn attachments(mut self, value: Vec<TransferOutDetailBillAttachment>) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn invoice_number(mut self, value: impl Into<String>) -> Self {
        self.invoice_number = Some(value.into());
        self
    }

    pub fn net_amount(mut self, value: impl Into<String>) -> Self {
        self.net_amount = Some(value.into());
        self
    }

    pub fn invoice_date(mut self, value: impl Into<String>) -> Self {
        self.invoice_date = Some(value.into());
        self
    }

    pub fn due_date(mut self, value: impl Into<String>) -> Self {
        self.due_date = Some(value.into());
        self
    }

    pub fn comments(mut self, value: impl Into<String>) -> Self {
        self.comments = Some(value.into());
        self
    }

    pub fn identifier(mut self, value: impl Into<String>) -> Self {
        self.identifier = Some(value.into());
        self
    }

    pub fn discount(mut self, value: f64) -> Self {
        self.discount = Some(value);
        self
    }

    pub fn total_amount(mut self, value: f64) -> Self {
        self.total_amount = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransferOutDetailBill`].
    pub fn build(self) -> Result<TransferOutDetailBill, BuildError> {
        Ok(TransferOutDetailBill {
            bill_id: self.bill_id,
            lot_number: self.lot_number,
            accounting_field_1: self.accounting_field_1,
            accounting_field_2: self.accounting_field_2,
            terms: self.terms,
            additional_data: self.additional_data,
            attachments: self.attachments,
            invoice_number: self.invoice_number,
            net_amount: self.net_amount,
            invoice_date: self.invoice_date,
            due_date: self.due_date,
            comments: self.comments,
            identifier: self.identifier,
            discount: self.discount,
            total_amount: self.total_amount,
        })
    }
}
