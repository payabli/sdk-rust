pub use crate::prelude::*;

/// Response object for bill details. Contains basic information about a bill.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BillDetailsResponse {
    #[serde(rename = "billId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_id: Option<BillId>,
    /// Lot number of the bill.
    #[serde(rename = "lotNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lot_number: Option<String>,
    /// Custom number identifying the bill.
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
    /// Bill due date in format YYYY-MM-DD or MM/DD/YYYY.
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<NaiveDate>,
    /// Bill date in format YYYY-MM-DD or MM/DD/YYYY.
    #[serde(rename = "invoiceDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_date: Option<NaiveDate>,
    /// Any comments about bill. **For managed payouts, this field has a limit of 100 characters**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<Comments>,
}

impl BillDetailsResponse {
    pub fn builder() -> BillDetailsResponseBuilder {
        <BillDetailsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillDetailsResponseBuilder {
    bill_id: Option<BillId>,
    lot_number: Option<String>,
    invoice_number: Option<InvoiceNumber>,
    net_amount: Option<NetAmountstring>,
    discount: Option<String>,
    due_date: Option<NaiveDate>,
    invoice_date: Option<NaiveDate>,
    comments: Option<Comments>,
}

impl BillDetailsResponseBuilder {
    pub fn bill_id(mut self, value: BillId) -> Self {
        self.bill_id = Some(value);
        self
    }

    pub fn lot_number(mut self, value: impl Into<String>) -> Self {
        self.lot_number = Some(value.into());
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

    pub fn due_date(mut self, value: NaiveDate) -> Self {
        self.due_date = Some(value);
        self
    }

    pub fn invoice_date(mut self, value: NaiveDate) -> Self {
        self.invoice_date = Some(value);
        self
    }

    pub fn comments(mut self, value: Comments) -> Self {
        self.comments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BillDetailsResponse`].
    pub fn build(self) -> Result<BillDetailsResponse, BuildError> {
        Ok(BillDetailsResponse {
            bill_id: self.bill_id,
            lot_number: self.lot_number,
            invoice_number: self.invoice_number,
            net_amount: self.net_amount,
            discount: self.discount,
            due_date: self.due_date,
            invoice_date: self.invoice_date,
            comments: self.comments,
        })
    }
}
