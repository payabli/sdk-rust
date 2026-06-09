pub use crate::prelude::*;

/// Payment method object for vendor payouts.
/// - `{ method: "managed" }` - Managed payment method
/// - `{ method: "vcard" }` - Virtual card payment method
/// - `{ method: "check" }` - Check payment method
/// - `{ method: "ach", achHolder: "...", achRouting: "...", achAccount: "...", achAccountType: "..." }` - ACH payment method with bank details
/// - `{ method: "ach", storedMethodId: "..." }` - ACH payment method using stored method ID
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AuthorizePaymentMethod {
    /// Payment method type - "managed", "vcard", "check", or "ach"
    #[serde(default)]
    pub method: String,
    /// Account holder name for ACH payments. Required when method is "ach" and not using `storedMethodId`.
    #[serde(rename = "achHolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_holder: Option<String>,
    /// Bank routing number for ACH payments. Required when method is "ach" and not using `storedMethodId`.
    #[serde(rename = "achRouting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_routing: Option<String>,
    /// Bank account number for ACH payments. Required when method is "ach" and not using `storedMethodId`.
    #[serde(rename = "achAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_account: Option<String>,
    /// Account type for ACH payments ("checking" or "savings"). Required when method is "ach" and not using `storedMethodId`.
    #[serde(rename = "achAccountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_account_type: Option<String>,
    #[serde(rename = "achCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_code: Option<AchSecCode>,
    #[serde(rename = "achHolderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_holder_type: Option<AchHolderType>,
    /// ID of the stored ACH payment method. Only applicable when method is `ach`. Use this to reference a previously saved ACH method instead of providing bank details directly.
    #[serde(rename = "storedMethodId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_method_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<Initiator>,
    #[serde(rename = "storedMethodUsageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_method_usage_type: Option<StoredMethodUsageType>,
}

impl AuthorizePaymentMethod {
    pub fn builder() -> AuthorizePaymentMethodBuilder {
        <AuthorizePaymentMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AuthorizePaymentMethodBuilder {
    method: Option<String>,
    ach_holder: Option<String>,
    ach_routing: Option<String>,
    ach_account: Option<String>,
    ach_account_type: Option<String>,
    ach_code: Option<AchSecCode>,
    ach_holder_type: Option<AchHolderType>,
    stored_method_id: Option<String>,
    initiator: Option<Initiator>,
    stored_method_usage_type: Option<StoredMethodUsageType>,
}

impl AuthorizePaymentMethodBuilder {
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

    pub fn ach_code(mut self, value: AchSecCode) -> Self {
        self.ach_code = Some(value);
        self
    }

    pub fn ach_holder_type(mut self, value: AchHolderType) -> Self {
        self.ach_holder_type = Some(value);
        self
    }

    pub fn stored_method_id(mut self, value: impl Into<String>) -> Self {
        self.stored_method_id = Some(value.into());
        self
    }

    pub fn initiator(mut self, value: Initiator) -> Self {
        self.initiator = Some(value);
        self
    }

    pub fn stored_method_usage_type(mut self, value: StoredMethodUsageType) -> Self {
        self.stored_method_usage_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AuthorizePaymentMethod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`method`](AuthorizePaymentMethodBuilder::method)
    pub fn build(self) -> Result<AuthorizePaymentMethod, BuildError> {
        Ok(AuthorizePaymentMethod {
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
            ach_holder: self.ach_holder,
            ach_routing: self.ach_routing,
            ach_account: self.ach_account,
            ach_account_type: self.ach_account_type,
            ach_code: self.ach_code,
            ach_holder_type: self.ach_holder_type,
            stored_method_id: self.stored_method_id,
            initiator: self.initiator,
            stored_method_usage_type: self.stored_method_usage_type,
        })
    }
}
