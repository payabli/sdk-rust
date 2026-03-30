pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
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

impl MethodElementSettingsApplePay {
    pub fn builder() -> MethodElementSettingsApplePayBuilder {
        <MethodElementSettingsApplePayBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MethodElementSettingsApplePayBuilder {
    button_style: Option<MethodElementSettingsApplePayButtonStyle>,
    button_type: Option<MethodElementSettingsApplePayButtonType>,
    language: Option<MethodElementSettingsApplePayLanguage>,
}

impl MethodElementSettingsApplePayBuilder {
    pub fn button_style(mut self, value: MethodElementSettingsApplePayButtonStyle) -> Self {
        self.button_style = Some(value);
        self
    }

    pub fn button_type(mut self, value: MethodElementSettingsApplePayButtonType) -> Self {
        self.button_type = Some(value);
        self
    }

    pub fn language(mut self, value: MethodElementSettingsApplePayLanguage) -> Self {
        self.language = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MethodElementSettingsApplePay`].
    pub fn build(self) -> Result<MethodElementSettingsApplePay, BuildError> {
        Ok(MethodElementSettingsApplePay {
            button_style: self.button_style,
            button_type: self.button_type,
            language: self.language,
        })
    }
}
