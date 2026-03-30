pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DocumentSectionTermsAndConditions {
    #[serde(rename = "tcLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tc_links: Option<Vec<DocumentSectionTermsAndConditionsTcLinksItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Visible>,
}

impl DocumentSectionTermsAndConditions {
    pub fn builder() -> DocumentSectionTermsAndConditionsBuilder {
        <DocumentSectionTermsAndConditionsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DocumentSectionTermsAndConditionsBuilder {
    tc_links: Option<Vec<DocumentSectionTermsAndConditionsTcLinksItem>>,
    visible: Option<Visible>,
}

impl DocumentSectionTermsAndConditionsBuilder {
    pub fn tc_links(mut self, value: Vec<DocumentSectionTermsAndConditionsTcLinksItem>) -> Self {
        self.tc_links = Some(value);
        self
    }

    pub fn visible(mut self, value: Visible) -> Self {
        self.visible = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DocumentSectionTermsAndConditions`].
    pub fn build(self) -> Result<DocumentSectionTermsAndConditions, BuildError> {
        Ok(DocumentSectionTermsAndConditions {
            tc_links: self.tc_links,
            visible: self.visible,
        })
    }
}
