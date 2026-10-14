pub use crate::prelude::*;

/// A vendor contact.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayoutSubscriptionVendorContact {
    /// Contact name.
    #[serde(rename = "contactName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    /// Contact email address.
    #[serde(rename = "contactEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<Email>,
    /// Contact title.
    #[serde(rename = "contactTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_title: Option<String>,
    /// Contact phone number.
    #[serde(rename = "contactPhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_phone: Option<String>,
}

impl PayoutSubscriptionVendorContact {
    pub fn builder() -> PayoutSubscriptionVendorContactBuilder {
        <PayoutSubscriptionVendorContactBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayoutSubscriptionVendorContactBuilder {
    contact_name: Option<String>,
    contact_email: Option<Email>,
    contact_title: Option<String>,
    contact_phone: Option<String>,
}

impl PayoutSubscriptionVendorContactBuilder {
    pub fn contact_name(mut self, value: impl Into<String>) -> Self {
        self.contact_name = Some(value.into());
        self
    }

    pub fn contact_email(mut self, value: Email) -> Self {
        self.contact_email = Some(value);
        self
    }

    pub fn contact_title(mut self, value: impl Into<String>) -> Self {
        self.contact_title = Some(value.into());
        self
    }

    pub fn contact_phone(mut self, value: impl Into<String>) -> Self {
        self.contact_phone = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PayoutSubscriptionVendorContact`].
    pub fn build(self) -> Result<PayoutSubscriptionVendorContact, BuildError> {
        Ok(PayoutSubscriptionVendorContact {
            contact_name: self.contact_name,
            contact_email: self.contact_email,
            contact_title: self.contact_title,
            contact_phone: self.contact_phone,
        })
    }
}
