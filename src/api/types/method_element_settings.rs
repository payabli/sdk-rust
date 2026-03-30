pub use crate::prelude::*;

/// Settings for wallet payment methods.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MethodElementSettings {
    #[serde(rename = "applePay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<MethodElementSettingsApplePay>,
}

impl MethodElementSettings {
    pub fn builder() -> MethodElementSettingsBuilder {
        <MethodElementSettingsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MethodElementSettingsBuilder {
    apple_pay: Option<MethodElementSettingsApplePay>,
}

impl MethodElementSettingsBuilder {
    pub fn apple_pay(mut self, value: MethodElementSettingsApplePay) -> Self {
        self.apple_pay = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MethodElementSettings`].
    pub fn build(self) -> Result<MethodElementSettings, BuildError> {
        Ok(MethodElementSettings {
            apple_pay: self.apple_pay,
        })
    }
}
