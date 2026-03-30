pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaymentCategories {
    /// Price/cost per unit of item or category.
    #[serde(default)]
    pub amount: f64,
    /// Description of item or category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Name of item or category.
    #[serde(default)]
    pub label: String,
    /// Quantity of item or category
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<i64>,
}

impl PaymentCategories {
    pub fn builder() -> PaymentCategoriesBuilder {
        <PaymentCategoriesBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaymentCategoriesBuilder {
    amount: Option<f64>,
    description: Option<String>,
    label: Option<String>,
    qty: Option<i64>,
}

impl PaymentCategoriesBuilder {
    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn qty(mut self, value: i64) -> Self {
        self.qty = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaymentCategories`].
    /// This method will fail if any of the following fields are not set:
    /// - [`amount`](PaymentCategoriesBuilder::amount)
    /// - [`label`](PaymentCategoriesBuilder::label)
    pub fn build(self) -> Result<PaymentCategories, BuildError> {
        Ok(PaymentCategories {
            amount: self
                .amount
                .ok_or_else(|| BuildError::missing_field("amount"))?,
            description: self.description,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            qty: self.qty,
        })
    }
}
