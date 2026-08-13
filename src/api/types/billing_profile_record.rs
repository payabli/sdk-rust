pub use crate::prelude::*;

/// A single billing profile as it appears in the list.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BillingProfileRecord {
    /// Unique, server-generated profile identifier.
    #[serde(default)]
    pub id: i64,
    /// Identifier of this specific version of the profile.
    #[serde(rename = "versionId")]
    #[serde(default)]
    pub version_id: i64,
    /// Sequential version counter. Starts at `1` and increments on every edit.
    #[serde(rename = "versionNumber")]
    #[serde(default)]
    pub version_number: i64,
    pub business: BillingEntityNamed,
    #[serde(rename = "serviceVertical")]
    pub service_vertical: ServiceVerticalName,
    /// Descriptive name for the profile.
    #[serde(default)]
    pub name: String,
    #[serde(rename = "feeType")]
    pub fee_type: FeeTypeName,
    /// When this version was created.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub created_at: DateTime<Utc>,
    /// When this version was last updated.
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "entitiesAssigned")]
    #[serde(default)]
    pub entities_assigned: EntitiesAssigned,
    /// Parent-entity reference formatted as `{entityType}:{entityId}` (for
    /// example, `1:2`).
    #[serde(rename = "parentId")]
    #[serde(default)]
    pub parent_id: String,
    /// Number of billable events configured on the profile.
    #[serde(rename = "countOfEvents")]
    #[serde(default)]
    pub count_of_events: i64,
}

impl BillingProfileRecord {
    pub fn builder() -> BillingProfileRecordBuilder {
        <BillingProfileRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillingProfileRecordBuilder {
    id: Option<i64>,
    version_id: Option<i64>,
    version_number: Option<i64>,
    business: Option<BillingEntityNamed>,
    service_vertical: Option<ServiceVerticalName>,
    name: Option<String>,
    fee_type: Option<FeeTypeName>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
    entities_assigned: Option<EntitiesAssigned>,
    parent_id: Option<String>,
    count_of_events: Option<i64>,
}

impl BillingProfileRecordBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn version_id(mut self, value: i64) -> Self {
        self.version_id = Some(value);
        self
    }

    pub fn version_number(mut self, value: i64) -> Self {
        self.version_number = Some(value);
        self
    }

    pub fn business(mut self, value: BillingEntityNamed) -> Self {
        self.business = Some(value);
        self
    }

    pub fn service_vertical(mut self, value: ServiceVerticalName) -> Self {
        self.service_vertical = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn fee_type(mut self, value: FeeTypeName) -> Self {
        self.fee_type = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<Utc>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn entities_assigned(mut self, value: EntitiesAssigned) -> Self {
        self.entities_assigned = Some(value);
        self
    }

    pub fn parent_id(mut self, value: impl Into<String>) -> Self {
        self.parent_id = Some(value.into());
        self
    }

    pub fn count_of_events(mut self, value: i64) -> Self {
        self.count_of_events = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BillingProfileRecord`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](BillingProfileRecordBuilder::id)
    /// - [`version_id`](BillingProfileRecordBuilder::version_id)
    /// - [`version_number`](BillingProfileRecordBuilder::version_number)
    /// - [`business`](BillingProfileRecordBuilder::business)
    /// - [`service_vertical`](BillingProfileRecordBuilder::service_vertical)
    /// - [`name`](BillingProfileRecordBuilder::name)
    /// - [`fee_type`](BillingProfileRecordBuilder::fee_type)
    /// - [`created_at`](BillingProfileRecordBuilder::created_at)
    /// - [`updated_at`](BillingProfileRecordBuilder::updated_at)
    /// - [`entities_assigned`](BillingProfileRecordBuilder::entities_assigned)
    /// - [`parent_id`](BillingProfileRecordBuilder::parent_id)
    /// - [`count_of_events`](BillingProfileRecordBuilder::count_of_events)
    pub fn build(self) -> Result<BillingProfileRecord, BuildError> {
        Ok(BillingProfileRecord {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            version_id: self
                .version_id
                .ok_or_else(|| BuildError::missing_field("version_id"))?,
            version_number: self
                .version_number
                .ok_or_else(|| BuildError::missing_field("version_number"))?,
            business: self
                .business
                .ok_or_else(|| BuildError::missing_field("business"))?,
            service_vertical: self
                .service_vertical
                .ok_or_else(|| BuildError::missing_field("service_vertical"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            fee_type: self
                .fee_type
                .ok_or_else(|| BuildError::missing_field("fee_type"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            entities_assigned: self
                .entities_assigned
                .ok_or_else(|| BuildError::missing_field("entities_assigned"))?,
            parent_id: self
                .parent_id
                .ok_or_else(|| BuildError::missing_field("parent_id"))?,
            count_of_events: self
                .count_of_events
                .ok_or_else(|| BuildError::missing_field("count_of_events"))?,
        })
    }
}
