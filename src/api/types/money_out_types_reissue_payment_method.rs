pub use crate::prelude::*;

/// Payment method for reissuing a payout transaction. The reissue endpoint uses the payment method details directly. It doesn't fall back to the vendor's managed payment method.
/// - `{ method: "vcard" }` - Reissue as a virtual card
/// - `{ method: "check" }` - Reissue as a paper check
/// - `{ method: "ach", achHolder: "...", achRouting: "...", achAccount: "...", achAccountType: "...", achHolderType: "..." }` - Reissue as ACH with bank details
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ReissuePaymentMethod {
    /// Payment method type. Must be `"ach"`, `"check"`, or `"vcard"`.
    #[serde(default)]
    pub method: String,
    /// Account holder name. Required when `method` is `"ach"`.
    #[serde(rename = "achHolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_holder: Option<String>,
    /// Bank routing number (9 digits). Required when `method` is `"ach"`.
    #[serde(rename = "achRouting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_routing: Option<String>,
    /// Bank account number (8-17 digits). Required when `method` is `"ach"`.
    #[serde(rename = "achAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_account: Option<String>,
    /// Bank account type (`"checking"` or `"savings"`). Required when `method` is `"ach"`.
    #[serde(rename = "achAccountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_account_type: Option<String>,
    #[serde(rename = "achHolderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_holder_type: Option<AchHolderType>,
}

impl ReissuePaymentMethod {
    pub fn builder() -> ReissuePaymentMethodBuilder {
        <ReissuePaymentMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ReissuePaymentMethodBuilder {
    method: Option<String>,
    ach_holder: Option<String>,
    ach_routing: Option<String>,
    ach_account: Option<String>,
    ach_account_type: Option<String>,
    ach_holder_type: Option<AchHolderType>,
}

impl ReissuePaymentMethodBuilder {
    pub fn method(mut self, value: impl Into<String>) -> Self {
        self.method = Some(value.into());
        self
    }

    pub fn ach_holder(mut self, value: impl Into<String>) -> Self {
        self.ach_holder = Some(value.into());
        self
    }

    pub fn ach_routing(mut self, value: impl Into<String>) -> Self {
        self.ach_routing = Some(value.into());
        self
    }

    pub fn ach_account(mut self, value: impl Into<String>) -> Self {
        self.ach_account = Some(value.into());
        self
    }

    pub fn ach_account_type(mut self, value: impl Into<String>) -> Self {
        self.ach_account_type = Some(value.into());
        self
    }

    pub fn ach_holder_type(mut self, value: AchHolderType) -> Self {
        self.ach_holder_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ReissuePaymentMethod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`method`](ReissuePaymentMethodBuilder::method)
    pub fn build(self) -> Result<ReissuePaymentMethod, BuildError> {
        Ok(ReissuePaymentMethod {
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
            ach_holder: self.ach_holder,
            ach_routing: self.ach_routing,
            ach_account: self.ach_account,
            ach_account_type: self.ach_account_type,
            ach_holder_type: self.ach_holder_type,
        })
    }
}
