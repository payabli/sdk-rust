pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
