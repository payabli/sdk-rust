pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AchSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance: Option<AchLinkTypes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fees: Option<BasicTable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<BasicTable>,
}

impl AchSection {
    pub fn builder() -> AchSectionBuilder {
        <AchSectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AchSectionBuilder {
    acceptance: Option<AchLinkTypes>,
    fees: Option<BasicTable>,
    price: Option<BasicTable>,
}

impl AchSectionBuilder {
    pub fn acceptance(mut self, value: AchLinkTypes) -> Self {
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

    /// Consumes the builder and constructs a [`AchSection`].
    pub fn build(self) -> Result<AchSection, BuildError> {
        Ok(AchSection {
            acceptance: self.acceptance,
            fees: self.fees,
            price: self.price,
        })
    }
}
