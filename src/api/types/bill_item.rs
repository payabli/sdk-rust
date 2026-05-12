pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BillItem {
    /// Array of tags classifying item or product.
    #[serde(rename = "itemCategories")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_categories: Option<Vec<Option<String>>>,
    #[serde(rename = "itemCommodityCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_commodity_code: Option<ItemCommodityCode>,
    /// Item or product price per unit.
    #[serde(rename = "itemCost")]
    #[serde(default)]
    pub item_cost: f64,
    #[serde(rename = "itemDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_description: Option<ItemDescription>,
    /// Internal class of item or product: value '0' is only for invoices , '1' for bills and, '2' common for both.
    #[serde(rename = "itemMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_mode: Option<i64>,
    #[serde(rename = "itemProductCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_product_code: Option<ItemProductCode>,
    #[serde(rename = "itemProductName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_product_name: Option<ItemProductName>,
    /// Quantity of item or product.
    #[serde(rename = "itemQty")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_qty: Option<i64>,
    /// Tax amount applied to item or product.
    #[serde(rename = "itemTaxAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_tax_amount: Option<f64>,
    /// Tax rate applied to item or product.
    #[serde(rename = "itemTaxRate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_tax_rate: Option<f64>,
    /// Total amount in item or product.
    #[serde(rename = "itemTotalAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_total_amount: Option<f64>,
    #[serde(rename = "itemUnitOfMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_unit_of_measure: Option<ItemUnitofMeasure>,
}

impl BillItem {
    pub fn builder() -> BillItemBuilder {
        <BillItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillItemBuilder {
    item_categories: Option<Vec<Option<String>>>,
    item_commodity_code: Option<ItemCommodityCode>,
    item_cost: Option<f64>,
    item_description: Option<ItemDescription>,
    item_mode: Option<i64>,
    item_product_code: Option<ItemProductCode>,
    item_product_name: Option<ItemProductName>,
    item_qty: Option<i64>,
    item_tax_amount: Option<f64>,
    item_tax_rate: Option<f64>,
    item_total_amount: Option<f64>,
    item_unit_of_measure: Option<ItemUnitofMeasure>,
}

impl BillItemBuilder {
    pub fn item_categories(mut self, value: Vec<Option<String>>) -> Self {
        self.item_categories = Some(value);
        self
    }

    pub fn item_commodity_code(mut self, value: ItemCommodityCode) -> Self {
        self.item_commodity_code = Some(value);
        self
    }

    pub fn item_cost(mut self, value: f64) -> Self {
        self.item_cost = Some(value);
        self
    }

    pub fn item_description(mut self, value: ItemDescription) -> Self {
        self.item_description = Some(value);
        self
    }

    pub fn item_mode(mut self, value: i64) -> Self {
        self.item_mode = Some(value);
        self
    }

    pub fn item_product_code(mut self, value: ItemProductCode) -> Self {
        self.item_product_code = Some(value);
        self
    }

    pub fn item_product_name(mut self, value: ItemProductName) -> Self {
        self.item_product_name = Some(value);
        self
    }

    pub fn item_qty(mut self, value: i64) -> Self {
        self.item_qty = Some(value);
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

    pub fn item_total_amount(mut self, value: f64) -> Self {
        self.item_total_amount = Some(value);
        self
    }

    pub fn item_unit_of_measure(mut self, value: ItemUnitofMeasure) -> Self {
        self.item_unit_of_measure = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BillItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_cost`](BillItemBuilder::item_cost)
    pub fn build(self) -> Result<BillItem, BuildError> {
        Ok(BillItem {
            item_categories: self.item_categories,
            item_commodity_code: self.item_commodity_code,
            item_cost: self
                .item_cost
                .ok_or_else(|| BuildError::missing_field("item_cost"))?,
            item_description: self.item_description,
            item_mode: self.item_mode,
            item_product_code: self.item_product_code,
            item_product_name: self.item_product_name,
            item_qty: self.item_qty,
            item_tax_amount: self.item_tax_amount,
            item_tax_rate: self.item_tax_rate,
            item_total_amount: self.item_total_amount,
            item_unit_of_measure: self.item_unit_of_measure,
        })
    }
}
