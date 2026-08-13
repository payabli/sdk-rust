pub use crate::prelude::*;

/// A chargeable action covered by a billing profile, with the fee schedule(s)
/// that apply to it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BillableEvent {
    /// Event identifier.
    #[serde(default)]
    pub id: i64,
    /// Internal label for the event, for example `payin-card-auth-all`.
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub vertical: ServiceGroupValue,
    #[serde(default)]
    pub service: ServiceValue,
    #[serde(rename = "serviceType")]
    #[serde(default)]
    pub service_type: ServiceTypeValue,
    #[serde(rename = "eventType")]
    #[serde(default)]
    pub event_type: EventTypeValue,
    #[serde(rename = "eventGroup")]
    #[serde(default)]
    pub event_group: EventGroupValue,
    #[serde(rename = "eventSource")]
    #[serde(default)]
    pub event_source: EventSourceValue,
    #[serde(rename = "regionType")]
    #[serde(default)]
    pub region_type: RegionTypeValue,
    /// The fee schedule(s) that apply to this event.
    #[serde(rename = "feeSchedules")]
    #[serde(default)]
    pub fee_schedules: Vec<FeeSchedule>,
}

impl BillableEvent {
    pub fn builder() -> BillableEventBuilder {
        <BillableEventBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillableEventBuilder {
    id: Option<i64>,
    name: Option<String>,
    vertical: Option<ServiceGroupValue>,
    service: Option<ServiceValue>,
    service_type: Option<ServiceTypeValue>,
    event_type: Option<EventTypeValue>,
    event_group: Option<EventGroupValue>,
    event_source: Option<EventSourceValue>,
    region_type: Option<RegionTypeValue>,
    fee_schedules: Option<Vec<FeeSchedule>>,
}

impl BillableEventBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn vertical(mut self, value: ServiceGroupValue) -> Self {
        self.vertical = Some(value);
        self
    }

    pub fn service(mut self, value: ServiceValue) -> Self {
        self.service = Some(value);
        self
    }

    pub fn service_type(mut self, value: ServiceTypeValue) -> Self {
        self.service_type = Some(value);
        self
    }

    pub fn event_type(mut self, value: EventTypeValue) -> Self {
        self.event_type = Some(value);
        self
    }

    pub fn event_group(mut self, value: EventGroupValue) -> Self {
        self.event_group = Some(value);
        self
    }

    pub fn event_source(mut self, value: EventSourceValue) -> Self {
        self.event_source = Some(value);
        self
    }

    pub fn region_type(mut self, value: RegionTypeValue) -> Self {
        self.region_type = Some(value);
        self
    }

    pub fn fee_schedules(mut self, value: Vec<FeeSchedule>) -> Self {
        self.fee_schedules = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BillableEvent`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](BillableEventBuilder::id)
    /// - [`name`](BillableEventBuilder::name)
    /// - [`vertical`](BillableEventBuilder::vertical)
    /// - [`service`](BillableEventBuilder::service)
    /// - [`service_type`](BillableEventBuilder::service_type)
    /// - [`event_type`](BillableEventBuilder::event_type)
    /// - [`event_group`](BillableEventBuilder::event_group)
    /// - [`event_source`](BillableEventBuilder::event_source)
    /// - [`region_type`](BillableEventBuilder::region_type)
    /// - [`fee_schedules`](BillableEventBuilder::fee_schedules)
    pub fn build(self) -> Result<BillableEvent, BuildError> {
        Ok(BillableEvent {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            name: self.name.ok_or_else(|| BuildError::missing_field("name"))?,
            vertical: self
                .vertical
                .ok_or_else(|| BuildError::missing_field("vertical"))?,
            service: self
                .service
                .ok_or_else(|| BuildError::missing_field("service"))?,
            service_type: self
                .service_type
                .ok_or_else(|| BuildError::missing_field("service_type"))?,
            event_type: self
                .event_type
                .ok_or_else(|| BuildError::missing_field("event_type"))?,
            event_group: self
                .event_group
                .ok_or_else(|| BuildError::missing_field("event_group"))?,
            event_source: self
                .event_source
                .ok_or_else(|| BuildError::missing_field("event_source"))?,
            region_type: self
                .region_type
                .ok_or_else(|| BuildError::missing_field("region_type"))?,
            fee_schedules: self
                .fee_schedules
                .ok_or_else(|| BuildError::missing_field("fee_schedules"))?,
        })
    }
}
