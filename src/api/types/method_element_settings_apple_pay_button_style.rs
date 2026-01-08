pub use crate::prelude::*;

/// The Apple Pay button style. See [Apple Pay Button Style](/developers/developer-guides/hosted-payment-page-apple-pay#param-applepay-button-style) for more information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MethodElementSettingsApplePayButtonStyle {
    #[serde(rename = "black")]
    Black,
    #[serde(rename = "white-outline")]
    WhiteOutline,
    #[serde(rename = "white")]
    White,
}
impl fmt::Display for MethodElementSettingsApplePayButtonStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Black => "black",
            Self::WhiteOutline => "white-outline",
            Self::White => "white",
        };
        write!(f, "{}", s)
    }
}
