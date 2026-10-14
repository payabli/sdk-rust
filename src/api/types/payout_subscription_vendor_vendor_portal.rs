pub use crate::prelude::*;

/// The vendor's vendor portal access. Always `null` on this endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayoutSubscriptionVendorVendorPortal {
    /// When `true`, at least one user has access to the vendor portal.
    #[serde(rename = "anyoneHasAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub anyone_has_access: Option<bool>,
    /// Users with access to the vendor portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<PayoutSubscriptionVendorVendorPortalUsersItem>>,
}

impl PayoutSubscriptionVendorVendorPortal {
    pub fn builder() -> PayoutSubscriptionVendorVendorPortalBuilder {
        <PayoutSubscriptionVendorVendorPortalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayoutSubscriptionVendorVendorPortalBuilder {
    anyone_has_access: Option<bool>,
    users: Option<Vec<PayoutSubscriptionVendorVendorPortalUsersItem>>,
}

impl PayoutSubscriptionVendorVendorPortalBuilder {
    pub fn anyone_has_access(mut self, value: bool) -> Self {
        self.anyone_has_access = Some(value);
        self
    }

    pub fn users(mut self, value: Vec<PayoutSubscriptionVendorVendorPortalUsersItem>) -> Self {
        self.users = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayoutSubscriptionVendorVendorPortal`].
    pub fn build(self) -> Result<PayoutSubscriptionVendorVendorPortal, BuildError> {
        Ok(PayoutSubscriptionVendorVendorPortal {
            anyone_has_access: self.anyone_has_access,
            users: self.users,
        })
    }
}
