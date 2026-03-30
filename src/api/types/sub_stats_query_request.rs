pub use crate::prelude::*;

/// Query parameters for SubStats
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SubStatsQueryRequest {
    /// List of parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, Option<String>>>,
}

impl SubStatsQueryRequest {
    pub fn builder() -> SubStatsQueryRequestBuilder {
        <SubStatsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SubStatsQueryRequestBuilder {
    parameters: Option<HashMap<String, Option<String>>>,
}

impl SubStatsQueryRequestBuilder {
    pub fn parameters(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.parameters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SubStatsQueryRequest`].
    pub fn build(self) -> Result<SubStatsQueryRequest, BuildError> {
        Ok(SubStatsQueryRequest {
            parameters: self.parameters,
        })
    }
}
