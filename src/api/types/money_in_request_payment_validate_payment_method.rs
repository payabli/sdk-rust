pub use crate::prelude::*;

/// Object describing payment method to use for validation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RequestPaymentValidatePaymentMethod {
    pub method: RequestPaymentValidatePaymentMethodMethod,
    #[serde(default)]
    pub cardnumber: Cardnumber,
    #[serde(default)]
    pub cardexp: Cardexp,
    #[serde(default)]
    pub cardzip: Cardzip,
    #[serde(rename = "cardHolder")]
    #[serde(default)]
    pub card_holder: Cardholder,
}

impl RequestPaymentValidatePaymentMethod {
    pub fn builder() -> RequestPaymentValidatePaymentMethodBuilder {
        <RequestPaymentValidatePaymentMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequestPaymentValidatePaymentMethodBuilder {
    method: Option<RequestPaymentValidatePaymentMethodMethod>,
    cardnumber: Option<Cardnumber>,
    cardexp: Option<Cardexp>,
    cardzip: Option<Cardzip>,
    card_holder: Option<Cardholder>,
}

impl RequestPaymentValidatePaymentMethodBuilder {
    pub fn method(mut self, value: RequestPaymentValidatePaymentMethodMethod) -> Self {
        self.method = Some(value);
        self
    }

    pub fn cardnumber(mut self, value: Cardnumber) -> Self {
        self.cardnumber = Some(value);
        self
    }

    pub fn cardexp(mut self, value: Cardexp) -> Self {
        self.cardexp = Some(value);
        self
    }

    pub fn cardzip(mut self, value: Cardzip) -> Self {
        self.cardzip = Some(value);
        self
    }

    pub fn card_holder(mut self, value: Cardholder) -> Self {
        self.card_holder = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RequestPaymentValidatePaymentMethod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`method`](RequestPaymentValidatePaymentMethodBuilder::method)
    /// - [`cardnumber`](RequestPaymentValidatePaymentMethodBuilder::cardnumber)
    /// - [`cardexp`](RequestPaymentValidatePaymentMethodBuilder::cardexp)
    /// - [`cardzip`](RequestPaymentValidatePaymentMethodBuilder::cardzip)
    /// - [`card_holder`](RequestPaymentValidatePaymentMethodBuilder::card_holder)
    pub fn build(self) -> Result<RequestPaymentValidatePaymentMethod, BuildError> {
        Ok(RequestPaymentValidatePaymentMethod {
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
            cardnumber: self
                .cardnumber
                .ok_or_else(|| BuildError::missing_field("cardnumber"))?,
            cardexp: self
                .cardexp
                .ok_or_else(|| BuildError::missing_field("cardexp"))?,
            cardzip: self
                .cardzip
                .ok_or_else(|| BuildError::missing_field("cardzip"))?,
            card_holder: self
                .card_holder
                .ok_or_else(|| BuildError::missing_field("card_holder"))?,
        })
    }
}
