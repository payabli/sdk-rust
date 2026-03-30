pub use crate::prelude::*;

/// Object containing the template's data.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TemplateData {
    /// The ID of the organization the template belongs to.
    #[serde(rename = "orgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<Orgid>,
    #[serde(rename = "pricingId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pricing_id: Option<i64>,
    #[serde(rename = "templateCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_code: Option<TemplateCode>,
    #[serde(rename = "templateContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_content: Option<TemplateContent>,
    /// A description for the template.
    #[serde(rename = "templateDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
    #[serde(rename = "templateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<TemplateName>,
}

impl TemplateData {
    pub fn builder() -> TemplateDataBuilder {
        <TemplateDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TemplateDataBuilder {
    org_id: Option<Orgid>,
    pricing_id: Option<i64>,
    template_code: Option<TemplateCode>,
    template_content: Option<TemplateContent>,
    template_description: Option<String>,
    template_name: Option<TemplateName>,
}

impl TemplateDataBuilder {
    pub fn org_id(mut self, value: Orgid) -> Self {
        self.org_id = Some(value);
        self
    }

    pub fn pricing_id(mut self, value: i64) -> Self {
        self.pricing_id = Some(value);
        self
    }

    pub fn template_code(mut self, value: TemplateCode) -> Self {
        self.template_code = Some(value);
        self
    }

    pub fn template_content(mut self, value: TemplateContent) -> Self {
        self.template_content = Some(value);
        self
    }

    pub fn template_description(mut self, value: impl Into<String>) -> Self {
        self.template_description = Some(value.into());
        self
    }

    pub fn template_name(mut self, value: TemplateName) -> Self {
        self.template_name = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TemplateData`].
    pub fn build(self) -> Result<TemplateData, BuildError> {
        Ok(TemplateData {
            org_id: self.org_id,
            pricing_id: self.pricing_id,
            template_code: self.template_code,
            template_content: self.template_content,
            template_description: self.template_description,
            template_name: self.template_name,
        })
    }
}
