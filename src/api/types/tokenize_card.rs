pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TokenizeCard {
    /// The type of payment method to tokenize. For cards, this is always `card`.
    #[serde(default)]
    pub method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardcvv: Option<Cardcvv>,
    #[serde(default)]
    pub cardexp: Cardexp,
    #[serde(rename = "cardHolder")]
    #[serde(default)]
    pub card_holder: Cardholder,
    #[serde(default)]
    pub cardnumber: Cardnumber,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardzip: Option<Cardzip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
}

impl TokenizeCard {
    pub fn builder() -> TokenizeCardBuilder {
        <TokenizeCardBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TokenizeCardBuilder {
    method: Option<String>,
    cardcvv: Option<Cardcvv>,
    cardexp: Option<Cardexp>,
    card_holder: Option<Cardholder>,
    cardnumber: Option<Cardnumber>,
    cardzip: Option<Cardzip>,
    device: Option<Device>,
}

impl TokenizeCardBuilder {
    pub fn method(mut self, value: impl Into<String>) -> Self {
        self.method = Some(value.into());
        self
    }

    pub fn cardcvv(mut self, value: Cardcvv) -> Self {
        self.cardcvv = Some(value);
        self
    }

    pub fn cardexp(mut self, value: Cardexp) -> Self {
        self.cardexp = Some(value);
        self
    }

    pub fn card_holder(mut self, value: Cardholder) -> Self {
        self.card_holder = Some(value);
        self
    }

    pub fn cardnumber(mut self, value: Cardnumber) -> Self {
        self.cardnumber = Some(value);
        self
    }

    pub fn cardzip(mut self, value: Cardzip) -> Self {
        self.cardzip = Some(value);
        self
    }

    pub fn device(mut self, value: Device) -> Self {
        self.device = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TokenizeCard`].
    /// This method will fail if any of the following fields are not set:
    /// - [`method`](TokenizeCardBuilder::method)
    /// - [`cardexp`](TokenizeCardBuilder::cardexp)
    /// - [`card_holder`](TokenizeCardBuilder::card_holder)
    /// - [`cardnumber`](TokenizeCardBuilder::cardnumber)
    pub fn build(self) -> Result<TokenizeCard, BuildError> {
        Ok(TokenizeCard {
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
            cardcvv: self.cardcvv,
            cardexp: self
                .cardexp
                .ok_or_else(|| BuildError::missing_field("cardexp"))?,
            card_holder: self
                .card_holder
                .ok_or_else(|| BuildError::missing_field("card_holder"))?,
            cardnumber: self
                .cardnumber
                .ok_or_else(|| BuildError::missing_field("cardnumber"))?,
            cardzip: self.cardzip,
            device: self.device,
        })
    }
}
