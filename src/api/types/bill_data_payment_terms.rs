pub use crate::prelude::*;

/// Payment terms for invoice. If no terms are defined, then response data for this field defaults to `NET30`.
///
/// **Available Values:**
///
/// - `PIA`: Payment in advance
///
/// - `CIA`: Cash in advance
///
/// - `UR`: Upon receipt
///
/// - `NET10`: 10 days after invoice date
///
/// - `NET20`: 20 days after invoice date
///
/// - `NET30`: 30 days after invoice date
///
/// - `NET45`: 45 days after invoice date
///
/// - `NET60`: 60 days after invoice date
///
/// - `NET90`: 90 days after invoice date
///
/// - `EOM`: Due end of this month
///
/// - `MFI`: 1st of the month following the invoice date
///
/// - `5MFI`: 5th of the month following the invoice date
///
/// - `10MFI`: 10th of the month following the invoice date
///
/// - `15MFI`: 15th of the month following the invoice date
///
/// - `20MFI`: 20th of the month following the invoice date
///
/// - `2/10NET30`: 2% discount if paid within 10 days, otherwise net 30 days
///
/// - `UF`: Until further notice
///
/// - `10UF`: 10 days until further notice
///
/// - `20UF`: 20 days until further notice
///
/// - `25UF`: 25 days until further notice
///
/// - `50UF`: 50 days until further notice
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BillDataPaymentTerms {
    #[serde(rename = "PIA")]
    Pia,
    #[serde(rename = "CIA")]
    Cia,
    #[serde(rename = "UR")]
    Ur,
    #[serde(rename = "NET10")]
    Net10,
    #[serde(rename = "NET20")]
    Net20,
    #[serde(rename = "NET30")]
    Net30,
    #[serde(rename = "NET45")]
    Net45,
    #[serde(rename = "NET60")]
    Net60,
    #[serde(rename = "NET90")]
    Net90,
    #[serde(rename = "EOM")]
    Eom,
    #[serde(rename = "MFI")]
    Mfi,
    #[serde(rename = "5MFI")]
    FiveMfi,
    #[serde(rename = "10MFI")]
    TenMfi,
    #[serde(rename = "15MFI")]
    FifteenMfi,
    #[serde(rename = "20MFI")]
    TwentyMfi,
    #[serde(rename = "2/10NET30")]
    Two10Net30,
    #[serde(rename = "UF")]
    Uf,
    #[serde(rename = "10UF")]
    TenUf,
    #[serde(rename = "20UF")]
    TwentyUf,
    #[serde(rename = "25UF")]
    TwentyFiveUf,
    #[serde(rename = "50UF")]
    FiftyUf,
}
impl fmt::Display for BillDataPaymentTerms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Pia => "PIA",
            Self::Cia => "CIA",
            Self::Ur => "UR",
            Self::Net10 => "NET10",
            Self::Net20 => "NET20",
            Self::Net30 => "NET30",
            Self::Net45 => "NET45",
            Self::Net60 => "NET60",
            Self::Net90 => "NET90",
            Self::Eom => "EOM",
            Self::Mfi => "MFI",
            Self::FiveMfi => "5MFI",
            Self::TenMfi => "10MFI",
            Self::FifteenMfi => "15MFI",
            Self::TwentyMfi => "20MFI",
            Self::Two10Net30 => "2/10NET30",
            Self::Uf => "UF",
            Self::TenUf => "10UF",
            Self::TwentyUf => "20UF",
            Self::TwentyFiveUf => "25UF",
            Self::FiftyUf => "50UF",
        };
        write!(f, "{}", s)
    }
}
