pub use crate::prelude::*;

/// Request for CaptureAllOut (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CaptureAllOutRequest {
    /// Controls what happens to a payout authorized with `sameDayACH` set to `true` when you capture it after the same-day ACH cutoff. When `true`, Payabli converts the payout to a standard ACH payment and captures it. When `false`, the capture is declined.
    ///
    /// This parameter has no effect on payouts that weren't authorized for same-day ACH.
    #[serde(rename = "autoConvertSameDayAch")]
    #[serde(skip)]
    pub auto_convert_same_day_ach: Option<bool>,
    #[serde(default)]
    pub body: Vec<String>,
}

impl CaptureAllOutRequest {
    pub fn builder() -> CaptureAllOutRequestBuilder {
        <CaptureAllOutRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CaptureAllOutRequestBuilder {
    auto_convert_same_day_ach: Option<bool>,
    body: Option<Vec<String>>,
}

impl CaptureAllOutRequestBuilder {
    pub fn auto_convert_same_day_ach(mut self, value: bool) -> Self {
        self.auto_convert_same_day_ach = Some(value);
        self
    }

    pub fn body(mut self, value: Vec<String>) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CaptureAllOutRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](CaptureAllOutRequestBuilder::body)
    pub fn build(self) -> Result<CaptureAllOutRequest, BuildError> {
        Ok(CaptureAllOutRequest {
            auto_convert_same_day_ach: self.auto_convert_same_day_ach,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
