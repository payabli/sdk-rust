pub use crate::prelude::*;

/// The results of a people search.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PeopleSearchResultsResponse {
    /// The matched records.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub results: Option<Vec<PeopleSearchResultResponse>>,
    /// The total number of matches.
    #[serde(default)]
    pub total: i64,
    /// Whether the person was sent to the identity vendor for evaluation.
    #[serde(rename = "isEnrichedByVendor")]
    #[serde(default)]
    pub is_enriched_by_vendor: bool,
}

impl PeopleSearchResultsResponse {
    pub fn builder() -> PeopleSearchResultsResponseBuilder {
        <PeopleSearchResultsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PeopleSearchResultsResponseBuilder {
    results: Option<Vec<PeopleSearchResultResponse>>,
    total: Option<i64>,
    is_enriched_by_vendor: Option<bool>,
}

impl PeopleSearchResultsResponseBuilder {
    pub fn results(mut self, value: Vec<PeopleSearchResultResponse>) -> Self {
        self.results = Some(value);
        self
    }

    pub fn total(mut self, value: i64) -> Self {
        self.total = Some(value);
        self
    }

    pub fn is_enriched_by_vendor(mut self, value: bool) -> Self {
        self.is_enriched_by_vendor = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PeopleSearchResultsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`total`](PeopleSearchResultsResponseBuilder::total)
    /// - [`is_enriched_by_vendor`](PeopleSearchResultsResponseBuilder::is_enriched_by_vendor)
    pub fn build(self) -> Result<PeopleSearchResultsResponse, BuildError> {
        Ok(PeopleSearchResultsResponse {
            results: self.results,
            total: self
                .total
                .ok_or_else(|| BuildError::missing_field("total"))?,
            is_enriched_by_vendor: self
                .is_enriched_by_vendor
                .ok_or_else(|| BuildError::missing_field("is_enriched_by_vendor"))?,
        })
    }
}
