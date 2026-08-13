pub use crate::prelude::*;

/// A fee schedule attached to a billable event. Flat and interchange-plus
/// schedules share this shape; `feeType` is the discriminator.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct FeeSchedule {
    /// Fee schedule identifier.
    #[serde(default)]
    pub id: i64,
    /// The flat-fee component of the fee, for example `0.30`.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub value: f64,
    /// The percentage-rate component of the fee, for example `2.9`.
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub rate: f64,
    #[serde(default)]
    pub passthrough: PassthroughValue,
    /// Entity responsible for paying this fee. `null` when not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payor: Option<BillingEntity>,
    /// Entity that collects this fee. `null` when not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collector: Option<BillingEntity>,
    /// Fallback payor used when the primary payor can't cover the fee. `null`
    /// when not set.
    #[serde(rename = "overflowPayor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overflow_payor: Option<BillingEntity>,
    /// Collection cadence for the overflow payor. `null` when not set.
    #[serde(rename = "overflowCollectionSchedule")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub overflow_collection_schedule: Option<CollectionScheduleValue>,
    /// Floor on the combined fee (rate + flat value) for this schedule.
    #[serde(rename = "minimumTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub minimum_total: Option<f64>,
    /// Ceiling on the combined fee (rate + flat value) for this schedule.
    #[serde(rename = "maximumTotal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub maximum_total: Option<f64>,
    /// When this schedule starts applying. Drives ordering when an event has
    /// multiple schedules.
    #[serde(rename = "effectiveDate")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub effective_date: DateTime<Utc>,
    /// When this schedule stops applying. `null` means no end date.
    #[serde(rename = "expirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub expiration_date: Option<DateTime<Utc>>,
    /// When this schedule was created.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub created_at: DateTime<Utc>,
    /// When this schedule was last updated.
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub updated_at: DateTime<Utc>,
    /// Identifier of the schedule's creator. Surfaces in the Payabli Portal as
    /// "Configuration Owner" — informational, not a permission boundary.
    #[serde(rename = "createdBy")]
    #[serde(default)]
    pub created_by: String,
    #[serde(rename = "collectionSchedule")]
    #[serde(default)]
    pub collection_schedule: CollectionScheduleValue,
    /// Day of the month a monthly bill is applied, when set. `null` otherwise.
    #[serde(rename = "billDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_date: Option<i64>,
    /// Identifier of another fee schedule this one overrides. `null` otherwise.
    #[serde(rename = "overrideFeeScheduleId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub override_fee_schedule_id: Option<i64>,
    #[serde(rename = "feeType")]
    #[serde(default)]
    pub fee_type: FeeTypeValue,
}

impl FeeSchedule {
    pub fn builder() -> FeeScheduleBuilder {
        <FeeScheduleBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct FeeScheduleBuilder {
    id: Option<i64>,
    value: Option<f64>,
    rate: Option<f64>,
    passthrough: Option<PassthroughValue>,
    payor: Option<BillingEntity>,
    collector: Option<BillingEntity>,
    overflow_payor: Option<BillingEntity>,
    overflow_collection_schedule: Option<CollectionScheduleValue>,
    minimum_total: Option<f64>,
    maximum_total: Option<f64>,
    effective_date: Option<DateTime<Utc>>,
    expiration_date: Option<DateTime<Utc>>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
    created_by: Option<String>,
    collection_schedule: Option<CollectionScheduleValue>,
    bill_date: Option<i64>,
    override_fee_schedule_id: Option<i64>,
    fee_type: Option<FeeTypeValue>,
}

impl FeeScheduleBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn rate(mut self, value: f64) -> Self {
        self.rate = Some(value);
        self
    }

    pub fn passthrough(mut self, value: PassthroughValue) -> Self {
        self.passthrough = Some(value);
        self
    }

    pub fn payor(mut self, value: BillingEntity) -> Self {
        self.payor = Some(value);
        self
    }

    pub fn collector(mut self, value: BillingEntity) -> Self {
        self.collector = Some(value);
        self
    }

    pub fn overflow_payor(mut self, value: BillingEntity) -> Self {
        self.overflow_payor = Some(value);
        self
    }

    pub fn overflow_collection_schedule(mut self, value: CollectionScheduleValue) -> Self {
        self.overflow_collection_schedule = Some(value);
        self
    }

    pub fn minimum_total(mut self, value: f64) -> Self {
        self.minimum_total = Some(value);
        self
    }

    pub fn maximum_total(mut self, value: f64) -> Self {
        self.maximum_total = Some(value);
        self
    }

    pub fn effective_date(mut self, value: DateTime<Utc>) -> Self {
        self.effective_date = Some(value);
        self
    }

    pub fn expiration_date(mut self, value: DateTime<Utc>) -> Self {
        self.expiration_date = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<Utc>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn created_by(mut self, value: impl Into<String>) -> Self {
        self.created_by = Some(value.into());
        self
    }

    pub fn collection_schedule(mut self, value: CollectionScheduleValue) -> Self {
        self.collection_schedule = Some(value);
        self
    }

    pub fn bill_date(mut self, value: i64) -> Self {
        self.bill_date = Some(value);
        self
    }

    pub fn override_fee_schedule_id(mut self, value: i64) -> Self {
        self.override_fee_schedule_id = Some(value);
        self
    }

    pub fn fee_type(mut self, value: FeeTypeValue) -> Self {
        self.fee_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`FeeSchedule`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](FeeScheduleBuilder::id)
    /// - [`value`](FeeScheduleBuilder::value)
    /// - [`rate`](FeeScheduleBuilder::rate)
    /// - [`passthrough`](FeeScheduleBuilder::passthrough)
    /// - [`effective_date`](FeeScheduleBuilder::effective_date)
    /// - [`created_at`](FeeScheduleBuilder::created_at)
    /// - [`updated_at`](FeeScheduleBuilder::updated_at)
    /// - [`created_by`](FeeScheduleBuilder::created_by)
    /// - [`collection_schedule`](FeeScheduleBuilder::collection_schedule)
    /// - [`fee_type`](FeeScheduleBuilder::fee_type)
    pub fn build(self) -> Result<FeeSchedule, BuildError> {
        Ok(FeeSchedule {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
            rate: self.rate.ok_or_else(|| BuildError::missing_field("rate"))?,
            passthrough: self
                .passthrough
                .ok_or_else(|| BuildError::missing_field("passthrough"))?,
            payor: self.payor,
            collector: self.collector,
            overflow_payor: self.overflow_payor,
            overflow_collection_schedule: self.overflow_collection_schedule,
            minimum_total: self.minimum_total,
            maximum_total: self.maximum_total,
            effective_date: self
                .effective_date
                .ok_or_else(|| BuildError::missing_field("effective_date"))?,
            expiration_date: self.expiration_date,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            created_by: self
                .created_by
                .ok_or_else(|| BuildError::missing_field("created_by"))?,
            collection_schedule: self
                .collection_schedule
                .ok_or_else(|| BuildError::missing_field("collection_schedule"))?,
            bill_date: self.bill_date,
            override_fee_schedule_id: self.override_fee_schedule_id,
            fee_type: self
                .fee_type
                .ok_or_else(|| BuildError::missing_field("fee_type"))?,
        })
    }
}
