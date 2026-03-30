pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TierItem {
    #[serde(rename = "amountxAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amountx_auth: Option<f64>,
    #[serde(rename = "highPayRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub high_pay_range: Option<f64>,
    #[serde(rename = "lowPayRange")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub low_pay_range: Option<f64>,
    #[serde(rename = "percentxAuth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentx_auth: Option<f64>,
}

impl TierItem {
    pub fn builder() -> TierItemBuilder {
        <TierItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TierItemBuilder {
    amountx_auth: Option<f64>,
    high_pay_range: Option<f64>,
    low_pay_range: Option<f64>,
    percentx_auth: Option<f64>,
}

impl TierItemBuilder {
    pub fn amountx_auth(mut self, value: f64) -> Self {
        self.amountx_auth = Some(value);
        self
    }

    pub fn high_pay_range(mut self, value: f64) -> Self {
        self.high_pay_range = Some(value);
        self
    }

    pub fn low_pay_range(mut self, value: f64) -> Self {
        self.low_pay_range = Some(value);
        self
    }

    pub fn percentx_auth(mut self, value: f64) -> Self {
        self.percentx_auth = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TierItem`].
    pub fn build(self) -> Result<TierItem, BuildError> {
        Ok(TierItem {
            amountx_auth: self.amountx_auth,
            high_pay_range: self.high_pay_range,
            low_pay_range: self.low_pay_range,
            percentx_auth: self.percentx_auth,
        })
    }
}
