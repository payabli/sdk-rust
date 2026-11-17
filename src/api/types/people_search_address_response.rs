pub use crate::prelude::*;

/// An address on a people search result. Search results use a shorter address
/// format than the person record: `street`, `city`, `state`, and `zip`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PeopleSearchAddressResponse {
    /// The street address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub street: Option<String>,
    /// The city.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    /// The state or province.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// The ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zip: Option<String>,
}

impl PeopleSearchAddressResponse {
    pub fn builder() -> PeopleSearchAddressResponseBuilder {
        <PeopleSearchAddressResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PeopleSearchAddressResponseBuilder {
    street: Option<String>,
    city: Option<String>,
    state: Option<String>,
    zip: Option<String>,
}

impl PeopleSearchAddressResponseBuilder {
    pub fn street(mut self, value: impl Into<String>) -> Self {
        self.street = Some(value.into());
        self
    }

    pub fn city(mut self, value: impl Into<String>) -> Self {
        self.city = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn zip(mut self, value: impl Into<String>) -> Self {
        self.zip = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PeopleSearchAddressResponse`].
    pub fn build(self) -> Result<PeopleSearchAddressResponse, BuildError> {
        Ok(PeopleSearchAddressResponse {
            street: self.street,
            city: self.city,
            state: self.state,
            zip: self.zip,
        })
    }
}
