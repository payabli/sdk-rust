pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Bnk {
    #[serde(rename = "accountNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<LinkData>,
    #[serde(rename = "bankName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<LinkData>,
    #[serde(rename = "routingAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_account: Option<LinkData>,
    #[serde(rename = "typeAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_account: Option<LinkData>,
}

impl Bnk {
    pub fn builder() -> BnkBuilder {
        <BnkBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BnkBuilder {
    account_number: Option<LinkData>,
    bank_name: Option<LinkData>,
    routing_account: Option<LinkData>,
    type_account: Option<LinkData>,
}

impl BnkBuilder {
    pub fn account_number(mut self, value: LinkData) -> Self {
        self.account_number = Some(value);
        self
    }

    pub fn bank_name(mut self, value: LinkData) -> Self {
        self.bank_name = Some(value);
        self
    }

    pub fn routing_account(mut self, value: LinkData) -> Self {
        self.routing_account = Some(value);
        self
    }

    pub fn type_account(mut self, value: LinkData) -> Self {
        self.type_account = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Bnk`].
    pub fn build(self) -> Result<Bnk, BuildError> {
        Ok(Bnk {
            account_number: self.account_number,
            bank_name: self.bank_name,
            routing_account: self.routing_account,
            type_account: self.type_account,
        })
    }
}
