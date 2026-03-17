pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BoardingQueryLinks {
    #[serde(rename = "acceptOauth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_oauth: Option<AcceptOauth>,
    #[serde(rename = "acceptRegister")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_register: Option<AcceptRegister>,
    #[serde(rename = "entryAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_attributes: Option<EntryAttributes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<BoardingLinkId>,
    #[serde(rename = "lastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<LastModified>,
    #[serde(rename = "orgParentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_parent_name: Option<OrgParentName>,
    #[serde(rename = "referenceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_name: Option<ReferenceName>,
    #[serde(rename = "referenceTemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_template_id: Option<ReferenceTemplateId>,
    #[serde(rename = "templateCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_code: Option<TemplateCode>,
    #[serde(rename = "templateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<TemplateName>,
}
