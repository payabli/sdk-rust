pub use crate::prelude::*;

/// Object containing the template's data.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
