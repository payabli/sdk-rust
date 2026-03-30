pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BasicTemplateElement {
    #[serde(rename = "readOnly")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<ReadOnly>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<RequiredElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Visible>,
}

impl BasicTemplateElement {
    pub fn builder() -> BasicTemplateElementBuilder {
        <BasicTemplateElementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BasicTemplateElementBuilder {
    read_only: Option<ReadOnly>,
    required: Option<RequiredElement>,
    visible: Option<Visible>,
}

impl BasicTemplateElementBuilder {
    pub fn read_only(mut self, value: ReadOnly) -> Self {
        self.read_only = Some(value);
        self
    }

    pub fn required(mut self, value: RequiredElement) -> Self {
        self.required = Some(value);
        self
    }

    pub fn visible(mut self, value: Visible) -> Self {
        self.visible = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BasicTemplateElement`].
    pub fn build(self) -> Result<BasicTemplateElement, BuildError> {
        Ok(BasicTemplateElement {
            read_only: self.read_only,
            required: self.required,
            visible: self.visible,
        })
    }
}
