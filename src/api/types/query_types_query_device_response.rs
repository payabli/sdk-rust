pub use crate::prelude::*;

/// Response body for queries about cloud devices.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryDeviceResponse {
    #[serde(rename = "Summary")]
    #[serde(default)]
    pub summary: QuerySummary,
    #[serde(rename = "Records")]
    #[serde(default)]
    pub records: Vec<DeviceQueryRecord>,
}

impl QueryDeviceResponse {
    pub fn builder() -> QueryDeviceResponseBuilder {
        <QueryDeviceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryDeviceResponseBuilder {
    summary: Option<QuerySummary>,
    records: Option<Vec<DeviceQueryRecord>>,
}

impl QueryDeviceResponseBuilder {
    pub fn summary(mut self, value: QuerySummary) -> Self {
        self.summary = Some(value);
        self
    }

    pub fn records(mut self, value: Vec<DeviceQueryRecord>) -> Self {
        self.records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryDeviceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`summary`](QueryDeviceResponseBuilder::summary)
    /// - [`records`](QueryDeviceResponseBuilder::records)
    pub fn build(self) -> Result<QueryDeviceResponse, BuildError> {
        Ok(QueryDeviceResponse {
            summary: self
                .summary
                .ok_or_else(|| BuildError::missing_field("summary"))?,
            records: self
                .records
                .ok_or_else(|| BuildError::missing_field("records"))?,
        })
    }
}
