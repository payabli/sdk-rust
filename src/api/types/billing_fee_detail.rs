pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct BillingFeeDetail {
    #[serde(rename = "billableEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billable_event: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(rename = "eventId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_id: Option<String>,
    /// Description of the billing fee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Category of the billing fee
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    /// Fixed price component of the fee
    #[serde(rename = "fixPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fix_price: Option<f64>,
    /// Percentage component of the fee
    #[serde(rename = "floatPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float_price: Option<f64>,
    /// Amount eligible for the fee
    #[serde(rename = "billableAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billable_amount: Option<f64>,
    /// Total fee amount charged
    #[serde(rename = "billAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bill_amount: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency: Option<String>,
    #[serde(rename = "serviceGroup")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_group: Option<String>,
}

impl BillingFeeDetail {
    pub fn builder() -> BillingFeeDetailBuilder {
        <BillingFeeDetailBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillingFeeDetailBuilder {
    billable_event: Option<String>,
    service: Option<String>,
    event_id: Option<String>,
    description: Option<String>,
    category: Option<String>,
    fix_price: Option<f64>,
    float_price: Option<f64>,
    billable_amount: Option<f64>,
    bill_amount: Option<f64>,
    frequency: Option<String>,
    service_group: Option<String>,
}

impl BillingFeeDetailBuilder {
    pub fn billable_event(mut self, value: impl Into<String>) -> Self {
        self.billable_event = Some(value.into());
        self
    }

    pub fn service(mut self, value: impl Into<String>) -> Self {
        self.service = Some(value.into());
        self
    }

    pub fn event_id(mut self, value: impl Into<String>) -> Self {
        self.event_id = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn category(mut self, value: impl Into<String>) -> Self {
        self.category = Some(value.into());
        self
    }

    pub fn fix_price(mut self, value: f64) -> Self {
        self.fix_price = Some(value);
        self
    }

    pub fn float_price(mut self, value: f64) -> Self {
        self.float_price = Some(value);
        self
    }

    pub fn billable_amount(mut self, value: f64) -> Self {
        self.billable_amount = Some(value);
        self
    }

    pub fn bill_amount(mut self, value: f64) -> Self {
        self.bill_amount = Some(value);
        self
    }

    pub fn frequency(mut self, value: impl Into<String>) -> Self {
        self.frequency = Some(value.into());
        self
    }

    pub fn service_group(mut self, value: impl Into<String>) -> Self {
        self.service_group = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`BillingFeeDetail`].
    pub fn build(self) -> Result<BillingFeeDetail, BuildError> {
        Ok(BillingFeeDetail {
            billable_event: self.billable_event,
            service: self.service,
            event_id: self.event_id,
            description: self.description,
            category: self.category,
            fix_price: self.fix_price,
            float_price: self.float_price,
            billable_amount: self.billable_amount,
            bill_amount: self.bill_amount,
            frequency: self.frequency,
            service_group: self.service_group,
        })
    }
}
