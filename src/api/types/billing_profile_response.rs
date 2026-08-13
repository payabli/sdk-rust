pub use crate::prelude::*;

/// A billing profile assigned to an entity, returned by the View profile
/// endpoint. A profile is a named configuration of billable events, each with
/// one or more fee schedules. Profiles are append-only versioned — every edit
/// mints a new version.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BillingProfileResponse {
    /// Unique, server-generated profile identifier.
    #[serde(default)]
    pub id: i64,
    /// Identifier of this specific version of the profile.
    #[serde(rename = "versionId")]
    #[serde(default)]
    pub version_id: i64,
    /// Sequential version counter. Starts at `1` and increments on every edit
    /// (profiles are append-only versioned, not mutated in place).
    #[serde(rename = "versionNumber")]
    #[serde(default)]
    pub version_number: i64,
    #[serde(default)]
    pub business: BillingEntity,
    /// Descriptive name for the profile.
    #[serde(default)]
    pub name: String,
    #[serde(rename = "feeType")]
    #[serde(default)]
    pub fee_type: FeeTypeValue,
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
    /// Parent-entity reference used for inheritance and permission checks,
    /// formatted as `{entityType}:{entityId}` (for example, `1:2` is
    /// organization `2`). Org-level profiles reference their own organization;
    /// paypoint-level profiles reference their parent organization.
    #[serde(rename = "parentId")]
    #[serde(default)]
    pub parent_id: String,
    /// The chargeable events this profile covers.
    #[serde(rename = "billableEvents")]
    #[serde(default)]
    pub billable_events: Vec<BillableEvent>,
}

impl BillingProfileResponse {
    pub fn builder() -> BillingProfileResponseBuilder {
        <BillingProfileResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillingProfileResponseBuilder {
    id: Option<i64>,
    version_id: Option<i64>,
    version_number: Option<i64>,
    business: Option<BillingEntity>,
    name: Option<String>,
    fee_type: Option<FeeTypeValue>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
    parent_id: Option<String>,
    billable_events: Option<Vec<BillableEvent>>,
}

impl BillingProfileResponseBuilder {
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

    pub fn business(mut self, value: BillingEntity) -> Self {
        self.business = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn fee_type(mut self, value: FeeTypeValue) -> Self {
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

    pub fn parent_id(mut self, value: impl Into<String>) -> Self {
        self.parent_id = Some(value.into());
        self
    }

    pub fn billable_events(mut self, value: Vec<BillableEvent>) -> Self {
        self.billable_events = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BillingProfileResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](BillingProfileResponseBuilder::id)
    /// - [`version_id`](BillingProfileResponseBuilder::version_id)
    /// - [`version_number`](BillingProfileResponseBuilder::version_number)
    /// - [`business`](BillingProfileResponseBuilder::business)
    /// - [`name`](BillingProfileResponseBuilder::name)
    /// - [`fee_type`](BillingProfileResponseBuilder::fee_type)
    /// - [`created_at`](BillingProfileResponseBuilder::created_at)
    /// - [`updated_at`](BillingProfileResponseBuilder::updated_at)
    /// - [`parent_id`](BillingProfileResponseBuilder::parent_id)
    /// - [`billable_events`](BillingProfileResponseBuilder::billable_events)
    pub fn build(self) -> Result<BillingProfileResponse, BuildError> {
        Ok(BillingProfileResponse {
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
            parent_id: self
                .parent_id
                .ok_or_else(|| BuildError::missing_field("parent_id"))?,
            billable_events: self
                .billable_events
                .ok_or_else(|| BuildError::missing_field("billable_events"))?,
        })
    }
}
