pub use crate::prelude::*;

/// Details about pricing and payment services for a business.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct ServicesSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<AchService>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<CardService>,
    #[serde(rename = "subFooter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_footer: Option<SubFooter>,
    #[serde(rename = "subHeader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_header: Option<SubHeader>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Visible>,
}

impl ServicesSection {
    pub fn builder() -> ServicesSectionBuilder {
        <ServicesSectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ServicesSectionBuilder {
    ach: Option<AchService>,
    card: Option<CardService>,
    sub_footer: Option<SubFooter>,
    sub_header: Option<SubHeader>,
    visible: Option<Visible>,
}

impl ServicesSectionBuilder {
    pub fn ach(mut self, value: AchService) -> Self {
        self.ach = Some(value);
        self
    }

    pub fn card(mut self, value: CardService) -> Self {
        self.card = Some(value);
        self
    }

    pub fn sub_footer(mut self, value: SubFooter) -> Self {
        self.sub_footer = Some(value);
        self
    }

    pub fn sub_header(mut self, value: SubHeader) -> Self {
        self.sub_header = Some(value);
        self
    }

    pub fn visible(mut self, value: Visible) -> Self {
        self.visible = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ServicesSection`].
    pub fn build(self) -> Result<ServicesSection, BuildError> {
        Ok(ServicesSection {
            ach: self.ach,
            card: self.card,
            sub_footer: self.sub_footer,
            sub_header: self.sub_header,
            visible: self.visible,
        })
    }
}
