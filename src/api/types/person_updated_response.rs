pub use crate::prelude::*;

/// The reference and updated timestamp after a person update.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PersonUpdatedResponse {
    /// The person reference.
    #[serde(rename = "personReference")]
    #[serde(default)]
    pub person_reference: String,
    /// When the person was last updated.
    #[serde(rename = "lastUpdated")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub last_updated: DateTime<Utc>,
    /// Hypermedia links for the person resource.
    #[serde(rename = "_links")]
    #[serde(default)]
    pub links: PersonLinks,
}

impl PersonUpdatedResponse {
    pub fn builder() -> PersonUpdatedResponseBuilder {
        <PersonUpdatedResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PersonUpdatedResponseBuilder {
    person_reference: Option<String>,
    last_updated: Option<DateTime<Utc>>,
    links: Option<PersonLinks>,
}

impl PersonUpdatedResponseBuilder {
    pub fn person_reference(mut self, value: impl Into<String>) -> Self {
        self.person_reference = Some(value.into());
        self
    }

    pub fn last_updated(mut self, value: DateTime<Utc>) -> Self {
        self.last_updated = Some(value);
        self
    }

    pub fn links(mut self, value: PersonLinks) -> Self {
        self.links = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PersonUpdatedResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`person_reference`](PersonUpdatedResponseBuilder::person_reference)
    /// - [`last_updated`](PersonUpdatedResponseBuilder::last_updated)
    /// - [`links`](PersonUpdatedResponseBuilder::links)
    pub fn build(self) -> Result<PersonUpdatedResponse, BuildError> {
        Ok(PersonUpdatedResponse {
            person_reference: self
                .person_reference
                .ok_or_else(|| BuildError::missing_field("person_reference"))?,
            last_updated: self
                .last_updated
                .ok_or_else(|| BuildError::missing_field("last_updated"))?,
            links: self
                .links
                .ok_or_else(|| BuildError::missing_field("links"))?,
        })
    }
}
