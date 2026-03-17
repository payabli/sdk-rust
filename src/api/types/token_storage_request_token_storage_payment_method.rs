pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum RequestTokenStoragePaymentMethod {
    TokenizeCard(TokenizeCard),

    TokenizeAch(TokenizeAch),

    ConvertToken(ConvertToken),
}

impl RequestTokenStoragePaymentMethod {
    pub fn is_tokenizecard(&self) -> bool {
        matches!(self, Self::TokenizeCard(_))
    }

    pub fn is_tokenizeach(&self) -> bool {
        matches!(self, Self::TokenizeAch(_))
    }

    pub fn is_converttoken(&self) -> bool {
        matches!(self, Self::ConvertToken(_))
    }

    pub fn as_tokenizecard(&self) -> Option<&TokenizeCard> {
        match self {
            Self::TokenizeCard(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_tokenizecard(self) -> Option<TokenizeCard> {
        match self {
            Self::TokenizeCard(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_tokenizeach(&self) -> Option<&TokenizeAch> {
        match self {
            Self::TokenizeAch(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_tokenizeach(self) -> Option<TokenizeAch> {
        match self {
            Self::TokenizeAch(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_converttoken(&self) -> Option<&ConvertToken> {
        match self {
            Self::ConvertToken(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_converttoken(self) -> Option<ConvertToken> {
        match self {
            Self::ConvertToken(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for RequestTokenStoragePaymentMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TokenizeCard(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::TokenizeAch(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::ConvertToken(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
