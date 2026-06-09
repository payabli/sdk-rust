pub use crate::prelude::*;

/// The text on Apple Pay button. See
/// [Apple Pay Button Type](/developers/developer-guides/hosted-payment-page-apple-pay#param-applepay-button-type)
/// for more information.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MethodElementSettingsApplePayButtonType {
    Plain,
    Buy,
    Donate,
    CheckOut,
    Book,
    Continue,
    TopUp,
    Order,
    Rent,
    Support,
    Contribute,
    Tip,
    Pay,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for MethodElementSettingsApplePayButtonType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Plain => serializer.serialize_str("plain"),
            Self::Buy => serializer.serialize_str("buy"),
            Self::Donate => serializer.serialize_str("donate"),
            Self::CheckOut => serializer.serialize_str("check-out"),
            Self::Book => serializer.serialize_str("book"),
            Self::Continue => serializer.serialize_str("continue"),
            Self::TopUp => serializer.serialize_str("top-up"),
            Self::Order => serializer.serialize_str("order"),
            Self::Rent => serializer.serialize_str("rent"),
            Self::Support => serializer.serialize_str("support"),
            Self::Contribute => serializer.serialize_str("contribute"),
            Self::Tip => serializer.serialize_str("tip"),
            Self::Pay => serializer.serialize_str("pay"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for MethodElementSettingsApplePayButtonType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "plain" => Ok(Self::Plain),
            "buy" => Ok(Self::Buy),
            "donate" => Ok(Self::Donate),
            "check-out" => Ok(Self::CheckOut),
            "book" => Ok(Self::Book),
            "continue" => Ok(Self::Continue),
            "top-up" => Ok(Self::TopUp),
            "order" => Ok(Self::Order),
            "rent" => Ok(Self::Rent),
            "support" => Ok(Self::Support),
            "contribute" => Ok(Self::Contribute),
            "tip" => Ok(Self::Tip),
            "pay" => Ok(Self::Pay),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for MethodElementSettingsApplePayButtonType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Plain => write!(f, "plain"),
            Self::Buy => write!(f, "buy"),
            Self::Donate => write!(f, "donate"),
            Self::CheckOut => write!(f, "check-out"),
            Self::Book => write!(f, "book"),
            Self::Continue => write!(f, "continue"),
            Self::TopUp => write!(f, "top-up"),
            Self::Order => write!(f, "order"),
            Self::Rent => write!(f, "rent"),
            Self::Support => write!(f, "support"),
            Self::Contribute => write!(f, "contribute"),
            Self::Tip => write!(f, "tip"),
            Self::Pay => write!(f, "pay"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
