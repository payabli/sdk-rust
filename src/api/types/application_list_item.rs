pub use crate::prelude::*;

/// A boarding application (request), as returned in a list of applications.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApplicationListItem {
    #[serde(flatten)]
    pub application_common_fields: ApplicationCommon,
    /// The name of the paypoint being boarded.
    #[serde(rename = "paypointName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_name: Option<String>,
    /// The legal name of the paypoint being boarded.
    #[serde(rename = "paypointLegalName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legal_name: Option<String>,
    /// The employer identification number, masked (for example `****8993`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ein: Option<String>,
    /// The name of the primary owner.
    #[serde(rename = "primaryOwnerName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_owner_name: Option<String>,
    /// The reference of the primary owner.
    #[serde(rename = "primaryOwnerReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_owner_reference: Option<String>,
}

impl ApplicationListItem {
    pub fn builder() -> ApplicationListItemBuilder {
        <ApplicationListItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplicationListItemBuilder {
    application_common_fields: Option<ApplicationCommon>,
    paypoint_name: Option<String>,
    paypoint_legal_name: Option<String>,
    ein: Option<String>,
    primary_owner_name: Option<String>,
    primary_owner_reference: Option<String>,
}

impl ApplicationListItemBuilder {
    pub fn application_common_fields(mut self, value: ApplicationCommon) -> Self {
        self.application_common_fields = Some(value);
        self
    }

    pub fn paypoint_name(mut self, value: impl Into<String>) -> Self {
        self.paypoint_name = Some(value.into());
        self
    }

    pub fn paypoint_legal_name(mut self, value: impl Into<String>) -> Self {
        self.paypoint_legal_name = Some(value.into());
        self
    }

    pub fn ein(mut self, value: impl Into<String>) -> Self {
        self.ein = Some(value.into());
        self
    }

    pub fn primary_owner_name(mut self, value: impl Into<String>) -> Self {
        self.primary_owner_name = Some(value.into());
        self
    }

    pub fn primary_owner_reference(mut self, value: impl Into<String>) -> Self {
        self.primary_owner_reference = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ApplicationListItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`application_common_fields`](ApplicationListItemBuilder::application_common_fields)
    pub fn build(self) -> Result<ApplicationListItem, BuildError> {
        Ok(ApplicationListItem {
            application_common_fields: self
                .application_common_fields
                .ok_or_else(|| BuildError::missing_field("application_common_fields"))?,
            paypoint_name: self.paypoint_name,
            paypoint_legal_name: self.paypoint_legal_name,
            ein: self.ein,
            primary_owner_name: self.primary_owner_name,
            primary_owner_reference: self.primary_owner_reference,
        })
    }
}
