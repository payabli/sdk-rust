pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayoutScheduleDetail {
    /// Subscription start date in any of the accepted formats: YYYY-MM-DD, MM/DD/YYYY. This must be a future date.
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
    /// Subscription end date in any of the accepted formats: YYYY-MM-DD, MM/DD/YYYY or the value `untilcancelled` to indicate a scheduled payout with infinite cycle.
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Frequency of the payout subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequency>,
}

impl PayoutScheduleDetail {
    pub fn builder() -> PayoutScheduleDetailBuilder {
        <PayoutScheduleDetailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayoutScheduleDetailBuilder {
    start_date: Option<String>,
    end_date: Option<String>,
    frequency: Option<Frequency>,
}

impl PayoutScheduleDetailBuilder {
    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());
        self
    }

    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
        self
    }

    pub fn frequency(mut self, value: Frequency) -> Self {
        self.frequency = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayoutScheduleDetail`].
    pub fn build(self) -> Result<PayoutScheduleDetail, BuildError> {
        Ok(PayoutScheduleDetail {
            start_date: self.start_date,
            end_date: self.end_date,
            frequency: self.frequency,
        })
    }
}
