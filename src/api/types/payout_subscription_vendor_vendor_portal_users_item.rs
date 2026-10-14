pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayoutSubscriptionVendorVendorPortalUsersItem {
    /// The user's ID.
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    /// The user's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The user's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The user's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

impl PayoutSubscriptionVendorVendorPortalUsersItem {
    pub fn builder() -> PayoutSubscriptionVendorVendorPortalUsersItemBuilder {
        <PayoutSubscriptionVendorVendorPortalUsersItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayoutSubscriptionVendorVendorPortalUsersItemBuilder {
    user_id: Option<i64>,
    email: Option<String>,
    name: Option<String>,
    phone: Option<String>,
}

impl PayoutSubscriptionVendorVendorPortalUsersItemBuilder {
    pub fn user_id(mut self, value: i64) -> Self {
        self.user_id = Some(value);
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn phone(mut self, value: impl Into<String>) -> Self {
        self.phone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PayoutSubscriptionVendorVendorPortalUsersItem`].
    pub fn build(self) -> Result<PayoutSubscriptionVendorVendorPortalUsersItem, BuildError> {
        Ok(PayoutSubscriptionVendorVendorPortalUsersItem {
            user_id: self.user_id,
            email: self.email,
            name: self.name,
            phone: self.phone,
        })
    }
}
