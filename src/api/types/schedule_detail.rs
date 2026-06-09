pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ScheduleDetail {
    /// Subscription end date in any of the accepted formats: YYYY-MM-DD, MM/DD/YYYY or the value `untilcancelled` to indicate a scheduled payment with infinite cycle.
    ///
    /// Not applicable for `BalanceDriven` subscriptions, which run until cancelled.
    #[serde(rename = "endDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    /// Frequency of the subscription.
    ///
    /// `BalanceDriven` subscriptions only accept the monthly cadences `firstofmonth`, `fifteenthofmonth`, and `endofmonth`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<Frequency>,
    /// This field is for future development, leave null. Identifier of subscription plan applied in the scheduled payment/subscription.
    #[serde(rename = "planId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan_id: Option<i64>,
    /// Subscription start date in any of the accepted formats: YYYY-MM-DD, MM/DD/YYYY. This must be a future date.
    ///
    /// Not applicable for `BalanceDriven` subscriptions, where the start date is calculated automatically from `frequency`.
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
}

impl ScheduleDetail {
    pub fn builder() -> ScheduleDetailBuilder {
        <ScheduleDetailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ScheduleDetailBuilder {
    end_date: Option<String>,
    frequency: Option<Frequency>,
    plan_id: Option<i64>,
    start_date: Option<String>,
}

impl ScheduleDetailBuilder {
    pub fn end_date(mut self, value: impl Into<String>) -> Self {
        self.end_date = Some(value.into());
        self
    }

    pub fn frequency(mut self, value: Frequency) -> Self {
        self.frequency = Some(value);
        self
    }

    pub fn plan_id(mut self, value: i64) -> Self {
        self.plan_id = Some(value);
        self
    }

    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`ScheduleDetail`].
    pub fn build(self) -> Result<ScheduleDetail, BuildError> {
        Ok(ScheduleDetail {
            end_date: self.end_date,
            frequency: self.frequency,
            plan_id: self.plan_id,
            start_date: self.start_date,
        })
    }
}
