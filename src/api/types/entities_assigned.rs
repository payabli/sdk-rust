pub use crate::prelude::*;

/// Counts of entities the profile is assigned to. Any non-zero count locks the
/// profile from deletion in the Payabli Portal.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EntitiesAssigned {
    /// Number of organizations the profile is assigned to.
    #[serde(default)]
    pub organizations: i64,
    /// Number of paypoints the profile is assigned to.
    #[serde(default)]
    pub paypoints: i64,
    /// Number of boarding templates the profile is assigned to.
    #[serde(default)]
    pub templates: i64,
    /// Number of boarding applications the profile is assigned to.
    #[serde(default)]
    pub applications: i64,
}

impl EntitiesAssigned {
    pub fn builder() -> EntitiesAssignedBuilder {
        <EntitiesAssignedBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EntitiesAssignedBuilder {
    organizations: Option<i64>,
    paypoints: Option<i64>,
    templates: Option<i64>,
    applications: Option<i64>,
}

impl EntitiesAssignedBuilder {
    pub fn organizations(mut self, value: i64) -> Self {
        self.organizations = Some(value);
        self
    }

    pub fn paypoints(mut self, value: i64) -> Self {
        self.paypoints = Some(value);
        self
    }

    pub fn templates(mut self, value: i64) -> Self {
        self.templates = Some(value);
        self
    }

    pub fn applications(mut self, value: i64) -> Self {
        self.applications = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EntitiesAssigned`].
    /// This method will fail if any of the following fields are not set:
    /// - [`organizations`](EntitiesAssignedBuilder::organizations)
    /// - [`paypoints`](EntitiesAssignedBuilder::paypoints)
    /// - [`templates`](EntitiesAssignedBuilder::templates)
    /// - [`applications`](EntitiesAssignedBuilder::applications)
    pub fn build(self) -> Result<EntitiesAssigned, BuildError> {
        Ok(EntitiesAssigned {
            organizations: self
                .organizations
                .ok_or_else(|| BuildError::missing_field("organizations"))?,
            paypoints: self
                .paypoints
                .ok_or_else(|| BuildError::missing_field("paypoints"))?,
            templates: self
                .templates
                .ok_or_else(|| BuildError::missing_field("templates"))?,
            applications: self
                .applications
                .ok_or_else(|| BuildError::missing_field("applications"))?,
        })
    }
}
