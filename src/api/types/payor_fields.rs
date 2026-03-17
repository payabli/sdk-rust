pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
