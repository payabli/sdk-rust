pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
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
    #[serde(default)]
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
    #[serde(default)]
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

impl LineItemQueryRecord {
    pub fn builder() -> LineItemQueryRecordBuilder {
        <LineItemQueryRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct LineItemQueryRecordBuilder {
    created_at: Option<CreatedAt>,
    id: Option<i64>,
    item_categories: Option<Vec<Option<String>>>,
    item_commodity_code: Option<ItemCommodityCode>,
    item_cost: Option<f64>,
    item_description: Option<ItemDescription>,
    item_mode: Option<i64>,
    item_product_code: Option<ItemProductCode>,
    item_product_name: Option<ItemProductName>,
    item_qty: Option<i64>,
    item_unit_of_measure: Option<ItemUnitofMeasure>,
    last_updated: Option<LastModified>,
    pageidentifier: Option<PageIdentifier>,
    parent_org_name: Option<OrgParentName>,
    paypoint_dbaname: Option<Dbaname>,
    paypoint_entryname: Option<Entrypointfield>,
    paypoint_legalname: Option<Legalname>,
}

impl LineItemQueryRecordBuilder {
    pub fn created_at(mut self, value: CreatedAt) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

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

    pub fn item_unit_of_measure(mut self, value: ItemUnitofMeasure) -> Self {
        self.item_unit_of_measure = Some(value);
        self
    }

    pub fn last_updated(mut self, value: LastModified) -> Self {
        self.last_updated = Some(value);
        self
    }

    pub fn pageidentifier(mut self, value: PageIdentifier) -> Self {
        self.pageidentifier = Some(value);
        self
    }

    pub fn parent_org_name(mut self, value: OrgParentName) -> Self {
        self.parent_org_name = Some(value);
        self
    }

    pub fn paypoint_dbaname(mut self, value: Dbaname) -> Self {
        self.paypoint_dbaname = Some(value);
        self
    }

    pub fn paypoint_entryname(mut self, value: Entrypointfield) -> Self {
        self.paypoint_entryname = Some(value);
        self
    }

    pub fn paypoint_legalname(mut self, value: Legalname) -> Self {
        self.paypoint_legalname = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`LineItemQueryRecord`].
    /// This method will fail if any of the following fields are not set:
    /// - [`item_cost`](LineItemQueryRecordBuilder::item_cost)
    /// - [`item_qty`](LineItemQueryRecordBuilder::item_qty)
    pub fn build(self) -> Result<LineItemQueryRecord, BuildError> {
        Ok(LineItemQueryRecord {
            created_at: self.created_at,
            id: self.id,
            item_categories: self.item_categories,
            item_commodity_code: self.item_commodity_code,
            item_cost: self
                .item_cost
                .ok_or_else(|| BuildError::missing_field("item_cost"))?,
            item_description: self.item_description,
            item_mode: self.item_mode,
            item_product_code: self.item_product_code,
            item_product_name: self.item_product_name,
            item_qty: self
                .item_qty
                .ok_or_else(|| BuildError::missing_field("item_qty"))?,
            item_unit_of_measure: self.item_unit_of_measure,
            last_updated: self.last_updated,
            pageidentifier: self.pageidentifier,
            parent_org_name: self.parent_org_name,
            paypoint_dbaname: self.paypoint_dbaname,
            paypoint_entryname: self.paypoint_entryname,
            paypoint_legalname: self.paypoint_legalname,
        })
    }
}
