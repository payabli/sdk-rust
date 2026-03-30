pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayCategory {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "optionalPay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optional_pay: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[serde(rename = "showDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub show_description: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl PayCategory {
    pub fn builder() -> PayCategoryBuilder {
        <PayCategoryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayCategoryBuilder {
    description: Option<String>,
    label: Option<String>,
    name: Option<String>,
    optional_pay: Option<bool>,
    order: Option<Order>,
    quantity: Option<i64>,
    show_description: Option<bool>,
    r#type: Option<String>,
    value: Option<String>,
}

impl PayCategoryBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn optional_pay(mut self, value: bool) -> Self {
        self.optional_pay = Some(value);
        self
    }

    pub fn order(mut self, value: Order) -> Self {
        self.order = Some(value);
        self
    }

    pub fn quantity(mut self, value: i64) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn show_description(mut self, value: bool) -> Self {
        self.show_description = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PayCategory`].
    pub fn build(self) -> Result<PayCategory, BuildError> {
        Ok(PayCategory {
            description: self.description,
            label: self.label,
            name: self.name,
            optional_pay: self.optional_pay,
            order: self.order,
            quantity: self.quantity,
            show_description: self.show_description,
            r#type: self.r#type,
            value: self.value,
        })
    }
}
