pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct FrequencyList {
    /// Enable or disable the annual frequency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annually: Option<bool>,
    /// Enable or disable the every-two-weeks frequency.
    #[serde(rename = "every2Weeks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub every_2_weeks: Option<bool>,
    /// Enable or disable the every-three-months frequency.
    #[serde(rename = "every3Months")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub every_3_months: Option<bool>,
    /// Enable or disable the every-six-months frequency.
    #[serde(rename = "every6Months")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub every_6_months: Option<bool>,
    /// Enable or disable the monthly frequency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly: Option<bool>,
    /// Enable or disable the one-time frequency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub onetime: Option<bool>,
    /// Enable or disable the weekly frequency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly: Option<bool>,
}

impl FrequencyList {
    pub fn builder() -> FrequencyListBuilder {
        <FrequencyListBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FrequencyListBuilder {
    annually: Option<bool>,
    every_2_weeks: Option<bool>,
    every_3_months: Option<bool>,
    every_6_months: Option<bool>,
    monthly: Option<bool>,
    onetime: Option<bool>,
    weekly: Option<bool>,
}

impl FrequencyListBuilder {
    pub fn annually(mut self, value: bool) -> Self {
        self.annually = Some(value);
        self
    }

    pub fn every_2_weeks(mut self, value: bool) -> Self {
        self.every_2_weeks = Some(value);
        self
    }

    pub fn every_3_months(mut self, value: bool) -> Self {
        self.every_3_months = Some(value);
        self
    }

    pub fn every_6_months(mut self, value: bool) -> Self {
        self.every_6_months = Some(value);
        self
    }

    pub fn monthly(mut self, value: bool) -> Self {
        self.monthly = Some(value);
        self
    }

    pub fn onetime(mut self, value: bool) -> Self {
        self.onetime = Some(value);
        self
    }

    pub fn weekly(mut self, value: bool) -> Self {
        self.weekly = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FrequencyList`].
    pub fn build(self) -> Result<FrequencyList, BuildError> {
        Ok(FrequencyList {
            annually: self.annually,
            every_2_weeks: self.every_2_weeks,
            every_3_months: self.every_3_months,
            every_6_months: self.every_6_months,
            monthly: self.monthly,
            onetime: self.onetime,
            weekly: self.weekly,
        })
    }
}
