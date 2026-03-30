pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TemplateAdditionalDataField {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Visible>,
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<ReadOnly>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<RequiredElement>,
    #[serde(rename = "posRow")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_row: Option<PosRow>,
    #[serde(rename = "posCol")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pos_col: Option<PosCol>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<ValueTemplates>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
}

impl TemplateAdditionalDataField {
    pub fn builder() -> TemplateAdditionalDataFieldBuilder {
        <TemplateAdditionalDataFieldBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TemplateAdditionalDataFieldBuilder {
    visible: Option<Visible>,
    read_only: Option<ReadOnly>,
    required: Option<RequiredElement>,
    pos_row: Option<PosRow>,
    pos_col: Option<PosCol>,
    value: Option<ValueTemplates>,
    label: Option<String>,
    r#type: Option<String>,
}

impl TemplateAdditionalDataFieldBuilder {
    pub fn visible(mut self, value: Visible) -> Self {
        self.visible = Some(value);
        self
    }

    pub fn read_only(mut self, value: ReadOnly) -> Self {
        self.read_only = Some(value);
        self
    }

    pub fn required(mut self, value: RequiredElement) -> Self {
        self.required = Some(value);
        self
    }

    pub fn pos_row(mut self, value: PosRow) -> Self {
        self.pos_row = Some(value);
        self
    }

    pub fn pos_col(mut self, value: PosCol) -> Self {
        self.pos_col = Some(value);
        self
    }

    pub fn value(mut self, value: ValueTemplates) -> Self {
        self.value = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`TemplateAdditionalDataField`].
    pub fn build(self) -> Result<TemplateAdditionalDataField, BuildError> {
        Ok(TemplateAdditionalDataField {
            visible: self.visible,
            read_only: self.read_only,
            required: self.required,
            pos_row: self.pos_row,
            pos_col: self.pos_col,
            value: self.value,
            label: self.label,
            r#type: self.r#type,
        })
    }
}
