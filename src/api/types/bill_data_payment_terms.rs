pub use crate::prelude::*;

/// Payment terms for invoice. If no terms are defined, then response data for
/// this field defaults to `NET30`. Mirrors the values in
/// [`Terms`](#schema-terms).
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BillDataPaymentTerms {
    Pia,
    Cia,
    Ur,
    Net10,
    Net20,
    Net30,
    Net45,
    Net60,
    Net90,
    Eom,
    Mfi,
    FiveMfi,
    TenMfi,
    FifteenMfi,
    TwentyMfi,
    Two10Net30,
    Uf,
    TenUf,
    TwentyUf,
    TwentyFiveUf,
    FiftyUf,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for BillDataPaymentTerms {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Pia => serializer.serialize_str("PIA"),
            Self::Cia => serializer.serialize_str("CIA"),
            Self::Ur => serializer.serialize_str("UR"),
            Self::Net10 => serializer.serialize_str("NET10"),
            Self::Net20 => serializer.serialize_str("NET20"),
            Self::Net30 => serializer.serialize_str("NET30"),
            Self::Net45 => serializer.serialize_str("NET45"),
            Self::Net60 => serializer.serialize_str("NET60"),
            Self::Net90 => serializer.serialize_str("NET90"),
            Self::Eom => serializer.serialize_str("EOM"),
            Self::Mfi => serializer.serialize_str("MFI"),
            Self::FiveMfi => serializer.serialize_str("5MFI"),
            Self::TenMfi => serializer.serialize_str("10MFI"),
            Self::FifteenMfi => serializer.serialize_str("15MFI"),
            Self::TwentyMfi => serializer.serialize_str("20MFI"),
            Self::Two10Net30 => serializer.serialize_str("2/10NET30"),
            Self::Uf => serializer.serialize_str("UF"),
            Self::TenUf => serializer.serialize_str("10UF"),
            Self::TwentyUf => serializer.serialize_str("20UF"),
            Self::TwentyFiveUf => serializer.serialize_str("25UF"),
            Self::FiftyUf => serializer.serialize_str("50UF"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for BillDataPaymentTerms {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "PIA" => Ok(Self::Pia),
            "CIA" => Ok(Self::Cia),
            "UR" => Ok(Self::Ur),
            "NET10" => Ok(Self::Net10),
            "NET20" => Ok(Self::Net20),
            "NET30" => Ok(Self::Net30),
            "NET45" => Ok(Self::Net45),
            "NET60" => Ok(Self::Net60),
            "NET90" => Ok(Self::Net90),
            "EOM" => Ok(Self::Eom),
            "MFI" => Ok(Self::Mfi),
            "5MFI" => Ok(Self::FiveMfi),
            "10MFI" => Ok(Self::TenMfi),
            "15MFI" => Ok(Self::FifteenMfi),
            "20MFI" => Ok(Self::TwentyMfi),
            "2/10NET30" => Ok(Self::Two10Net30),
            "UF" => Ok(Self::Uf),
            "10UF" => Ok(Self::TenUf),
            "20UF" => Ok(Self::TwentyUf),
            "25UF" => Ok(Self::TwentyFiveUf),
            "50UF" => Ok(Self::FiftyUf),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for BillDataPaymentTerms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pia => write!(f, "PIA"),
            Self::Cia => write!(f, "CIA"),
            Self::Ur => write!(f, "UR"),
            Self::Net10 => write!(f, "NET10"),
            Self::Net20 => write!(f, "NET20"),
            Self::Net30 => write!(f, "NET30"),
            Self::Net45 => write!(f, "NET45"),
            Self::Net60 => write!(f, "NET60"),
            Self::Net90 => write!(f, "NET90"),
            Self::Eom => write!(f, "EOM"),
            Self::Mfi => write!(f, "MFI"),
            Self::FiveMfi => write!(f, "5MFI"),
            Self::TenMfi => write!(f, "10MFI"),
            Self::FifteenMfi => write!(f, "15MFI"),
            Self::TwentyMfi => write!(f, "20MFI"),
            Self::Two10Net30 => write!(f, "2/10NET30"),
            Self::Uf => write!(f, "UF"),
            Self::TenUf => write!(f, "10UF"),
            Self::TwentyUf => write!(f, "20UF"),
            Self::TwentyFiveUf => write!(f, "25UF"),
            Self::FiftyUf => write!(f, "50UF"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
