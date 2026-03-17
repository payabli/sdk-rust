pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum PaymentMethod {
    PayMethodCredit(PayMethodCredit),

    PayMethodAch(PayMethodAch),

    PayMethodStoredMethod(PayMethodStoredMethod),

    PayMethodCloud(PayMethodCloud),

    Check(Check),

    Cash(Cash),

    PayMethodBodyAllFields(PayMethodBodyAllFields),
}

impl PaymentMethod {
    pub fn is_paymethodcredit(&self) -> bool {
        matches!(self, Self::PayMethodCredit(_))
    }

    pub fn is_paymethodach(&self) -> bool {
        matches!(self, Self::PayMethodAch(_))
    }

    pub fn is_paymethodstoredmethod(&self) -> bool {
        matches!(self, Self::PayMethodStoredMethod(_))
    }

    pub fn is_paymethodcloud(&self) -> bool {
        matches!(self, Self::PayMethodCloud(_))
    }

    pub fn is_check(&self) -> bool {
        matches!(self, Self::Check(_))
    }

    pub fn is_cash(&self) -> bool {
        matches!(self, Self::Cash(_))
    }

    pub fn is_paymethodbodyallfields(&self) -> bool {
        matches!(self, Self::PayMethodBodyAllFields(_))
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

    pub fn as_paymethodach(&self) -> Option<&PayMethodAch> {
        match self {
            Self::PayMethodAch(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_paymethodach(self) -> Option<PayMethodAch> {
        match self {
            Self::PayMethodAch(value) => Some(value),
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

    pub fn as_check(&self) -> Option<&Check> {
        match self {
            Self::Check(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_check(self) -> Option<Check> {
        match self {
            Self::Check(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_cash(&self) -> Option<&Cash> {
        match self {
            Self::Cash(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_cash(self) -> Option<Cash> {
        match self {
            Self::Cash(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_paymethodbodyallfields(&self) -> Option<&PayMethodBodyAllFields> {
        match self {
            Self::PayMethodBodyAllFields(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_paymethodbodyallfields(self) -> Option<PayMethodBodyAllFields> {
        match self {
            Self::PayMethodBodyAllFields(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for PaymentMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PayMethodCredit(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::PayMethodAch(value) => write!(
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
            Self::Check(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::Cash(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::PayMethodBodyAllFields(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
