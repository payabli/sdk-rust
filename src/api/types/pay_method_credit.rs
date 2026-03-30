pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PayMethodCredit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardcvv: Option<Cardcvv>,
    #[serde(default)]
    pub cardexp: Cardexp,
    #[serde(rename = "cardHolder")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_holder: Option<Cardholder>,
    #[serde(default)]
    pub cardnumber: Cardnumber,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardzip: Option<Cardzip>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<Initiator>,
    /// Method to use for the transaction. For transactions with a credit or debit card, or a tokenized card, use `card`.
    pub method: String,
    #[serde(rename = "saveIfSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_if_success: Option<SaveIfSuccess>,
}

impl PayMethodCredit {
    pub fn builder() -> PayMethodCreditBuilder {
        <PayMethodCreditBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayMethodCreditBuilder {
    cardcvv: Option<Cardcvv>,
    cardexp: Option<Cardexp>,
    card_holder: Option<Cardholder>,
    cardnumber: Option<Cardnumber>,
    cardzip: Option<Cardzip>,
    initiator: Option<Initiator>,
    method: Option<String>,
    save_if_success: Option<SaveIfSuccess>,
}

impl PayMethodCreditBuilder {
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

    pub fn initiator(mut self, value: Initiator) -> Self {
        self.initiator = Some(value);
        self
    }

    pub fn method(mut self, value: impl Into<String>) -> Self {
        self.method = Some(value.into());
        self
    }

    pub fn save_if_success(mut self, value: SaveIfSuccess) -> Self {
        self.save_if_success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayMethodCredit`].
    /// This method will fail if any of the following fields are not set:
    /// - [`cardexp`](PayMethodCreditBuilder::cardexp)
    /// - [`cardnumber`](PayMethodCreditBuilder::cardnumber)
    /// - [`method`](PayMethodCreditBuilder::method)
    pub fn build(self) -> Result<PayMethodCredit, BuildError> {
        Ok(PayMethodCredit {
            cardcvv: self.cardcvv,
            cardexp: self
                .cardexp
                .ok_or_else(|| BuildError::missing_field("cardexp"))?,
            card_holder: self.card_holder,
            cardnumber: self
                .cardnumber
                .ok_or_else(|| BuildError::missing_field("cardnumber"))?,
            cardzip: self.cardzip,
            initiator: self.initiator,
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
            save_if_success: self.save_if_success,
        })
    }
}
