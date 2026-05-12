pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct SummaryOrg {
    #[serde(rename = "amountSubs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount_subs: Option<f64>,
    #[serde(rename = "amountTx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub amount_tx: Option<f64>,
    #[serde(rename = "childOrgs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_orgs: Option<i64>,
    #[serde(rename = "childPaypoints")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub child_paypoints: Option<i64>,
    #[serde(rename = "countSubs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_subs: Option<i64>,
    #[serde(rename = "countTx")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count_tx: Option<i64>,
}

impl SummaryOrg {
    pub fn builder() -> SummaryOrgBuilder {
        <SummaryOrgBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SummaryOrgBuilder {
    amount_subs: Option<f64>,
    amount_tx: Option<f64>,
    child_orgs: Option<i64>,
    child_paypoints: Option<i64>,
    count_subs: Option<i64>,
    count_tx: Option<i64>,
}

impl SummaryOrgBuilder {
    pub fn amount_subs(mut self, value: f64) -> Self {
        self.amount_subs = Some(value);
        self
    }

    pub fn amount_tx(mut self, value: f64) -> Self {
        self.amount_tx = Some(value);
        self
    }

    pub fn child_orgs(mut self, value: i64) -> Self {
        self.child_orgs = Some(value);
        self
    }

    pub fn child_paypoints(mut self, value: i64) -> Self {
        self.child_paypoints = Some(value);
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

    /// Consumes the builder and constructs a [`SummaryOrg`].
    pub fn build(self) -> Result<SummaryOrg, BuildError> {
        Ok(SummaryOrg {
            amount_subs: self.amount_subs,
            amount_tx: self.amount_tx,
            child_orgs: self.child_orgs,
            child_paypoints: self.child_paypoints,
            count_subs: self.count_subs,
            count_tx: self.count_tx,
        })
    }
}
