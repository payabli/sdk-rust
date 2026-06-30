pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MethodsList {
    /// When `true`, American Express is accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amex: Option<bool>,
    /// When `true`, Apple Pay is accepted.
    #[serde(rename = "applePay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<bool>,
    /// When `true`, Google Pay is accepted.
    #[serde(rename = "googlePay")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<bool>,
    /// When `true`, Discover is accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discover: Option<bool>,
    /// When `true`, ACH is accepted.
    #[serde(rename = "eCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_check: Option<bool>,
    /// When `true`, Mastercard is accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mastercard: Option<bool>,
    /// When `true`, Visa is accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa: Option<bool>,
    /// When `true`, Diners Club is accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub diners: Option<bool>,
    /// When `true`, JCB is accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jcb: Option<bool>,
    /// When `true`, Remote Deposit Capture (RDC) is accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rdc: Option<bool>,
}

impl MethodsList {
    pub fn builder() -> MethodsListBuilder {
        <MethodsListBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MethodsListBuilder {
    amex: Option<bool>,
    apple_pay: Option<bool>,
    google_pay: Option<bool>,
    discover: Option<bool>,
    e_check: Option<bool>,
    mastercard: Option<bool>,
    visa: Option<bool>,
    diners: Option<bool>,
    jcb: Option<bool>,
    rdc: Option<bool>,
}

impl MethodsListBuilder {
    pub fn amex(mut self, value: bool) -> Self {
        self.amex = Some(value);
        self
    }

    pub fn apple_pay(mut self, value: bool) -> Self {
        self.apple_pay = Some(value);
        self
    }

    pub fn google_pay(mut self, value: bool) -> Self {
        self.google_pay = Some(value);
        self
    }

    pub fn discover(mut self, value: bool) -> Self {
        self.discover = Some(value);
        self
    }

    pub fn e_check(mut self, value: bool) -> Self {
        self.e_check = Some(value);
        self
    }

    pub fn mastercard(mut self, value: bool) -> Self {
        self.mastercard = Some(value);
        self
    }

    pub fn visa(mut self, value: bool) -> Self {
        self.visa = Some(value);
        self
    }

    pub fn diners(mut self, value: bool) -> Self {
        self.diners = Some(value);
        self
    }

    pub fn jcb(mut self, value: bool) -> Self {
        self.jcb = Some(value);
        self
    }

    pub fn rdc(mut self, value: bool) -> Self {
        self.rdc = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MethodsList`].
    pub fn build(self) -> Result<MethodsList, BuildError> {
        Ok(MethodsList {
            amex: self.amex,
            apple_pay: self.apple_pay,
            google_pay: self.google_pay,
            discover: self.discover,
            e_check: self.e_check,
            mastercard: self.mastercard,
            visa: self.visa,
            diners: self.diners,
            jcb: self.jcb,
            rdc: self.rdc,
        })
    }
}
