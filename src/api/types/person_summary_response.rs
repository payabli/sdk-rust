pub use crate::prelude::*;

/// A person as it appears in a list response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PersonSummaryResponse {
    /// Hypermedia links for the person resource.
    #[serde(rename = "_links")]
    #[serde(default)]
    pub links: PersonLinks,
    /// The person reference, used to read and manage the person.
    #[serde(rename = "personReference")]
    #[serde(default)]
    pub person_reference: String,
    /// The person's first name.
    #[serde(rename = "firstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// The person's last name.
    #[serde(rename = "lastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The person's primary email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// The person's identity verification status, for example `notstarted`.
    #[serde(rename = "verificationStatus")]
    #[serde(default)]
    pub verification_status: String,
    /// When the person was created.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub created_at: DateTime<Utc>,
    /// When the person was last updated.
    #[serde(rename = "lastUpdated")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub last_updated: DateTime<Utc>,
}

impl PersonSummaryResponse {
    pub fn builder() -> PersonSummaryResponseBuilder {
        <PersonSummaryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PersonSummaryResponseBuilder {
    links: Option<PersonLinks>,
    person_reference: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    email: Option<String>,
    verification_status: Option<String>,
    created_at: Option<DateTime<Utc>>,
    last_updated: Option<DateTime<Utc>>,
}

impl PersonSummaryResponseBuilder {
    pub fn links(mut self, value: PersonLinks) -> Self {
        self.links = Some(value);
        self
    }

    pub fn person_reference(mut self, value: impl Into<String>) -> Self {
        self.person_reference = Some(value.into());
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn email(mut self, value: impl Into<String>) -> Self {
        self.email = Some(value.into());
        self
    }

    pub fn verification_status(mut self, value: impl Into<String>) -> Self {
        self.verification_status = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn last_updated(mut self, value: DateTime<Utc>) -> Self {
        self.last_updated = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PersonSummaryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`links`](PersonSummaryResponseBuilder::links)
    /// - [`person_reference`](PersonSummaryResponseBuilder::person_reference)
    /// - [`verification_status`](PersonSummaryResponseBuilder::verification_status)
    /// - [`created_at`](PersonSummaryResponseBuilder::created_at)
    /// - [`last_updated`](PersonSummaryResponseBuilder::last_updated)
    pub fn build(self) -> Result<PersonSummaryResponse, BuildError> {
        Ok(PersonSummaryResponse {
            links: self
                .links
                .ok_or_else(|| BuildError::missing_field("links"))?,
            person_reference: self
                .person_reference
                .ok_or_else(|| BuildError::missing_field("person_reference"))?,
            first_name: self.first_name,
            last_name: self.last_name,
            email: self.email,
            verification_status: self
                .verification_status
                .ok_or_else(|| BuildError::missing_field("verification_status"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            last_updated: self
                .last_updated
                .ok_or_else(|| BuildError::missing_field("last_updated"))?,
        })
    }
}
