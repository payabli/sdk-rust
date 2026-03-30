pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<AchSection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardSection>,
}

impl SSection {
    pub fn builder() -> SSectionBuilder {
        <SSectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SSectionBuilder {
    ach: Option<AchSection>,
    card: Option<CardSection>,
}

impl SSectionBuilder {
    pub fn ach(mut self, value: AchSection) -> Self {
        self.ach = Some(value);
        self
    }

    pub fn card(mut self, value: CardSection) -> Self {
        self.card = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SSection`].
    pub fn build(self) -> Result<SSection, BuildError> {
        Ok(SSection {
            ach: self.ach,
            card: self.card,
        })
    }
}
