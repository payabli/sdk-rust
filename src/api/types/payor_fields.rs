pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayorFields {
    /// Flag indicating if the input field will show in container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display: Option<bool>,
    /// Flag indicating if the value in input field is read-only or not.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed: Option<bool>,
    /// Flag indicating if the input field is a customer identifier
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<bool>,
    /// Label to display for field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Name of field to show. Should be one of the standard customer fields or a custom field name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    /// Flag indicating if the input field is required for validation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    /// Type of validation to apply to the input field Accepted values:
    ///
    /// - alpha for alphabetical
    ///
    /// - numbers for numeric
    ///
    /// - text for alphanumeric
    ///
    /// - email for masked email address input
    ///
    /// - phone for US phone numbers
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validation: Option<String>,
    /// Pre-populated value for field
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Numeric value indicating the size of input relative to the container. Accepted values:
    ///
    /// - 4 = 1/3
    ///
    /// - 6 = 1/2
    ///
    /// - 8 = 2/3
    ///
    /// - 12 = 3/3
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<i64>,
}

impl PayorFields {
    pub fn builder() -> PayorFieldsBuilder {
        <PayorFieldsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayorFieldsBuilder {
    display: Option<bool>,
    fixed: Option<bool>,
    identifier: Option<bool>,
    label: Option<String>,
    name: Option<String>,
    order: Option<Order>,
    required: Option<bool>,
    validation: Option<String>,
    value: Option<String>,
    width: Option<i64>,
}

impl PayorFieldsBuilder {
    pub fn display(mut self, value: bool) -> Self {
        self.display = Some(value);
        self
    }

    pub fn fixed(mut self, value: bool) -> Self {
        self.fixed = Some(value);
        self
    }

    pub fn identifier(mut self, value: bool) -> Self {
        self.identifier = Some(value);
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

    pub fn order(mut self, value: Order) -> Self {
        self.order = Some(value);
        self
    }

    pub fn required(mut self, value: bool) -> Self {
        self.required = Some(value);
        self
    }

    pub fn validation(mut self, value: impl Into<String>) -> Self {
        self.validation = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn width(mut self, value: i64) -> Self {
        self.width = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayorFields`].
    pub fn build(self) -> Result<PayorFields, BuildError> {
        Ok(PayorFields {
            display: self.display,
            fixed: self.fixed,
            identifier: self.identifier,
            label: self.label,
            name: self.name,
            order: self.order,
            required: self.required,
            validation: self.validation,
            value: self.value,
            width: self.width,
        })
    }
}
