pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AmountElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<PayCategory>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<Enabled>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
}
