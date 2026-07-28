pub use crate::prelude::*;

/// The Pay In and Pay Out services the bank account applies to. Include at least one entry across the two lists.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BankAccountServices {
    /// Pay In services the account is used for.
    #[serde(rename = "moneyIn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub money_in: Option<Vec<MoneyInService>>,
    /// Pay Out services the account is used for.
    #[serde(rename = "moneyOut")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub money_out: Option<Vec<MoneyOutService>>,
}

impl BankAccountServices {
    pub fn builder() -> BankAccountServicesBuilder {
        <BankAccountServicesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BankAccountServicesBuilder {
    money_in: Option<Vec<MoneyInService>>,
    money_out: Option<Vec<MoneyOutService>>,
}

impl BankAccountServicesBuilder {
    pub fn money_in(mut self, value: Vec<MoneyInService>) -> Self {
        self.money_in = Some(value);
        self
    }

    pub fn money_out(mut self, value: Vec<MoneyOutService>) -> Self {
        self.money_out = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BankAccountServices`].
    pub fn build(self) -> Result<BankAccountServices, BuildError> {
        Ok(BankAccountServices {
            money_in: self.money_in,
            money_out: self.money_out,
        })
    }
}
