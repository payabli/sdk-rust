pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Contacts {
    /// Contact email address.
    #[serde(rename = "contactEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<Email>,
    /// Contact name.
    #[serde(rename = "contactName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<String>,
    /// Contact phone number.
    #[serde(rename = "contactPhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_phone: Option<String>,
    /// Contact title.
    #[serde(rename = "contactTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_title: Option<String>,
    #[serde(rename = "additionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalDataString>,
}

impl Contacts {
    pub fn builder() -> ContactsBuilder {
        <ContactsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ContactsBuilder {
    contact_email: Option<Email>,
    contact_name: Option<String>,
    contact_phone: Option<String>,
    contact_title: Option<String>,
    additional_data: Option<AdditionalDataString>,
}

impl ContactsBuilder {
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

    pub fn additional_data(mut self, value: AdditionalDataString) -> Self {
        self.additional_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Contacts`].
    pub fn build(self) -> Result<Contacts, BuildError> {
        Ok(Contacts {
            contact_email: self.contact_email,
            contact_name: self.contact_name,
            contact_phone: self.contact_phone,
            contact_title: self.contact_title,
            additional_data: self.additional_data,
        })
    }
}
