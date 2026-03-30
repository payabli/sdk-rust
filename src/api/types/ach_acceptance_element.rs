pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AchAcceptanceElement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub types: Option<AchTypes>,
    #[serde(rename = "posCol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_col: Option<PosCol>,
    #[serde(rename = "posRow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_row: Option<PosRow>,
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<ReadOnly>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ValueTemplates>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Visible>,
}

impl AchAcceptanceElement {
    pub fn builder() -> AchAcceptanceElementBuilder {
        <AchAcceptanceElementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AchAcceptanceElementBuilder {
    types: Option<AchTypes>,
    pos_col: Option<PosCol>,
    pos_row: Option<PosRow>,
    read_only: Option<ReadOnly>,
    value: Option<ValueTemplates>,
    visible: Option<Visible>,
}

impl AchAcceptanceElementBuilder {
    pub fn types(mut self, value: AchTypes) -> Self {
        self.types = Some(value);
        self
    }

    pub fn pos_col(mut self, value: PosCol) -> Self {
        self.pos_col = Some(value);
        self
    }

    pub fn pos_row(mut self, value: PosRow) -> Self {
        self.pos_row = Some(value);
        self
    }

    pub fn read_only(mut self, value: ReadOnly) -> Self {
        self.read_only = Some(value);
        self
    }

    pub fn value(mut self, value: ValueTemplates) -> Self {
        self.value = Some(value);
        self
    }

    pub fn visible(mut self, value: Visible) -> Self {
        self.visible = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AchAcceptanceElement`].
    pub fn build(self) -> Result<AchAcceptanceElement, BuildError> {
        Ok(AchAcceptanceElement {
            types: self.types,
            pos_col: self.pos_col,
            pos_row: self.pos_row,
            read_only: self.read_only,
            value: self.value,
            visible: self.visible,
        })
    }
}
