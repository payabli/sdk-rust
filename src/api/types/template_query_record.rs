pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
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

impl TemplateQueryRecord {
    pub fn builder() -> TemplateQueryRecordBuilder {
        <TemplateQueryRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TemplateQueryRecordBuilder {
    add_price: Option<bool>,
    boarding_links: Option<Vec<BoardingQueryLinks>>,
    created_at: Option<CreatedAt>,
    id_template: Option<i64>,
    is_root: Option<IsRoot>,
    org_parent_name: Option<OrgParentName>,
    recipient_email_notification: Option<RecipientEmailNotification>,
    resumable: Option<Resumable>,
    template_code: Option<TemplateCode>,
    template_content: Option<TemplateContentResponse>,
    template_description: Option<String>,
    template_title: Option<String>,
    used_by: Option<i64>,
}

impl TemplateQueryRecordBuilder {
    pub fn add_price(mut self, value: bool) -> Self {
        self.add_price = Some(value);
        self
    }

    pub fn boarding_links(mut self, value: Vec<BoardingQueryLinks>) -> Self {
        self.boarding_links = Some(value);
        self
    }

    pub fn created_at(mut self, value: CreatedAt) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn id_template(mut self, value: i64) -> Self {
        self.id_template = Some(value);
        self
    }

    pub fn is_root(mut self, value: IsRoot) -> Self {
        self.is_root = Some(value);
        self
    }

    pub fn org_parent_name(mut self, value: OrgParentName) -> Self {
        self.org_parent_name = Some(value);
        self
    }

    pub fn recipient_email_notification(mut self, value: RecipientEmailNotification) -> Self {
        self.recipient_email_notification = Some(value);
        self
    }

    pub fn resumable(mut self, value: Resumable) -> Self {
        self.resumable = Some(value);
        self
    }

    pub fn template_code(mut self, value: TemplateCode) -> Self {
        self.template_code = Some(value);
        self
    }

    pub fn template_content(mut self, value: TemplateContentResponse) -> Self {
        self.template_content = Some(value);
        self
    }

    pub fn template_description(mut self, value: impl Into<String>) -> Self {
        self.template_description = Some(value.into());
        self
    }

    pub fn template_title(mut self, value: impl Into<String>) -> Self {
        self.template_title = Some(value.into());
        self
    }

    pub fn used_by(mut self, value: i64) -> Self {
        self.used_by = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TemplateQueryRecord`].
    pub fn build(self) -> Result<TemplateQueryRecord, BuildError> {
        Ok(TemplateQueryRecord {
            add_price: self.add_price,
            boarding_links: self.boarding_links,
            created_at: self.created_at,
            id_template: self.id_template,
            is_root: self.is_root,
            org_parent_name: self.org_parent_name,
            recipient_email_notification: self.recipient_email_notification,
            resumable: self.resumable,
            template_code: self.template_code,
            template_content: self.template_content,
            template_description: self.template_description,
            template_title: self.template_title,
            used_by: self.used_by,
        })
    }
}
