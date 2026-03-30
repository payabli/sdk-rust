pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DSection {
    #[serde(rename = "depositAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_account: Option<Bnk>,
    #[serde(rename = "withdrawalAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdrawal_account: Option<Bnk>,
}

impl DSection {
    pub fn builder() -> DSectionBuilder {
        <DSectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DSectionBuilder {
    deposit_account: Option<Bnk>,
    withdrawal_account: Option<Bnk>,
}

impl DSectionBuilder {
    pub fn deposit_account(mut self, value: Bnk) -> Self {
        self.deposit_account = Some(value);
        self
    }

    pub fn withdrawal_account(mut self, value: Bnk) -> Self {
        self.withdrawal_account = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DSection`].
    pub fn build(self) -> Result<DSection, BuildError> {
        Ok(DSection {
            deposit_account: self.deposit_account,
            withdrawal_account: self.withdrawal_account,
        })
    }
}
