pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ServiceCost {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Enabled>,
    #[serde(rename = "monthlyCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub monthly_cost: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reseller: Option<bool>,
    #[serde(rename = "setupCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub setup_cost: Option<f64>,
    #[serde(rename = "txCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub tx_cost: Option<f64>,
    #[serde(rename = "txPercentCost")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub tx_percent_cost: Option<f64>,
}

impl ServiceCost {
    pub fn builder() -> ServiceCostBuilder {
        <ServiceCostBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ServiceCostBuilder {
    description: Option<String>,
    enabled: Option<Enabled>,
    monthly_cost: Option<f64>,
    name: Option<String>,
    reseller: Option<bool>,
    setup_cost: Option<f64>,
    tx_cost: Option<f64>,
    tx_percent_cost: Option<f64>,
}

impl ServiceCostBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn enabled(mut self, value: Enabled) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn monthly_cost(mut self, value: f64) -> Self {
        self.monthly_cost = Some(value);
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn reseller(mut self, value: bool) -> Self {
        self.reseller = Some(value);
        self
    }

    pub fn setup_cost(mut self, value: f64) -> Self {
        self.setup_cost = Some(value);
        self
    }

    pub fn tx_cost(mut self, value: f64) -> Self {
        self.tx_cost = Some(value);
        self
    }

    pub fn tx_percent_cost(mut self, value: f64) -> Self {
        self.tx_percent_cost = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ServiceCost`].
    pub fn build(self) -> Result<ServiceCost, BuildError> {
        Ok(ServiceCost {
            description: self.description,
            enabled: self.enabled,
            monthly_cost: self.monthly_cost,
            name: self.name,
            reseller: self.reseller,
            setup_cost: self.setup_cost,
            tx_cost: self.tx_cost,
            tx_percent_cost: self.tx_percent_cost,
        })
    }
}
