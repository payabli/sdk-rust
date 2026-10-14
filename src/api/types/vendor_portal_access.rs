pub use crate::prelude::*;

/// The vendor's vendor portal access.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VendorPortalAccess {
    /// When `true`, at least one user has access to the vendor portal.
    #[serde(rename = "AnyoneHasAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anyone_has_access: Option<bool>,
    /// Users with access to the vendor portal.
    #[serde(rename = "Users")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<VendorPortalAccessUsersItem>>,
}

impl VendorPortalAccess {
    pub fn builder() -> VendorPortalAccessBuilder {
        <VendorPortalAccessBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VendorPortalAccessBuilder {
    anyone_has_access: Option<bool>,
    users: Option<Vec<VendorPortalAccessUsersItem>>,
}

impl VendorPortalAccessBuilder {
    pub fn anyone_has_access(mut self, value: bool) -> Self {
        self.anyone_has_access = Some(value);
        self
    }

    pub fn users(mut self, value: Vec<VendorPortalAccessUsersItem>) -> Self {
        self.users = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VendorPortalAccess`].
    pub fn build(self) -> Result<VendorPortalAccess, BuildError> {
        Ok(VendorPortalAccess {
            anyone_has_access: self.anyone_has_access,
            users: self.users,
        })
    }
}
