pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VCardQueryResponse {
    #[serde(rename = "Summary")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<VCardSummary>,
    #[serde(rename = "Records")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub records: Option<Vec<VCardRecord>>,
}

impl VCardQueryResponse {
    pub fn builder() -> VCardQueryResponseBuilder {
        <VCardQueryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VCardQueryResponseBuilder {
    summary: Option<VCardSummary>,
    records: Option<Vec<VCardRecord>>,
}

impl VCardQueryResponseBuilder {
    pub fn summary(mut self, value: VCardSummary) -> Self {
        self.summary = Some(value);
        self
    }

    pub fn records(mut self, value: Vec<VCardRecord>) -> Self {
        self.records = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VCardQueryResponse`].
    pub fn build(self) -> Result<VCardQueryResponse, BuildError> {
        Ok(VCardQueryResponse {
            summary: self.summary,
            records: self.records,
        })
    }
}
