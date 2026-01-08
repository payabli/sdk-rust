pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LineItemQueryRecord {
    /// Timestamp of when line item was created, in UTC.
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    /// Identifier of line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
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
    /// Internal class of item or product: value '0' is only for invoices , '1' for bills, and '2' common for both.
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
    pub item_qty: i64,
    #[serde(rename = "itemUnitOfMeasure")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_unit_of_measure: Option<ItemUnitofMeasure>,
    /// Timestamp of when the line item was updated, in UTC.
    #[serde(rename = "lastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<LastModified>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageidentifier: Option<PageIdentifier>,
    /// The name of the paypoint's parent organization.
    #[serde(rename = "ParentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<OrgParentName>,
    /// The paypoint's DBA name.
    #[serde(rename = "PaypointDbaname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dbaname: Option<Dbaname>,
    /// The paypoint's entryname (entrypoint) value.
    #[serde(rename = "PaypointEntryname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_entryname: Option<Entrypointfield>,
    /// The paypoint's legal name.
    #[serde(rename = "PaypointLegalname")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legalname: Option<Legalname>,
}
