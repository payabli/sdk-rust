pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TemplateAdditionalDataSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Visible>,
    #[serde(default)]
    pub fields: HashMap<String, TemplateAdditionalDataField>,
}

impl TemplateAdditionalDataSection {
    pub fn builder() -> TemplateAdditionalDataSectionBuilder {
        <TemplateAdditionalDataSectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TemplateAdditionalDataSectionBuilder {
    visible: Option<Visible>,
    fields: Option<HashMap<String, TemplateAdditionalDataField>>,
}

impl TemplateAdditionalDataSectionBuilder {
    pub fn visible(mut self, value: Visible) -> Self {
        self.visible = Some(value);
        self
    }

    pub fn fields(mut self, value: HashMap<String, TemplateAdditionalDataField>) -> Self {
        self.fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TemplateAdditionalDataSection`].
    /// This method will fail if any of the following fields are not set:
    /// - [`fields`](TemplateAdditionalDataSectionBuilder::fields)
    pub fn build(self) -> Result<TemplateAdditionalDataSection, BuildError> {
        Ok(TemplateAdditionalDataSection {
            visible: self.visible,
            fields: self
                .fields
                .ok_or_else(|| BuildError::missing_field("fields"))?,
        })
    }
}
