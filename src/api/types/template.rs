pub use crate::prelude::*;

/// A boarding template, including its current content version.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Template {
    /// The template's unique identifier, used to read and manage the template.
    #[serde(default)]
    pub uuid: String,
    /// The numeric identifier of the organization that owns the template.
    #[serde(rename = "orgId")]
    #[serde(default)]
    pub org_id: i64,
    /// The name of the organization that owns the template.
    #[serde(rename = "orgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_name: Option<String>,
    /// The template's display name.
    #[serde(default)]
    pub name: String,
    /// An optional description of the template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// An optional URL-friendly identifier for the template.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub slug: Option<String>,
    pub status: TemplateStatus,
    /// The UUID of the template's active content version.
    #[serde(rename = "currentVersionUuid")]
    #[serde(default)]
    pub current_version_uuid: String,
    /// The template's active content version.
    #[serde(rename = "currentVersion")]
    pub current_version: TemplateVersion,
    /// Whether the template is inherited from an ancestor organization rather than owned directly.
    #[serde(rename = "isInherited")]
    #[serde(default)]
    pub is_inherited: bool,
    /// Whether the template is read-only for the calling organization.
    #[serde(rename = "readOnly")]
    #[serde(default)]
    pub read_only: bool,
    /// The identifier of the user who created the template.
    #[serde(rename = "createdBy")]
    #[serde(default)]
    pub created_by: String,
    /// The identifier of the user who last updated the template.
    #[serde(rename = "updatedBy")]
    #[serde(default)]
    pub updated_by: String,
    /// When the template was created.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    /// When the template was last updated.
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
}

impl Template {
    pub fn builder() -> TemplateBuilder {
        <TemplateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TemplateBuilder {
    uuid: Option<String>,
    org_id: Option<i64>,
    org_name: Option<String>,
    name: Option<String>,
    description: Option<String>,
    slug: Option<String>,
    status: Option<TemplateStatus>,
    current_version_uuid: Option<String>,
    current_version: Option<TemplateVersion>,
    is_inherited: Option<bool>,
    read_only: Option<bool>,
    created_by: Option<String>,
    updated_by: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
}

impl TemplateBuilder {
    pub fn uuid(mut self, value: impl Into<String>) -> Self {
        self.uuid = Some(value.into());
        self
    }

    pub fn org_id(mut self, value: i64) -> Self {
        self.org_id = Some(value);
        self
    }

    pub fn org_name(mut self, value: impl Into<String>) -> Self {
        self.org_name = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn slug(mut self, value: impl Into<String>) -> Self {
        self.slug = Some(value.into());
        self
    }

    pub fn status(mut self, value: TemplateStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn current_version_uuid(mut self, value: impl Into<String>) -> Self {
        self.current_version_uuid = Some(value.into());
        self
    }

    pub fn current_version(mut self, value: TemplateVersion) -> Self {
        self.current_version = Some(value);
        self
    }

    pub fn is_inherited(mut self, value: bool) -> Self {
        self.is_inherited = Some(value);
        self
    }

    pub fn read_only(mut self, value: bool) -> Self {
        self.read_only = Some(value);
        self
    }

    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
        self
    }

    pub fn updated_by(mut self, value: impl Into<String>) -> Self {
        self.updated_by = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Template`].
    /// This method will fail if any of the following fields are not set:
    /// - [`uuid`](TemplateBuilder::uuid)
    /// - [`org_id`](TemplateBuilder::org_id)
    /// - [`name`](TemplateBuilder::name)
    /// - [`status`](TemplateBuilder::status)
    /// - [`current_version_uuid`](TemplateBuilder::current_version_uuid)
    /// - [`current_version`](TemplateBuilder::current_version)
    /// - [`is_inherited`](TemplateBuilder::is_inherited)
    /// - [`read_only`](TemplateBuilder::read_only)
    /// - [`created_by`](TemplateBuilder::created_by)
    /// - [`updated_by`](TemplateBuilder::updated_by)
    /// - [`created_at`](TemplateBuilder::created_at)
    /// - [`updated_at`](TemplateBuilder::updated_at)
    pub fn build(self) -> Result<Template, BuildError> {
        Ok(Template {
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
            org_id: self
                .org_id
                .ok_or_else(|| BuildError::missing_field("org_id"))?,
            org_name: self.org_name,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            description: self.description,
            slug: self.slug,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            current_version_uuid: self
                .current_version_uuid
                .ok_or_else(|| BuildError::missing_field("current_version_uuid"))?,
            current_version: self
                .current_version
                .ok_or_else(|| BuildError::missing_field("current_version"))?,
            is_inherited: self
                .is_inherited
                .ok_or_else(|| BuildError::missing_field("is_inherited"))?,
            read_only: self
                .read_only
                .ok_or_else(|| BuildError::missing_field("read_only"))?,
            created_by: self
                .created_by
                .ok_or_else(|| BuildError::missing_field("created_by"))?,
            updated_by: self
                .updated_by
                .ok_or_else(|| BuildError::missing_field("updated_by"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
        })
    }
}
