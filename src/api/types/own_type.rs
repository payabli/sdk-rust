pub use crate::prelude::*;

/// The business ownership type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OwnType {
    #[serde(rename = "Limited Liability Company")]
    LimitedLiabilityCompany,
    #[serde(rename = "Non-Profit Org")]
    NonProfitOrg,
    Partnership,
    #[serde(rename = "Private Corp")]
    PrivateCorp,
    #[serde(rename = "Public Corp")]
    PublicCorp,
    #[serde(rename = "Tax Exempt")]
    TaxExempt,
    Government,
    #[serde(rename = "Sole Proprietor")]
    SoleProprietor,
}
impl fmt::Display for OwnType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::LimitedLiabilityCompany => "Limited Liability Company",
            Self::NonProfitOrg => "Non-Profit Org",
            Self::Partnership => "Partnership",
            Self::PrivateCorp => "Private Corp",
            Self::PublicCorp => "Public Corp",
            Self::TaxExempt => "Tax Exempt",
            Self::Government => "Government",
            Self::SoleProprietor => "Sole Proprietor",
        };
        write!(f, "{}", s)
    }
}
