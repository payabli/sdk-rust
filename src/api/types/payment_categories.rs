pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaymentCategories {
    /// Price/cost per unit of item or category.
    pub amount: f64,
    /// Description of item or category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Name of item or category.
    pub label: String,
    /// Quantity of item or category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<i64>,
}
