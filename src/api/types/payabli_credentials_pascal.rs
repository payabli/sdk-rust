pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PayabliCredentialsPascal {
    /// The payment service that this credential applies to. A paypoint can support multiple services, each represented by its own credential object in the array. Possible values are `card` (credit/debit card), `ach` (ACH bank transfer), `check` (paper check), `vcard` (virtual card), `cloud` (card-present), `cash`, `managed` (managed payment service), and `wallet`.
    #[serde(rename = "Service")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    /// The payment mode supported by this service. `0` for one-time payments, `1` for recurring payments, `2` for both.
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i64>,
    #[serde(rename = "MinTicket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_ticket: Option<MinTicket>,
    #[serde(rename = "MaxTicket")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_ticket: Option<MaxTicket>,
    #[serde(rename = "CfeeFix")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cfee_fix: Option<f64>,
    #[serde(rename = "CfeeFloat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cfee_float: Option<f64>,
    #[serde(rename = "CfeeMin")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cfee_min: Option<f64>,
    #[serde(rename = "CfeeMax")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers::option")]
    pub cfee_max: Option<f64>,
    /// The identifier for the payment connector, matching the `accountId` of the linked bank account.
    #[serde(rename = "AccountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountId>,
    #[serde(rename = "ReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<i64>,
    #[serde(rename = "acceptSameDayACH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_same_day_ach: Option<bool>,
    /// The default currency for the paypoint, either `USD` or `CAD`.
    #[serde(rename = "Currency")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<Vec<String>>,
    #[serde(rename = "GreaterValueAllowed")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greater_value_allowed: Option<GreaterValueAllowed>,
    #[serde(rename = "AbsorbDifference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub absorb_difference: Option<AbsorbDifference>,
    #[serde(rename = "AllowOverride")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_override: Option<AllowOverride>,
}

impl PayabliCredentialsPascal {
    pub fn builder() -> PayabliCredentialsPascalBuilder {
        <PayabliCredentialsPascalBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayabliCredentialsPascalBuilder {
    service: Option<String>,
    mode: Option<i64>,
    min_ticket: Option<MinTicket>,
    max_ticket: Option<MaxTicket>,
    cfee_fix: Option<f64>,
    cfee_float: Option<f64>,
    cfee_min: Option<f64>,
    cfee_max: Option<f64>,
    account_id: Option<AccountId>,
    reference_id: Option<i64>,
    accept_same_day_ach: Option<bool>,
    currency: Option<Vec<String>>,
    greater_value_allowed: Option<GreaterValueAllowed>,
    absorb_difference: Option<AbsorbDifference>,
    allow_override: Option<AllowOverride>,
}

impl PayabliCredentialsPascalBuilder {
    pub fn service(mut self, value: impl Into<String>) -> Self {
        self.service = Some(value.into());
        self
    }

    pub fn mode(mut self, value: i64) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn min_ticket(mut self, value: MinTicket) -> Self {
        self.min_ticket = Some(value);
        self
    }

    pub fn max_ticket(mut self, value: MaxTicket) -> Self {
        self.max_ticket = Some(value);
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

    pub fn cfee_min(mut self, value: f64) -> Self {
        self.cfee_min = Some(value);
        self
    }

    pub fn cfee_max(mut self, value: f64) -> Self {
        self.cfee_max = Some(value);
        self
    }

    pub fn account_id(mut self, value: AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn reference_id(mut self, value: i64) -> Self {
        self.reference_id = Some(value);
        self
    }

    pub fn accept_same_day_ach(mut self, value: bool) -> Self {
        self.accept_same_day_ach = Some(value);
        self
    }

    pub fn currency(mut self, value: Vec<String>) -> Self {
        self.currency = Some(value);
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

    /// Consumes the builder and constructs a [`PayabliCredentialsPascal`].
    pub fn build(self) -> Result<PayabliCredentialsPascal, BuildError> {
        Ok(PayabliCredentialsPascal {
            service: self.service,
            mode: self.mode,
            min_ticket: self.min_ticket,
            max_ticket: self.max_ticket,
            cfee_fix: self.cfee_fix,
            cfee_float: self.cfee_float,
            cfee_min: self.cfee_min,
            cfee_max: self.cfee_max,
            account_id: self.account_id,
            reference_id: self.reference_id,
            accept_same_day_ach: self.accept_same_day_ach,
            currency: self.currency,
            greater_value_allowed: self.greater_value_allowed,
            absorb_difference: self.absorb_difference,
            allow_override: self.allow_override,
        })
    }
}
