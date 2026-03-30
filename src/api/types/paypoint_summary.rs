pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaypointSummary {
    #[serde(rename = "amountSubs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_subs: Option<f64>,
    #[serde(rename = "amountTx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tx: Option<f64>,
    #[serde(rename = "countSubs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_subs: Option<i64>,
    #[serde(rename = "countTx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_tx: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customers: Option<i64>,
}

impl PaypointSummary {
    pub fn builder() -> PaypointSummaryBuilder {
        <PaypointSummaryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaypointSummaryBuilder {
    amount_subs: Option<f64>,
    amount_tx: Option<f64>,
    count_subs: Option<i64>,
    count_tx: Option<i64>,
    customers: Option<i64>,
}

impl PaypointSummaryBuilder {
    pub fn amount_subs(mut self, value: f64) -> Self {
        self.amount_subs = Some(value);
        self
    }

    pub fn amount_tx(mut self, value: f64) -> Self {
        self.amount_tx = Some(value);
        self
    }

    pub fn count_subs(mut self, value: i64) -> Self {
        self.count_subs = Some(value);
        self
    }

    pub fn count_tx(mut self, value: i64) -> Self {
        self.count_tx = Some(value);
        self
    }

    pub fn customers(mut self, value: i64) -> Self {
        self.customers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaypointSummary`].
    pub fn build(self) -> Result<PaypointSummary, BuildError> {
        Ok(PaypointSummary {
            amount_subs: self.amount_subs,
            amount_tx: self.amount_tx,
            count_subs: self.count_subs,
            count_tx: self.count_tx,
            customers: self.customers,
        })
    }
}
