pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CardSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance: Option<CardLinkTypes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fees: Option<BasicTable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<BasicTable>,
}

impl CardSection {
    pub fn builder() -> CardSectionBuilder {
        <CardSectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CardSectionBuilder {
    acceptance: Option<CardLinkTypes>,
    fees: Option<BasicTable>,
    price: Option<BasicTable>,
}

impl CardSectionBuilder {
    pub fn acceptance(mut self, value: CardLinkTypes) -> Self {
        self.acceptance = Some(value);
        self
    }

    pub fn fees(mut self, value: BasicTable) -> Self {
        self.fees = Some(value);
        self
    }

    pub fn price(mut self, value: BasicTable) -> Self {
        self.price = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CardSection`].
    pub fn build(self) -> Result<CardSection, BuildError> {
        Ok(CardSection {
            acceptance: self.acceptance,
            fees: self.fees,
            price: self.price,
        })
    }
}
