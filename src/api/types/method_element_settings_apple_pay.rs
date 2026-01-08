pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct MethodElementSettingsApplePay {
    /// The Apple Pay button style. See [Apple Pay Button Style](/developers/developer-guides/hosted-payment-page-apple-pay#param-applepay-button-style) for more information.
    #[serde(rename = "buttonStyle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_style: Option<MethodElementSettingsApplePayButtonStyle>,
    /// The text on Apple Pay button. See [Apple Pay Button Type](/developers/developer-guides/hosted-payment-page-apple-pay#param-applepay-button-type) for more information.
    #[serde(rename = "buttonType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub button_type: Option<MethodElementSettingsApplePayButtonType>,
    /// The Apple Pay button locale. See [Apple Pay Button Language](/developers/developer-guides/hosted-payment-page-apple-pay#param-applepay-language) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<MethodElementSettingsApplePayLanguage>,
}
