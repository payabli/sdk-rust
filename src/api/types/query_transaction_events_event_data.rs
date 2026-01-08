pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum QueryTransactionEventsEventData {
    Map0(HashMap<String, serde_json::Value>),

    String(String),
}

impl QueryTransactionEventsEventData {
    pub fn is_map0(&self) -> bool {
        matches!(self, Self::Map0(_))
    }

    pub fn is_string(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn as_map0(&self) -> Option<&HashMap<String, serde_json::Value>> {
        match self {
            Self::Map0(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_map0(self) -> Option<HashMap<String, serde_json::Value>> {
        match self {
            Self::Map0(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_string(&self) -> Option<&String> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_string(self) -> Option<String> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }
}
