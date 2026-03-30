pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CardFlatSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<CardType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Visible>,
}

impl CardFlatSection {
    pub fn builder() -> CardFlatSectionBuilder {
        <CardFlatSectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CardFlatSectionBuilder {
    tiers: Option<Vec<CardType>>,
    visible: Option<Visible>,
}

impl CardFlatSectionBuilder {
    pub fn tiers(mut self, value: Vec<CardType>) -> Self {
        self.tiers = Some(value);
        self
    }

    pub fn visible(mut self, value: Visible) -> Self {
        self.visible = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CardFlatSection`].
    pub fn build(self) -> Result<CardFlatSection, BuildError> {
        Ok(CardFlatSection {
            tiers: self.tiers,
            visible: self.visible,
        })
    }
}
