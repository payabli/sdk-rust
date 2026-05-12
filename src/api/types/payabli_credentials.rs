pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PayabliCredentials {
    /// The identifier for the payment connector, matching the `accountId` of the linked bank account.
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountId>,
    #[serde(rename = "cfeeFix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cfee_fix: Option<f64>,
    #[serde(rename = "cfeeFloat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cfee_float: Option<f64>,
    #[serde(rename = "cfeeMax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cfee_max: Option<f64>,
    #[serde(rename = "cfeeMin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cfee_min: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub maxticket: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub minticket: Option<f64>,
    /// The payment mode supported by this service. `0` for one-time payments, `1` for recurring payments, `2` for both.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i64>,
    #[serde(rename = "referenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<i64>,
    /// The payment service that this credential applies to. A paypoint can support multiple services, each represented by its own credential object in the array. Possible values are `card` (credit/debit card), `ach` (ACH bank transfer), `check` (paper check), `vcard` (virtual card), `cloud` (card-present), `cash`, `managed` (managed payment service), and `wallet`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(rename = "greaterValueAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greater_value_allowed: Option<GreaterValueAllowed>,
    #[serde(rename = "absorbDifference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absorb_difference: Option<AbsorbDifference>,
    #[serde(rename = "allowOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_override: Option<AllowOverride>,
}

impl PayabliCredentials {
    pub fn builder() -> PayabliCredentialsBuilder {
        <PayabliCredentialsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayabliCredentialsBuilder {
    account_id: Option<AccountId>,
    cfee_fix: Option<f64>,
    cfee_float: Option<f64>,
    cfee_max: Option<f64>,
    cfee_min: Option<f64>,
    maxticket: Option<f64>,
    minticket: Option<f64>,
    mode: Option<i64>,
    reference_id: Option<i64>,
    service: Option<String>,
    greater_value_allowed: Option<GreaterValueAllowed>,
    absorb_difference: Option<AbsorbDifference>,
    allow_override: Option<AllowOverride>,
}

impl PayabliCredentialsBuilder {
    pub fn account_id(mut self, value: AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn cfee_fix(mut self, value: f64) -> Self {
        self.cfee_fix = Some(value);
        self
    }

    pub fn cfee_float(mut self, value: f64) -> Self {
        self.cfee_float = Some(value);
        self
    }

    pub fn cfee_max(mut self, value: f64) -> Self {
        self.cfee_max = Some(value);
        self
    }

    pub fn cfee_min(mut self, value: f64) -> Self {
        self.cfee_min = Some(value);
        self
    }

    pub fn maxticket(mut self, value: f64) -> Self {
        self.maxticket = Some(value);
        self
    }

    pub fn minticket(mut self, value: f64) -> Self {
        self.minticket = Some(value);
        self
    }

    pub fn mode(mut self, value: i64) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn reference_id(mut self, value: i64) -> Self {
        self.reference_id = Some(value);
        self
    }

    pub fn service(mut self, value: impl Into<String>) -> Self {
        self.service = Some(value.into());
        self
    }

    pub fn greater_value_allowed(mut self, value: GreaterValueAllowed) -> Self {
        self.greater_value_allowed = Some(value);
        self
    }

    pub fn absorb_difference(mut self, value: AbsorbDifference) -> Self {
        self.absorb_difference = Some(value);
        self
    }

    pub fn allow_override(mut self, value: AllowOverride) -> Self {
        self.allow_override = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayabliCredentials`].
    pub fn build(self) -> Result<PayabliCredentials, BuildError> {
        Ok(PayabliCredentials {
            account_id: self.account_id,
            cfee_fix: self.cfee_fix,
            cfee_float: self.cfee_float,
            cfee_max: self.cfee_max,
            cfee_min: self.cfee_min,
            maxticket: self.maxticket,
            minticket: self.minticket,
            mode: self.mode,
            reference_id: self.reference_id,
            service: self.service,
            greater_value_allowed: self.greater_value_allowed,
            absorb_difference: self.absorb_difference,
            allow_override: self.allow_override,
        })
    }
}
