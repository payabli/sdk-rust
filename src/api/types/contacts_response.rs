pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ContactsResponse {
    /// Contact email address.
    #[serde(rename = "ContactEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<Email>,
    /// Contact name.
    #[serde(rename = "ContactName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    /// Contact phone number.
    #[serde(rename = "ContactPhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_phone: Option<String>,
    /// Contact title.
    #[serde(rename = "ContactTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_title: Option<String>,
}

impl ContactsResponse {
    pub fn builder() -> ContactsResponseBuilder {
        <ContactsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContactsResponseBuilder {
    contact_email: Option<Email>,
    contact_name: Option<String>,
    contact_phone: Option<String>,
    contact_title: Option<String>,
}

impl ContactsResponseBuilder {
    pub fn contact_email(mut self, value: Email) -> Self {
        self.contact_email = Some(value);
        self
    }

    pub fn contact_name(mut self, value: impl Into<String>) -> Self {
        self.contact_name = Some(value.into());
        self
    }

    pub fn contact_phone(mut self, value: impl Into<String>) -> Self {
        self.contact_phone = Some(value.into());
        self
    }

    pub fn contact_title(mut self, value: impl Into<String>) -> Self {
        self.contact_title = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ContactsResponse`].
    pub fn build(self) -> Result<ContactsResponse, BuildError> {
        Ok(ContactsResponse {
            contact_email: self.contact_email,
            contact_name: self.contact_name,
            contact_phone: self.contact_phone,
            contact_title: self.contact_title,
        })
    }
}
