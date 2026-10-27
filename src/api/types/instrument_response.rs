pub use crate::prelude::*;

/// The organization's billing information. Fields that aren't set return `null`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct InstrumentResponse {
    /// Bank account number, masked.
    #[serde(rename = "achAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_account: Option<String>,
    /// ABA/routing number of bank account.
    #[serde(rename = "achRouting")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_routing: Option<String>,
    /// Billing address.
    #[serde(rename = "billingAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<String>,
    /// Billing city.
    #[serde(rename = "billingCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_city: Option<String>,
    /// Billing address country.
    #[serde(rename = "billingCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_country: Option<String>,
    /// Billing state.
    #[serde(rename = "billingState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_state: Option<String>,
    /// Billing address ZIP code.
    #[serde(rename = "billingZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_zip: Option<String>,
}

impl InstrumentResponse {
    pub fn builder() -> InstrumentResponseBuilder {
        <InstrumentResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InstrumentResponseBuilder {
    ach_account: Option<String>,
    ach_routing: Option<String>,
    billing_address: Option<String>,
    billing_city: Option<String>,
    billing_country: Option<String>,
    billing_state: Option<String>,
    billing_zip: Option<String>,
}

impl InstrumentResponseBuilder {
    pub fn ach_account(mut self, value: impl Into<String>) -> Self {
        self.ach_account = Some(value.into());
        self
    }

    pub fn ach_routing(mut self, value: impl Into<String>) -> Self {
        self.ach_routing = Some(value.into());
        self
    }

    pub fn billing_address(mut self, value: impl Into<String>) -> Self {
        self.billing_address = Some(value.into());
        self
    }

    pub fn billing_city(mut self, value: impl Into<String>) -> Self {
        self.billing_city = Some(value.into());
        self
    }

    pub fn billing_country(mut self, value: impl Into<String>) -> Self {
        self.billing_country = Some(value.into());
        self
    }

    pub fn billing_state(mut self, value: impl Into<String>) -> Self {
        self.billing_state = Some(value.into());
        self
    }

    pub fn billing_zip(mut self, value: impl Into<String>) -> Self {
        self.billing_zip = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`InstrumentResponse`].
    pub fn build(self) -> Result<InstrumentResponse, BuildError> {
        Ok(InstrumentResponse {
            ach_account: self.ach_account,
            ach_routing: self.ach_routing,
            billing_address: self.billing_address,
            billing_city: self.billing_city,
            billing_country: self.billing_country,
            billing_state: self.billing_state,
            billing_zip: self.billing_zip,
        })
    }
}
