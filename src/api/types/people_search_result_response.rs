pub use crate::prelude::*;

/// A single identity match returned by a search.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PeopleSearchResultResponse {
    /// The person reference of the matching existing person. Null when `isFromVendor` is `true`.
    #[serde(rename = "personReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub person_reference: Option<String>,
    /// The matched first name.
    #[serde(rename = "firstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// The matched last name.
    #[serde(rename = "lastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The matched date of birth. The standard search masks it (for example `****-**-14`), and the unmasked search returns the full `YYYY-MM-DD` value.
    #[serde(rename = "dateOfBirth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date_of_birth: Option<String>,
    /// The matched primary email address.
    #[serde(rename = "primaryEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_email: Option<String>,
    /// The matched primary phone number.
    #[serde(rename = "primaryPhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_phone: Option<String>,
    /// The risk decision for the match.
    #[serde(rename = "riskDecision")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub risk_decision: Option<String>,
    /// How closely the record matches the search criteria.
    #[serde(rename = "matchScore")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub match_score: Option<f64>,
    /// Whether this is the highest-scoring match.
    #[serde(rename = "isTopMatch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_top_match: Option<bool>,
    /// When `true`, no existing person matched, and this result comes only from the identity vendor. A vendor result has no `personReference`.
    #[serde(rename = "isFromVendor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_from_vendor: Option<bool>,
    /// The person's phone numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phones: Option<Vec<String>>,
    /// The person's email addresses.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
    /// The number of addresses on file for the person.
    #[serde(rename = "addressCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_count: Option<i64>,
    /// The number of phone numbers on file for the person.
    #[serde(rename = "phoneCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_count: Option<i64>,
    /// The number of email addresses on file for the person.
    #[serde(rename = "emailCount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_count: Option<i64>,
}

impl PeopleSearchResultResponse {
    pub fn builder() -> PeopleSearchResultResponseBuilder {
        <PeopleSearchResultResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PeopleSearchResultResponseBuilder {
    person_reference: Option<String>,
    first_name: Option<String>,
    last_name: Option<String>,
    date_of_birth: Option<String>,
    primary_email: Option<String>,
    primary_phone: Option<String>,
    risk_decision: Option<String>,
    match_score: Option<f64>,
    is_top_match: Option<bool>,
    is_from_vendor: Option<bool>,
    phones: Option<Vec<String>>,
    emails: Option<Vec<String>>,
    address_count: Option<i64>,
    phone_count: Option<i64>,
    email_count: Option<i64>,
}

impl PeopleSearchResultResponseBuilder {
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

    pub fn date_of_birth(mut self, value: impl Into<String>) -> Self {
        self.date_of_birth = Some(value.into());
        self
    }

    pub fn primary_email(mut self, value: impl Into<String>) -> Self {
        self.primary_email = Some(value.into());
        self
    }

    pub fn primary_phone(mut self, value: impl Into<String>) -> Self {
        self.primary_phone = Some(value.into());
        self
    }

    pub fn risk_decision(mut self, value: impl Into<String>) -> Self {
        self.risk_decision = Some(value.into());
        self
    }

    pub fn match_score(mut self, value: f64) -> Self {
        self.match_score = Some(value);
        self
    }

    pub fn is_top_match(mut self, value: bool) -> Self {
        self.is_top_match = Some(value);
        self
    }

    pub fn is_from_vendor(mut self, value: bool) -> Self {
        self.is_from_vendor = Some(value);
        self
    }

    pub fn phones(mut self, value: Vec<String>) -> Self {
        self.phones = Some(value);
        self
    }

    pub fn emails(mut self, value: Vec<String>) -> Self {
        self.emails = Some(value);
        self
    }

    pub fn address_count(mut self, value: i64) -> Self {
        self.address_count = Some(value);
        self
    }

    pub fn phone_count(mut self, value: i64) -> Self {
        self.phone_count = Some(value);
        self
    }

    pub fn email_count(mut self, value: i64) -> Self {
        self.email_count = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PeopleSearchResultResponse`].
    pub fn build(self) -> Result<PeopleSearchResultResponse, BuildError> {
        Ok(PeopleSearchResultResponse {
            person_reference: self.person_reference,
            first_name: self.first_name,
            last_name: self.last_name,
            date_of_birth: self.date_of_birth,
            primary_email: self.primary_email,
            primary_phone: self.primary_phone,
            risk_decision: self.risk_decision,
            match_score: self.match_score,
            is_top_match: self.is_top_match,
            is_from_vendor: self.is_from_vendor,
            phones: self.phones,
            emails: self.emails,
            address_count: self.address_count,
            phone_count: self.phone_count,
            email_count: self.email_count,
        })
    }
}
