pub use crate::prelude::*;

/// Contact information structure.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VCardGetResponseContact {
    /// Name of the contact.
    #[serde(rename = "ContactName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    /// Email of the contact.
    #[serde(rename = "ContactEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<String>,
    /// Title of the contact.
    #[serde(rename = "ContactTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_title: Option<String>,
    /// Phone number of the contact.
    #[serde(rename = "ContactPhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_phone: Option<String>,
}

impl VCardGetResponseContact {
    pub fn builder() -> VCardGetResponseContactBuilder {
        <VCardGetResponseContactBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VCardGetResponseContactBuilder {
    contact_name: Option<String>,
    contact_email: Option<String>,
    contact_title: Option<String>,
    contact_phone: Option<String>,
}

impl VCardGetResponseContactBuilder {
    pub fn contact_name(mut self, value: impl Into<String>) -> Self {
        self.contact_name = Some(value.into());
        self
    }

    pub fn contact_email(mut self, value: impl Into<String>) -> Self {
        self.contact_email = Some(value.into());
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

    /// Consumes the builder and constructs a [`VCardGetResponseContact`].
    pub fn build(self) -> Result<VCardGetResponseContact, BuildError> {
        Ok(VCardGetResponseContact {
            contact_name: self.contact_name,
            contact_email: self.contact_email,
            contact_title: self.contact_title,
            contact_phone: self.contact_phone,
        })
    }
}
