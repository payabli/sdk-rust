pub use crate::prelude::*;

/// A page of people.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListPeopleResponse {
    /// The people on this page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<PersonSummaryResponse>>,
    /// Pagination metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pagination: Option<Pagination>,
}

impl ListPeopleResponse {
    pub fn builder() -> ListPeopleResponseBuilder {
        <ListPeopleResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListPeopleResponseBuilder {
    data: Option<Vec<PersonSummaryResponse>>,
    pagination: Option<Pagination>,
}

impl ListPeopleResponseBuilder {
    pub fn data(mut self, value: Vec<PersonSummaryResponse>) -> Self {
        self.data = Some(value);
        self
    }

    pub fn pagination(mut self, value: Pagination) -> Self {
        self.pagination = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListPeopleResponse`].
    pub fn build(self) -> Result<ListPeopleResponse, BuildError> {
        Ok(ListPeopleResponse {
            data: self.data,
            pagination: self.pagination,
        })
    }
}
