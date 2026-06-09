pub use crate::prelude::*;

/// Bill payload sent when creating or updating a bill.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
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
    /// An array of bill images. Attachments aren't required, but we strongly
    /// recommend including them. Including a bill image can make payouts
    /// smoother and prevent delays. You can include either the Base64-encoded
    /// file content, or you can include a `furl` to a public file. The maximum
    /// file size for image uploads is 30 MB.
    ///
    /// When vendor enrichment is enabled and the first attachment is a PDF,
    /// the invoice is scanned and extracted vendor contact information and
    /// bill details (invoice number, amount due, due date) are merged into
    /// the request. Fields in the request body take precedence over extracted
    /// data. If the scan fails, bill creation proceeds with the original
    /// request data. See the
    /// [vendor enrichment guide](/guides/pay-out-vendor-enrichment-overview)
    /// for details. Contact Payabli to enable this feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Attachments>,
    /// Date of bill. Accepted formats: YYYY-MM-DD, MM/DD/YYYY.
    #[serde(rename = "billDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_date: Option<NaiveDate>,
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
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub discount: Option<f64>,
    /// Due date of bill. Accepted formats: YYYY-MM-DD, MM/DD/YYYY.
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<NaiveDate>,
    /// End date for scheduled bills. Applied only in `Mode` = 1. Accepted
    /// formats: YYYY-MM-DD, MM/DD/YYYY.
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequency>,
    /// Lot number associated with the bill.
    #[serde(rename = "lotNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lot_number: Option<String>,
    /// Bill mode: value `0` for one-time bills, `1` for scheduled bills.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i64>,
    /// Net amount owed in bill. Required when adding a bill.
    #[serde(rename = "netAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub net_amount: Option<f64>,
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
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub total_amount: Option<f64>,
    /// The vendor associated with the bill. Although you can create a vendor
    /// in a create bill request, Payabli recommends creating a vendor
    /// separately and passing a valid `vendorNumber` here. At minimum, the
    /// `vendorNumber` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<BillOutDataVendor>,
}

impl BillOutData {
    pub fn builder() -> BillOutDataBuilder {
        <BillOutDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillOutDataBuilder {
    accounting_field_1: Option<AccountingField>,
    accounting_field_2: Option<AccountingField>,
    additional_data: Option<AdditionalDataString>,
    attachments: Option<Attachments>,
    bill_date: Option<NaiveDate>,
    bill_items: Option<Billitems>,
    bill_number: Option<String>,
    comments: Option<Comments>,
    discount: Option<f64>,
    due_date: Option<NaiveDate>,
    end_date: Option<NaiveDate>,
    frequency: Option<Frequency>,
    lot_number: Option<String>,
    mode: Option<i64>,
    net_amount: Option<f64>,
    scheduled_options: Option<BillOutDataScheduledOptions>,
    status: Option<Billstatus>,
    terms: Option<Terms>,
    total_amount: Option<f64>,
    vendor: Option<BillOutDataVendor>,
}

impl BillOutDataBuilder {
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

    pub fn bill_date(mut self, value: NaiveDate) -> Self {
        self.bill_date = Some(value);
        self
    }

    pub fn bill_items(mut self, value: Billitems) -> Self {
        self.bill_items = Some(value);
        self
    }

    pub fn bill_number(mut self, value: impl Into<String>) -> Self {
        self.bill_number = Some(value.into());
        self
    }

    pub fn comments(mut self, value: Comments) -> Self {
        self.comments = Some(value);
        self
    }

    pub fn discount(mut self, value: f64) -> Self {
        self.discount = Some(value);
        self
    }

    pub fn due_date(mut self, value: NaiveDate) -> Self {
        self.due_date = Some(value);
        self
    }

    pub fn end_date(mut self, value: NaiveDate) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn frequency(mut self, value: Frequency) -> Self {
        self.frequency = Some(value);
        self
    }

    pub fn lot_number(mut self, value: impl Into<String>) -> Self {
        self.lot_number = Some(value.into());
        self
    }

    pub fn mode(mut self, value: i64) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn net_amount(mut self, value: f64) -> Self {
        self.net_amount = Some(value);
        self
    }

    pub fn scheduled_options(mut self, value: BillOutDataScheduledOptions) -> Self {
        self.scheduled_options = Some(value);
        self
    }

    pub fn status(mut self, value: Billstatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn terms(mut self, value: Terms) -> Self {
        self.terms = Some(value);
        self
    }

    pub fn total_amount(mut self, value: f64) -> Self {
        self.total_amount = Some(value);
        self
    }

    pub fn vendor(mut self, value: BillOutDataVendor) -> Self {
        self.vendor = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BillOutData`].
    pub fn build(self) -> Result<BillOutData, BuildError> {
        Ok(BillOutData {
            accounting_field_1: self.accounting_field_1,
            accounting_field_2: self.accounting_field_2,
            additional_data: self.additional_data,
            attachments: self.attachments,
            bill_date: self.bill_date,
            bill_items: self.bill_items,
            bill_number: self.bill_number,
            comments: self.comments,
            discount: self.discount,
            due_date: self.due_date,
            end_date: self.end_date,
            frequency: self.frequency,
            lot_number: self.lot_number,
            mode: self.mode,
            net_amount: self.net_amount,
            scheduled_options: self.scheduled_options,
            status: self.status,
            terms: self.terms,
            total_amount: self.total_amount,
            vendor: self.vendor,
        })
    }
}
