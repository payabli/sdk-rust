pub use crate::prelude::*;

/// Query parameters for CaptureOut
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CaptureOutQueryRequest {
    /// Controls what happens to a payout authorized with `sameDayACH` set to `true` when you capture it after the same-day ACH cutoff. When `true`, Payabli converts the payout to a standard ACH payment and captures it. When `false`, the capture is declined.
    ///
    /// This parameter has no effect on payouts that weren't authorized for same-day ACH.
    #[serde(rename = "autoConvertSameDayAch")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_convert_same_day_ach: Option<bool>,
}

impl CaptureOutQueryRequest {
    pub fn builder() -> CaptureOutQueryRequestBuilder {
        <CaptureOutQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CaptureOutQueryRequestBuilder {
    auto_convert_same_day_ach: Option<bool>,
}

impl CaptureOutQueryRequestBuilder {
    pub fn auto_convert_same_day_ach(mut self, value: bool) -> Self {
        self.auto_convert_same_day_ach = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CaptureOutQueryRequest`].
    pub fn build(self) -> Result<CaptureOutQueryRequest, BuildError> {
        Ok(CaptureOutQueryRequest {
            auto_convert_same_day_ach: self.auto_convert_same_day_ach,
        })
    }
}
