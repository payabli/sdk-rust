pub use crate::prelude::*;

/// Object describing the ACH payment method to use for transaction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RequestCreditPaymentMethod {
    #[serde(rename = "achAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_account: Option<Achaccount>,
    #[serde(rename = "achAccountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_account_type: Option<Achaccounttype>,
    #[serde(rename = "achCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_code: Option<AchSecCode>,
    /// Bank account holder.
    #[serde(rename = "achHolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_holder: Option<AchHolder>,
    #[serde(rename = "achRouting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_routing: Option<Achrouting>,
    /// Method to use for the transaction. Must be ACH.
    pub method: String,
}

impl RequestCreditPaymentMethod {
    pub fn builder() -> RequestCreditPaymentMethodBuilder {
        <RequestCreditPaymentMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequestCreditPaymentMethodBuilder {
    ach_account: Option<Achaccount>,
    ach_account_type: Option<Achaccounttype>,
    ach_code: Option<AchSecCode>,
    ach_holder: Option<AchHolder>,
    ach_routing: Option<Achrouting>,
    method: Option<String>,
}

impl RequestCreditPaymentMethodBuilder {
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

    pub fn ach_routing(mut self, value: Achrouting) -> Self {
        self.ach_routing = Some(value);
        self
    }

    pub fn method(mut self, value: impl Into<String>) -> Self {
        self.method = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RequestCreditPaymentMethod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`method`](RequestCreditPaymentMethodBuilder::method)
    pub fn build(self) -> Result<RequestCreditPaymentMethod, BuildError> {
        Ok(RequestCreditPaymentMethod {
            ach_account: self.ach_account,
            ach_account_type: self.ach_account_type,
            ach_code: self.ach_code,
            ach_holder: self.ach_holder,
            ach_routing: self.ach_routing,
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
        })
    }
}
