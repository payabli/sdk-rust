pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum RequestSchedulePaymentMethod {
    PayMethodCredit(PayMethodCredit),

    PayMethodAch(PayMethodAch),

    RequestSchedulePaymentMethodInitiator(RequestSchedulePaymentMethodInitiator),
}

impl RequestSchedulePaymentMethod {
    pub fn is_paymethodcredit(&self) -> bool {
        matches!(self, Self::PayMethodCredit(_))
    }

    pub fn is_paymethodach(&self) -> bool {
        matches!(self, Self::PayMethodAch(_))
    }

    pub fn is_requestschedulepaymentmethodinitiator(&self) -> bool {
        matches!(self, Self::RequestSchedulePaymentMethodInitiator(_))
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

    pub fn as_requestschedulepaymentmethodinitiator(
        &self,
    ) -> Option<&RequestSchedulePaymentMethodInitiator> {
        match self {
            Self::RequestSchedulePaymentMethodInitiator(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_requestschedulepaymentmethodinitiator(
        self,
    ) -> Option<RequestSchedulePaymentMethodInitiator> {
        match self {
            Self::RequestSchedulePaymentMethodInitiator(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for RequestSchedulePaymentMethod {
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
            Self::RequestSchedulePaymentMethodInitiator(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
