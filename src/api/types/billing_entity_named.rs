pub use crate::prelude::*;

/// An owning entity, as returned by the List profiles endpoint (`entityType`
/// serialized as a name).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BillingEntityNamed {
    #[serde(rename = "entityType")]
    pub entity_type: EntityTypeName,
    /// Identifier of the entity.
    #[serde(rename = "entityId")]
    #[serde(default)]
    pub entity_id: i64,
}

impl BillingEntityNamed {
    pub fn builder() -> BillingEntityNamedBuilder {
        <BillingEntityNamedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillingEntityNamedBuilder {
    entity_type: Option<EntityTypeName>,
    entity_id: Option<i64>,
}

impl BillingEntityNamedBuilder {
    pub fn entity_type(mut self, value: EntityTypeName) -> Self {
        self.entity_type = Some(value);
        self
    }

    pub fn entity_id(mut self, value: i64) -> Self {
        self.entity_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BillingEntityNamed`].
    /// This method will fail if any of the following fields are not set:
    /// - [`entity_type`](BillingEntityNamedBuilder::entity_type)
    /// - [`entity_id`](BillingEntityNamedBuilder::entity_id)
    pub fn build(self) -> Result<BillingEntityNamed, BuildError> {
        Ok(BillingEntityNamed {
            entity_type: self
                .entity_type
                .ok_or_else(|| BuildError::missing_field("entity_type"))?,
            entity_id: self
                .entity_id
                .ok_or_else(|| BuildError::missing_field("entity_id"))?,
        })
    }
}
