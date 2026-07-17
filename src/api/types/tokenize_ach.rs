pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct TokenizeAch {
    /// The type of payment method to tokenize. For ACH, this is always `ach`.
    #[serde(default)]
    pub method: String,
    #[serde(rename = "achAccount")]
    #[serde(default)]
    pub ach_account: Achaccount,
    #[serde(rename = "achAccountType")]
    pub ach_account_type: Achaccounttype,
    #[serde(rename = "achCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_code: Option<AchSecCode>,
    /// Bank account holder. This field is **required** when `method` is `ach`. Only letters, numbers, spaces, hyphens, apostrophes, and periods are allowed.
    #[serde(rename = "achHolder")]
    #[serde(default)]
    pub ach_holder: String,
    #[serde(rename = "achHolderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_holder_type: Option<AchHolderType>,
    #[serde(rename = "achRouting")]
    #[serde(default)]
    pub ach_routing: Achrouting,
}

impl TokenizeAch {
    pub fn builder() -> TokenizeAchBuilder {
        <TokenizeAchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TokenizeAchBuilder {
    method: Option<String>,
    ach_account: Option<Achaccount>,
    ach_account_type: Option<Achaccounttype>,
    ach_code: Option<AchSecCode>,
    ach_holder: Option<String>,
    ach_holder_type: Option<AchHolderType>,
    ach_routing: Option<Achrouting>,
}

impl TokenizeAchBuilder {
    pub fn method(mut self, value: impl Into<String>) -> Self {
        self.method = Some(value.into());
        self
    }

    pub fn ach_account(mut self, value: Achaccount) -> Self {
        self.ach_account = Some(value);
        self
    }

    pub fn ach_account_type(mut self, value: Achaccounttype) -> Self {
        self.ach_account_type = Some(value);
        self
    }

    pub fn ach_code(mut self, value: AchSecCode) -> Self {
        self.ach_code = Some(value);
        self
    }

    pub fn ach_holder(mut self, value: impl Into<String>) -> Self {
        self.ach_holder = Some(value.into());
        self
    }

    pub fn ach_holder_type(mut self, value: AchHolderType) -> Self {
        self.ach_holder_type = Some(value);
        self
    }

    pub fn ach_routing(mut self, value: Achrouting) -> Self {
        self.ach_routing = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TokenizeAch`].
    /// This method will fail if any of the following fields are not set:
    /// - [`method`](TokenizeAchBuilder::method)
    /// - [`ach_account`](TokenizeAchBuilder::ach_account)
    /// - [`ach_account_type`](TokenizeAchBuilder::ach_account_type)
    /// - [`ach_holder`](TokenizeAchBuilder::ach_holder)
    /// - [`ach_routing`](TokenizeAchBuilder::ach_routing)
    pub fn build(self) -> Result<TokenizeAch, BuildError> {
        Ok(TokenizeAch {
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
            ach_account: self
                .ach_account
                .ok_or_else(|| BuildError::missing_field("ach_account"))?,
            ach_account_type: self
                .ach_account_type
                .ok_or_else(|| BuildError::missing_field("ach_account_type"))?,
            ach_code: self.ach_code,
            ach_holder: self
                .ach_holder
                .ok_or_else(|| BuildError::missing_field("ach_holder"))?,
            ach_holder_type: self.ach_holder_type,
            ach_routing: self
                .ach_routing
                .ok_or_else(|| BuildError::missing_field("ach_routing"))?,
        })
    }
}
