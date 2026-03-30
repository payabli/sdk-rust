pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CardIcpSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<CardType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Visible>,
}

impl CardIcpSection {
    pub fn builder() -> CardIcpSectionBuilder {
        <CardIcpSectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CardIcpSectionBuilder {
    tiers: Option<Vec<CardType>>,
    visible: Option<Visible>,
}

impl CardIcpSectionBuilder {
    pub fn tiers(mut self, value: Vec<CardType>) -> Self {
        self.tiers = Some(value);
        self
    }

    pub fn visible(mut self, value: Visible) -> Self {
        self.visible = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CardIcpSection`].
    pub fn build(self) -> Result<CardIcpSection, BuildError> {
        Ok(CardIcpSection {
            tiers: self.tiers,
            visible: self.visible,
        })
    }
}
