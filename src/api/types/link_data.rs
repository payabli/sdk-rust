pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct LinkData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ro: Option<ReadOnly>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rq: Option<RequiredElement>,
    /// The type of validation applied to the field. Available values:
    /// - text
    /// - alpha
    /// - ein
    /// - url
    /// - phone
    /// - alphanumeric
    /// - zipcode
    /// - numbers
    /// - float
    /// - ssn
    /// - email
    /// - routing
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validator: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ValueTemplates>,
}
