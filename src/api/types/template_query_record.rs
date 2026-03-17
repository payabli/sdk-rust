pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TemplateQueryRecord {
    #[serde(rename = "addPrice")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_price: Option<bool>,
    #[serde(rename = "boardingLinks")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boarding_links: Option<Vec<BoardingQueryLinks>>,
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    #[serde(rename = "idTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_template: Option<i64>,
    #[serde(rename = "isRoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_root: Option<IsRoot>,
    #[serde(rename = "orgParentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_parent_name: Option<OrgParentName>,
    #[serde(rename = "recipientEmailNotification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_email_notification: Option<RecipientEmailNotification>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resumable: Option<Resumable>,
    #[serde(rename = "templateCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_code: Option<TemplateCode>,
    #[serde(rename = "templateContent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_content: Option<TemplateContentResponse>,
    #[serde(rename = "templateDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_description: Option<String>,
    #[serde(rename = "templateTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_title: Option<String>,
    #[serde(rename = "usedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub used_by: Option<i64>,
}
