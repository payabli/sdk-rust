pub use crate::prelude::*;

/// The text on Apple Pay button. See [Apple Pay Button Type](/developers/developer-guides/hosted-payment-page-apple-pay#param-applepay-button-type) for more information.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum MethodElementSettingsApplePayButtonType {
    #[serde(rename = "plain")]
    Plain,
    #[serde(rename = "buy")]
    Buy,
    #[serde(rename = "donate")]
    Donate,
    #[serde(rename = "check-out")]
    CheckOut,
    #[serde(rename = "book")]
    Book,
    #[serde(rename = "continue")]
    Continue,
    #[serde(rename = "top-up")]
    TopUp,
    #[serde(rename = "order")]
    Order,
    #[serde(rename = "rent")]
    Rent,
    #[serde(rename = "support")]
    Support,
    #[serde(rename = "contribute")]
    Contribute,
    #[serde(rename = "tip")]
    Tip,
    #[serde(rename = "pay")]
    Pay,
}
impl fmt::Display for MethodElementSettingsApplePayButtonType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Plain => "plain",
            Self::Buy => "buy",
            Self::Donate => "donate",
            Self::CheckOut => "check-out",
            Self::Book => "book",
            Self::Continue => "continue",
            Self::TopUp => "top-up",
            Self::Order => "order",
            Self::Rent => "rent",
            Self::Support => "support",
            Self::Contribute => "contribute",
            Self::Tip => "tip",
            Self::Pay => "pay",
        };
        write!(f, "{}", s)
    }
}
