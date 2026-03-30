pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Instrument {
    #[serde(rename = "achAccount")]
    #[serde(default)]
    pub ach_account: Achaccount,
    #[serde(rename = "achRouting")]
    #[serde(default)]
    pub ach_routing: Achrouting,
    #[serde(rename = "billingAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<BillingAddressNullable>,
    #[serde(rename = "billingCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_city: Option<BillingCityNullable>,
    #[serde(rename = "billingCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_country: Option<BillingCountryNullable>,
    #[serde(rename = "billingState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_state: Option<BillingStateNullable>,
    #[serde(rename = "billingZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_zip: Option<BillingZip>,
}

impl Instrument {
    pub fn builder() -> InstrumentBuilder {
        <InstrumentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InstrumentBuilder {
    ach_account: Option<Achaccount>,
    ach_routing: Option<Achrouting>,
    billing_address: Option<BillingAddressNullable>,
    billing_city: Option<BillingCityNullable>,
    billing_country: Option<BillingCountryNullable>,
    billing_state: Option<BillingStateNullable>,
    billing_zip: Option<BillingZip>,
}

impl InstrumentBuilder {
    pub fn ach_account(mut self, value: Achaccount) -> Self {
        self.ach_account = Some(value);
        self
    }

    pub fn ach_routing(mut self, value: Achrouting) -> Self {
        self.ach_routing = Some(value);
        self
    }

    pub fn billing_address(mut self, value: BillingAddressNullable) -> Self {
        self.billing_address = Some(value);
        self
    }

    pub fn billing_city(mut self, value: BillingCityNullable) -> Self {
        self.billing_city = Some(value);
        self
    }

    pub fn billing_country(mut self, value: BillingCountryNullable) -> Self {
        self.billing_country = Some(value);
        self
    }

    pub fn billing_state(mut self, value: BillingStateNullable) -> Self {
        self.billing_state = Some(value);
        self
    }

    pub fn billing_zip(mut self, value: BillingZip) -> Self {
        self.billing_zip = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`Instrument`].
    /// This method will fail if any of the following fields are not set:
    /// - [`ach_account`](InstrumentBuilder::ach_account)
    /// - [`ach_routing`](InstrumentBuilder::ach_routing)
    pub fn build(self) -> Result<Instrument, BuildError> {
        Ok(Instrument {
            ach_account: self
                .ach_account
                .ok_or_else(|| BuildError::missing_field("ach_account"))?,
            ach_routing: self
                .ach_routing
                .ok_or_else(|| BuildError::missing_field("ach_routing"))?,
            billing_address: self.billing_address,
            billing_city: self.billing_city,
            billing_country: self.billing_country,
            billing_state: self.billing_state,
            billing_zip: self.billing_zip,
        })
    }
}
