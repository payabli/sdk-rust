pub use crate::prelude::*;

/// The Apple Pay button style. See
/// [Apple Pay Button Style](/developers/developer-guides/hosted-payment-page-apple-pay#param-applepay-button-style)
/// for more information.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MethodElementSettingsApplePayButtonStyle {
    Black,
    WhiteOutline,
    White,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MethodElementSettingsApplePayButtonStyle {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Black => serializer.serialize_str("black"),
            Self::WhiteOutline => serializer.serialize_str("white-outline"),
            Self::White => serializer.serialize_str("white"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MethodElementSettingsApplePayButtonStyle {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "black" => Ok(Self::Black),
            "white-outline" => Ok(Self::WhiteOutline),
            "white" => Ok(Self::White),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MethodElementSettingsApplePayButtonStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Black => write!(f, "black"),
            Self::WhiteOutline => write!(f, "white-outline"),
            Self::White => write!(f, "white"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
