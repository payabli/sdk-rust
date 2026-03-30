pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CList {
    #[serde(rename = "contactEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<LinkData>,
    #[serde(rename = "contactName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<LinkData>,
    #[serde(rename = "contactPhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_phone: Option<LinkData>,
    #[serde(rename = "contactTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_title: Option<LinkData>,
}

impl CList {
    pub fn builder() -> CListBuilder {
        <CListBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CListBuilder {
    contact_email: Option<LinkData>,
    contact_name: Option<LinkData>,
    contact_phone: Option<LinkData>,
    contact_title: Option<LinkData>,
}

impl CListBuilder {
    pub fn contact_email(mut self, value: LinkData) -> Self {
        self.contact_email = Some(value);
        self
    }

    pub fn contact_name(mut self, value: LinkData) -> Self {
        self.contact_name = Some(value);
        self
    }

    pub fn contact_phone(mut self, value: LinkData) -> Self {
        self.contact_phone = Some(value);
        self
    }

    pub fn contact_title(mut self, value: LinkData) -> Self {
        self.contact_title = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CList`].
    pub fn build(self) -> Result<CList, BuildError> {
        Ok(CList {
            contact_email: self.contact_email,
            contact_name: self.contact_name,
            contact_phone: self.contact_phone,
            contact_title: self.contact_title,
        })
    }
}
