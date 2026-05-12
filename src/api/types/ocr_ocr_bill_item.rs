pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OcrBillItem {
    #[serde(rename = "itemTotalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_total_amount: Option<f64>,
    #[serde(rename = "itemTaxAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_tax_amount: Option<f64>,
    #[serde(rename = "itemTaxRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_tax_rate: Option<f64>,
    #[serde(rename = "itemProductCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_product_code: Option<String>,
    #[serde(rename = "itemProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_product_name: Option<String>,
    #[serde(rename = "itemDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_description: Option<String>,
    #[serde(rename = "itemCommodityCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_commodity_code: Option<String>,
    #[serde(rename = "itemUnitOfMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_unit_of_measure: Option<String>,
    #[serde(rename = "itemCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_cost: Option<f64>,
    #[serde(rename = "itemQty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_qty: Option<i64>,
    #[serde(rename = "itemMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_mode: Option<i64>,
    #[serde(rename = "itemCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_categories: Option<Vec<String>>,
}

impl OcrBillItem {
    pub fn builder() -> OcrBillItemBuilder {
        <OcrBillItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OcrBillItemBuilder {
    item_total_amount: Option<f64>,
    item_tax_amount: Option<f64>,
    item_tax_rate: Option<f64>,
    item_product_code: Option<String>,
    item_product_name: Option<String>,
    item_description: Option<String>,
    item_commodity_code: Option<String>,
    item_unit_of_measure: Option<String>,
    item_cost: Option<f64>,
    item_qty: Option<i64>,
    item_mode: Option<i64>,
    item_categories: Option<Vec<String>>,
}

impl OcrBillItemBuilder {
    pub fn item_total_amount(mut self, value: f64) -> Self {
        self.item_total_amount = Some(value);
        self
    }

    pub fn item_tax_amount(mut self, value: f64) -> Self {
        self.item_tax_amount = Some(value);
        self
    }

    pub fn item_tax_rate(mut self, value: f64) -> Self {
        self.item_tax_rate = Some(value);
        self
    }

    pub fn item_product_code(mut self, value: impl Into<String>) -> Self {
        self.item_product_code = Some(value.into());
        self
    }

    pub fn item_product_name(mut self, value: impl Into<String>) -> Self {
        self.item_product_name = Some(value.into());
        self
    }

    pub fn item_description(mut self, value: impl Into<String>) -> Self {
        self.item_description = Some(value.into());
        self
    }

    pub fn item_commodity_code(mut self, value: impl Into<String>) -> Self {
        self.item_commodity_code = Some(value.into());
        self
    }

    pub fn item_unit_of_measure(mut self, value: impl Into<String>) -> Self {
        self.item_unit_of_measure = Some(value.into());
        self
    }

    pub fn item_cost(mut self, value: f64) -> Self {
        self.item_cost = Some(value);
        self
    }

    pub fn item_qty(mut self, value: i64) -> Self {
        self.item_qty = Some(value);
        self
    }

    pub fn item_mode(mut self, value: i64) -> Self {
        self.item_mode = Some(value);
        self
    }

    pub fn item_categories(mut self, value: Vec<String>) -> Self {
        self.item_categories = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OcrBillItem`].
    pub fn build(self) -> Result<OcrBillItem, BuildError> {
        Ok(OcrBillItem {
            item_total_amount: self.item_total_amount,
            item_tax_amount: self.item_tax_amount,
            item_tax_rate: self.item_tax_rate,
            item_product_code: self.item_product_code,
            item_product_name: self.item_product_name,
            item_description: self.item_description,
            item_commodity_code: self.item_commodity_code,
            item_unit_of_measure: self.item_unit_of_measure,
            item_cost: self.item_cost,
            item_qty: self.item_qty,
            item_mode: self.item_mode,
            item_categories: self.item_categories,
        })
    }
}
