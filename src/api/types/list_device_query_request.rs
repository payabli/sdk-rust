pub use crate::prelude::*;

/// Query parameters for ListDevice
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ListDeviceQueryRequest {
    /// When `true`, the request retrieves an updated list of devices from the processor instead of returning a cached list of devices.
    #[serde(rename = "forceRefresh")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_refresh: Option<bool>,
}

impl ListDeviceQueryRequest {
    pub fn builder() -> ListDeviceQueryRequestBuilder {
        <ListDeviceQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ListDeviceQueryRequestBuilder {
    force_refresh: Option<bool>,
}

impl ListDeviceQueryRequestBuilder {
    pub fn force_refresh(mut self, value: bool) -> Self {
        self.force_refresh = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ListDeviceQueryRequest`].
    pub fn build(self) -> Result<ListDeviceQueryRequest, BuildError> {
        Ok(ListDeviceQueryRequest {
            force_refresh: self.force_refresh,
        })
    }
}
