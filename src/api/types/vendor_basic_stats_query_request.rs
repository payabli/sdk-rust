pub use crate::prelude::*;

/// Query parameters for VendorBasicStats
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VendorBasicStatsQueryRequest {
    /// List of parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, Option<String>>>,
}

impl VendorBasicStatsQueryRequest {
    pub fn builder() -> VendorBasicStatsQueryRequestBuilder {
        <VendorBasicStatsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VendorBasicStatsQueryRequestBuilder {
    parameters: Option<HashMap<String, Option<String>>>,
}

impl VendorBasicStatsQueryRequestBuilder {
    pub fn parameters(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.parameters = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VendorBasicStatsQueryRequest`].
    pub fn build(self) -> Result<VendorBasicStatsQueryRequest, BuildError> {
        Ok(VendorBasicStatsQueryRequest {
            parameters: self.parameters,
        })
    }
}
