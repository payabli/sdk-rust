pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DocumentSectionTermsAndConditionsTcLinksItem {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}

impl DocumentSectionTermsAndConditionsTcLinksItem {
    pub fn builder() -> DocumentSectionTermsAndConditionsTcLinksItemBuilder {
        <DocumentSectionTermsAndConditionsTcLinksItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentSectionTermsAndConditionsTcLinksItemBuilder {
    label: Option<String>,
    value: Option<String>,
}

impl DocumentSectionTermsAndConditionsTcLinksItemBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DocumentSectionTermsAndConditionsTcLinksItem`].
    pub fn build(self) -> Result<DocumentSectionTermsAndConditionsTcLinksItem, BuildError> {
        Ok(DocumentSectionTermsAndConditionsTcLinksItem {
            label: self.label,
            value: self.value,
        })
    }
}
