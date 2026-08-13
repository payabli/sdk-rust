pub use crate::prelude::*;

/// An owning or participating entity, as returned by the View profile endpoint
/// (`entityType` serialized as an integer).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BillingEntity {
    #[serde(rename = "entityType")]
    #[serde(default)]
    pub entity_type: EntityTypeValue,
    /// Identifier of the entity.
    #[serde(rename = "entityId")]
    #[serde(default)]
    pub entity_id: i64,
}

impl BillingEntity {
    pub fn builder() -> BillingEntityBuilder {
        <BillingEntityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillingEntityBuilder {
    entity_type: Option<EntityTypeValue>,
    entity_id: Option<i64>,
}

impl BillingEntityBuilder {
    pub fn entity_type(mut self, value: EntityTypeValue) -> Self {
        self.entity_type = Some(value);
        self
    }

    pub fn entity_id(mut self, value: i64) -> Self {
        self.entity_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BillingEntity`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entity_type`](BillingEntityBuilder::entity_type)
    /// - [`entity_id`](BillingEntityBuilder::entity_id)
    pub fn build(self) -> Result<BillingEntity, BuildError> {
        Ok(BillingEntity {
            entity_type: self
                .entity_type
                .ok_or_else(|| BuildError::missing_field("entity_type"))?,
            entity_id: self
                .entity_id
                .ok_or_else(|| BuildError::missing_field("entity_id"))?,
        })
    }
}
