pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OcrResultData {
    #[serde(rename = "billNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_number: Option<String>,
    #[serde(rename = "netAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub net_amount: Option<f64>,
    #[serde(rename = "billDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub bill_date: Option<DateTime<Utc>>,
    #[serde(rename = "dueDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub due_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(rename = "billItems")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_items: Option<Vec<OcrBillItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i64>,
    #[serde(rename = "accountingField1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_field_1: Option<String>,
    #[serde(rename = "accountingField2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accounting_field_2: Option<String>,
    #[serde(rename = "additionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<OcrBillItemAdditionalData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor: Option<OcrVendor>,
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub end_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<i64>,
    #[serde(rename = "lotNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lot_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attachments: Option<Vec<OcrAttachment>>,
}

impl OcrResultData {
    pub fn builder() -> OcrResultDataBuilder {
        <OcrResultDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OcrResultDataBuilder {
    bill_number: Option<String>,
    net_amount: Option<f64>,
    bill_date: Option<DateTime<Utc>>,
    due_date: Option<DateTime<Utc>>,
    comments: Option<String>,
    bill_items: Option<Vec<OcrBillItem>>,
    mode: Option<i64>,
    accounting_field_1: Option<String>,
    accounting_field_2: Option<String>,
    additional_data: Option<OcrBillItemAdditionalData>,
    vendor: Option<OcrVendor>,
    end_date: Option<DateTime<Utc>>,
    frequency: Option<String>,
    terms: Option<String>,
    status: Option<i64>,
    lot_number: Option<String>,
    attachments: Option<Vec<OcrAttachment>>,
}

impl OcrResultDataBuilder {
    pub fn bill_number(mut self, value: impl Into<String>) -> Self {
        self.bill_number = Some(value.into());
        self
    }

    pub fn net_amount(mut self, value: f64) -> Self {
        self.net_amount = Some(value);
        self
    }

    pub fn bill_date(mut self, value: DateTime<Utc>) -> Self {
        self.bill_date = Some(value);
        self
    }

    pub fn due_date(mut self, value: DateTime<Utc>) -> Self {
        self.due_date = Some(value);
        self
    }

    pub fn comments(mut self, value: impl Into<String>) -> Self {
        self.comments = Some(value.into());
        self
    }

    pub fn bill_items(mut self, value: Vec<OcrBillItem>) -> Self {
        self.bill_items = Some(value);
        self
    }

    pub fn mode(mut self, value: i64) -> Self {
        self.mode = Some(value);
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

    pub fn additional_data(mut self, value: OcrBillItemAdditionalData) -> Self {
        self.additional_data = Some(value);
        self
    }

    pub fn vendor(mut self, value: OcrVendor) -> Self {
        self.vendor = Some(value);
        self
    }

    pub fn end_date(mut self, value: DateTime<Utc>) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn frequency(mut self, value: impl Into<String>) -> Self {
        self.frequency = Some(value.into());
        self
    }

    pub fn terms(mut self, value: impl Into<String>) -> Self {
        self.terms = Some(value.into());
        self
    }

    pub fn status(mut self, value: i64) -> Self {
        self.status = Some(value);
        self
    }

    pub fn lot_number(mut self, value: impl Into<String>) -> Self {
        self.lot_number = Some(value.into());
        self
    }

    pub fn attachments(mut self, value: Vec<OcrAttachment>) -> Self {
        self.attachments = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OcrResultData`].
    pub fn build(self) -> Result<OcrResultData, BuildError> {
        Ok(OcrResultData {
            bill_number: self.bill_number,
            net_amount: self.net_amount,
            bill_date: self.bill_date,
            due_date: self.due_date,
            comments: self.comments,
            bill_items: self.bill_items,
            mode: self.mode,
            accounting_field_1: self.accounting_field_1,
            accounting_field_2: self.accounting_field_2,
            additional_data: self.additional_data,
            vendor: self.vendor,
            end_date: self.end_date,
            frequency: self.frequency,
            terms: self.terms,
            status: self.status,
            lot_number: self.lot_number,
            attachments: self.attachments,
        })
    }
}
