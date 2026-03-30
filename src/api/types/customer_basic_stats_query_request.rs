pub use crate::prelude::*;

/// Query parameters for CustomerBasicStats
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CustomerBasicStatsQueryRequest {
    /// List of parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, Option<String>>>,
}

impl CustomerBasicStatsQueryRequest {
    pub fn builder() -> CustomerBasicStatsQueryRequestBuilder {
        <CustomerBasicStatsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomerBasicStatsQueryRequestBuilder {
    parameters: Option<HashMap<String, Option<String>>>,
}

impl CustomerBasicStatsQueryRequestBuilder {
    pub fn parameters(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.parameters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CustomerBasicStatsQueryRequest`].
    pub fn build(self) -> Result<CustomerBasicStatsQueryRequest, BuildError> {
        Ok(CustomerBasicStatsQueryRequest {
            parameters: self.parameters,
        })
    }
}
