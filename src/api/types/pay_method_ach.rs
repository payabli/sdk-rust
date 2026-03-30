pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PayMethodAch {
    /// Bank account number. This field is **required** when method = 'ach'.
    #[serde(rename = "achAccount")]
    #[serde(default)]
    pub ach_account: Achaccount,
    /// Bank account type. This field is **required** when method = 'ach'.
    #[serde(rename = "achAccountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_account_type: Option<Achaccounttype>,
    #[serde(rename = "achCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_code: Option<AchSecCode>,
    #[serde(rename = "achHolder")]
    #[serde(default)]
    pub ach_holder: AchHolder,
    #[serde(rename = "achHolderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_holder_type: Option<AchHolderType>,
    /// ABA/routing number of bank account. This field is **required** when method = 'ach'.
    #[serde(rename = "achRouting")]
    #[serde(default)]
    pub ach_routing: Achrouting,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
    pub method: String,
}

impl PayMethodAch {
    pub fn builder() -> PayMethodAchBuilder {
        <PayMethodAchBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayMethodAchBuilder {
    ach_account: Option<Achaccount>,
    ach_account_type: Option<Achaccounttype>,
    ach_code: Option<AchSecCode>,
    ach_holder: Option<AchHolder>,
    ach_holder_type: Option<AchHolderType>,
    ach_routing: Option<Achrouting>,
    device: Option<Device>,
    method: Option<String>,
}

impl PayMethodAchBuilder {
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

    pub fn ach_holder(mut self, value: AchHolder) -> Self {
        self.ach_holder = Some(value);
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

    pub fn device(mut self, value: Device) -> Self {
        self.device = Some(value);
        self
    }

    pub fn method(mut self, value: impl Into<String>) -> Self {
        self.method = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PayMethodAch`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ach_account`](PayMethodAchBuilder::ach_account)
    /// - [`ach_holder`](PayMethodAchBuilder::ach_holder)
    /// - [`ach_routing`](PayMethodAchBuilder::ach_routing)
    /// - [`method`](PayMethodAchBuilder::method)
    pub fn build(self) -> Result<PayMethodAch, BuildError> {
        Ok(PayMethodAch {
            ach_account: self
                .ach_account
                .ok_or_else(|| BuildError::missing_field("ach_account"))?,
            ach_account_type: self.ach_account_type,
            ach_code: self.ach_code,
            ach_holder: self
                .ach_holder
                .ok_or_else(|| BuildError::missing_field("ach_holder"))?,
            ach_holder_type: self.ach_holder_type,
            ach_routing: self
                .ach_routing
                .ok_or_else(|| BuildError::missing_field("ach_routing"))?,
            device: self.device,
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
        })
    }
}
