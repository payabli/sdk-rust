pub use crate::prelude::*;

/// Query parameters for BasicStats
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BasicStatsQueryRequest {
    /// Used with `custom` mode. The end date for the range.
    /// Valid formats:
    /// - YYYY-mm-dd
    /// - YYYY/mm/dd
    /// - mm-dd-YYYY
    /// - mm/dd/YYYY
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// List of parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, Option<String>>>,
    /// Used with `custom` mode. The start date for the range.
    /// Valid formats:
    /// - YYYY-mm-dd
    /// - YYYY/mm/dd
    /// -  mm-dd-YYYY
    /// - mm/dd/YYYY
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
}

impl BasicStatsQueryRequest {
    pub fn builder() -> BasicStatsQueryRequestBuilder {
        <BasicStatsQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BasicStatsQueryRequestBuilder {
    end_date: Option<String>,
    parameters: Option<HashMap<String, Option<String>>>,
    start_date: Option<String>,
}

impl BasicStatsQueryRequestBuilder {
    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
        self
    }

    pub fn parameters(mut self, value: HashMap<String, Option<String>>) -> Self {
        self.parameters = Some(value);
        self
    }

    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BasicStatsQueryRequest`].
    pub fn build(self) -> Result<BasicStatsQueryRequest, BuildError> {
        Ok(BasicStatsQueryRequest {
            end_date: self.end_date,
            parameters: self.parameters,
            start_date: self.start_date,
        })
    }
}
