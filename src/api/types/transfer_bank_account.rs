pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TransferBankAccount {
    #[serde(rename = "accountNumber")]
    pub account_number: AccountNumber,
    #[serde(rename = "routingNumber")]
    pub routing_number: RoutingAccount,
}
