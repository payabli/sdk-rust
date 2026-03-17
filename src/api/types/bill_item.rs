pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
