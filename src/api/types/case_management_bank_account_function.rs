pub use crate::prelude::*;

/// What the bank account is used for. `None` isn't accepted on a request.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CaseManagementBankAccountFunction {
    Deposits,
    Withdrawals,
    DepositsAndWithdrawals,
    Remittances,
    RemittancesAndDeposits,
    RemittancesAndWithdrawals,
    RemittancesDepositsAndWithdrawals,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for CaseManagementBankAccountFunction {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Deposits => serializer.serialize_str("Deposits"),
            Self::Withdrawals => serializer.serialize_str("Withdrawals"),
            Self::DepositsAndWithdrawals => serializer.serialize_str("DepositsAndWithdrawals"),
            Self::Remittances => serializer.serialize_str("Remittances"),
            Self::RemittancesAndDeposits => serializer.serialize_str("RemittancesAndDeposits"),
            Self::RemittancesAndWithdrawals => {
                serializer.serialize_str("RemittancesAndWithdrawals")
            }
            Self::RemittancesDepositsAndWithdrawals => {
                serializer.serialize_str("RemittancesDepositsAndWithdrawals")
            }
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for CaseManagementBankAccountFunction {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "Deposits" => Ok(Self::Deposits),
            "Withdrawals" => Ok(Self::Withdrawals),
            "DepositsAndWithdrawals" => Ok(Self::DepositsAndWithdrawals),
            "Remittances" => Ok(Self::Remittances),
            "RemittancesAndDeposits" => Ok(Self::RemittancesAndDeposits),
            "RemittancesAndWithdrawals" => Ok(Self::RemittancesAndWithdrawals),
            "RemittancesDepositsAndWithdrawals" => Ok(Self::RemittancesDepositsAndWithdrawals),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for CaseManagementBankAccountFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Deposits => write!(f, "Deposits"),
            Self::Withdrawals => write!(f, "Withdrawals"),
            Self::DepositsAndWithdrawals => write!(f, "DepositsAndWithdrawals"),
            Self::Remittances => write!(f, "Remittances"),
            Self::RemittancesAndDeposits => write!(f, "RemittancesAndDeposits"),
            Self::RemittancesAndWithdrawals => write!(f, "RemittancesAndWithdrawals"),
            Self::RemittancesDepositsAndWithdrawals => {
                write!(f, "RemittancesDepositsAndWithdrawals")
            }
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
