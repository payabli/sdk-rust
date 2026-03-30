pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum RequestPaymentAuthorizePaymentMethod {
    PayMethodCredit(PayMethodCredit),

    PayMethodStoredMethod(PayMethodStoredMethod),

    PayMethodCloud(PayMethodCloud),
}

impl RequestPaymentAuthorizePaymentMethod {
    pub fn is_pay_method_credit(&self) -> bool {
        matches!(self, Self::PayMethodCredit(_))
    }

    pub fn is_pay_method_stored_method(&self) -> bool {
        matches!(self, Self::PayMethodStoredMethod(_))
    }

    pub fn is_pay_method_cloud(&self) -> bool {
        matches!(self, Self::PayMethodCloud(_))
    }

    pub fn as_pay_method_credit(&self) -> Option<&PayMethodCredit> {
        match self {
            Self::PayMethodCredit(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_pay_method_credit(self) -> Option<PayMethodCredit> {
        match self {
            Self::PayMethodCredit(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_pay_method_stored_method(&self) -> Option<&PayMethodStoredMethod> {
        match self {
            Self::PayMethodStoredMethod(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_pay_method_stored_method(self) -> Option<PayMethodStoredMethod> {
        match self {
            Self::PayMethodStoredMethod(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_pay_method_cloud(&self) -> Option<&PayMethodCloud> {
        match self {
            Self::PayMethodCloud(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_pay_method_cloud(self) -> Option<PayMethodCloud> {
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
