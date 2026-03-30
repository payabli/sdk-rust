pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ASection {
    #[serde(rename = "minimumDocuments")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_documents: Option<i64>,
    #[serde(rename = "multipleContacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_contacts: Option<bool>,
    #[serde(rename = "multipleOwners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_owners: Option<bool>,
}

impl ASection {
    pub fn builder() -> ASectionBuilder {
        <ASectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ASectionBuilder {
    minimum_documents: Option<i64>,
    multiple_contacts: Option<bool>,
    multiple_owners: Option<bool>,
}

impl ASectionBuilder {
    pub fn minimum_documents(mut self, value: i64) -> Self {
        self.minimum_documents = Some(value);
        self
    }

    pub fn multiple_contacts(mut self, value: bool) -> Self {
        self.multiple_contacts = Some(value);
        self
    }

    pub fn multiple_owners(mut self, value: bool) -> Self {
        self.multiple_owners = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ASection`].
    pub fn build(self) -> Result<ASection, BuildError> {
        Ok(ASection {
            minimum_documents: self.minimum_documents,
            multiple_contacts: self.multiple_contacts,
            multiple_owners: self.multiple_owners,
        })
    }
}
