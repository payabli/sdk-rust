pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum RequestPaymentAuthorizePaymentMethod {
    PayMethodCredit(PayMethodCredit),

    PayMethodStoredMethod(PayMethodStoredMethod),

    PayMethodCloud(PayMethodCloud),
}

impl RequestPaymentAuthorizePaymentMethod {
    pub fn is_paymethodcredit(&self) -> bool {
        matches!(self, Self::PayMethodCredit(_))
    }

    pub fn is_paymethodstoredmethod(&self) -> bool {
        matches!(self, Self::PayMethodStoredMethod(_))
    }

    pub fn is_paymethodcloud(&self) -> bool {
        matches!(self, Self::PayMethodCloud(_))
    }

    pub fn as_paymethodcredit(&self) -> Option<&PayMethodCredit> {
        match self {
            Self::PayMethodCredit(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_paymethodcredit(self) -> Option<PayMethodCredit> {
        match self {
            Self::PayMethodCredit(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_paymethodstoredmethod(&self) -> Option<&PayMethodStoredMethod> {
        match self {
            Self::PayMethodStoredMethod(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_paymethodstoredmethod(self) -> Option<PayMethodStoredMethod> {
        match self {
            Self::PayMethodStoredMethod(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_paymethodcloud(&self) -> Option<&PayMethodCloud> {
        match self {
            Self::PayMethodCloud(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_paymethodcloud(self) -> Option<PayMethodCloud> {
        match self {
            Self::PayMethodCloud(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for RequestPaymentAuthorizePaymentMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PayMethodCredit(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::PayMethodStoredMethod(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::PayMethodCloud(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
