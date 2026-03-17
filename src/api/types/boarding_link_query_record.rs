pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct BoardingLinkQueryRecord {
    #[serde(rename = "acceptOauth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_oauth: Option<AcceptOauth>,
    #[serde(rename = "acceptRegister")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_register: Option<AcceptRegister>,
    #[serde(rename = "builderData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub builder_data: Option<BuilderData>,
    #[serde(rename = "entryAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_attributes: Option<EntryAttributes>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    /// Object containing logo file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo: Option<FileContent>,
    #[serde(rename = "orgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<Orgid>,
    #[serde(rename = "pageIdentifier:")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
    #[serde(rename = "recipientEmailNotification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_email_notification: Option<RecipientEmailNotification>,
    #[serde(rename = "referenceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_name: Option<ReferenceName>,
    #[serde(rename = "referenceTemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_template_id: Option<ReferenceTemplateId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resumable: Option<Resumable>,
}
