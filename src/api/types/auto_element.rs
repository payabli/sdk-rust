pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AutoElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Enabled>,
    /// Type of end date
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finish: Option<Finishtype>,
    /// accepted frequencies for autopay
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<FrequencyList>,
    /// Value of pre-selected frequency
    #[serde(rename = "frequencySelected")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_selected: Option<String>,
    /// Header text for section
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    /// Range of days enabled in calendar. Leave empty to enable all days.
    #[serde(rename = "startDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<String>,
}

impl AutoElement {
    pub fn builder() -> AutoElementBuilder {
        <AutoElementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AutoElementBuilder {
    enabled: Option<Enabled>,
    finish: Option<Finishtype>,
    frequency: Option<FrequencyList>,
    frequency_selected: Option<String>,
    header: Option<String>,
    order: Option<Order>,
    start_date: Option<String>,
}

impl AutoElementBuilder {
    pub fn enabled(mut self, value: Enabled) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn finish(mut self, value: Finishtype) -> Self {
        self.finish = Some(value);
        self
    }

    pub fn frequency(mut self, value: FrequencyList) -> Self {
        self.frequency = Some(value);
        self
    }

    pub fn frequency_selected(mut self, value: impl Into<String>) -> Self {
        self.frequency_selected = Some(value.into());
        self
    }

    pub fn header(mut self, value: impl Into<String>) -> Self {
        self.header = Some(value.into());
        self
    }

    pub fn order(mut self, value: Order) -> Self {
        self.order = Some(value);
        self
    }

    pub fn start_date(mut self, value: impl Into<String>) -> Self {
        self.start_date = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AutoElement`].
    pub fn build(self) -> Result<AutoElement, BuildError> {
        Ok(AutoElement {
            enabled: self.enabled,
            finish: self.finish,
            frequency: self.frequency,
            frequency_selected: self.frequency_selected,
            header: self.header,
            order: self.order,
            start_date: self.start_date,
        })
    }
}
