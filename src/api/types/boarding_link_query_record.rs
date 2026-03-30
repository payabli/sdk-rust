pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
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

impl BoardingLinkQueryRecord {
    pub fn builder() -> BoardingLinkQueryRecordBuilder {
        <BoardingLinkQueryRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BoardingLinkQueryRecordBuilder {
    accept_oauth: Option<AcceptOauth>,
    accept_register: Option<AcceptRegister>,
    builder_data: Option<BuilderData>,
    entry_attributes: Option<EntryAttributes>,
    id: Option<i64>,
    logo: Option<FileContent>,
    org_id: Option<Orgid>,
    page_identifier: Option<PageIdentifier>,
    recipient_email_notification: Option<RecipientEmailNotification>,
    reference_name: Option<ReferenceName>,
    reference_template_id: Option<ReferenceTemplateId>,
    resumable: Option<Resumable>,
}

impl BoardingLinkQueryRecordBuilder {
    pub fn accept_oauth(mut self, value: AcceptOauth) -> Self {
        self.accept_oauth = Some(value);
        self
    }

    pub fn accept_register(mut self, value: AcceptRegister) -> Self {
        self.accept_register = Some(value);
        self
    }

    pub fn builder_data(mut self, value: BuilderData) -> Self {
        self.builder_data = Some(value);
        self
    }

    pub fn entry_attributes(mut self, value: EntryAttributes) -> Self {
        self.entry_attributes = Some(value);
        self
    }

    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn logo(mut self, value: FileContent) -> Self {
        self.logo = Some(value);
        self
    }

    pub fn org_id(mut self, value: Orgid) -> Self {
        self.org_id = Some(value);
        self
    }

    pub fn page_identifier(mut self, value: PageIdentifier) -> Self {
        self.page_identifier = Some(value);
        self
    }

    pub fn recipient_email_notification(mut self, value: RecipientEmailNotification) -> Self {
        self.recipient_email_notification = Some(value);
        self
    }

    pub fn reference_name(mut self, value: ReferenceName) -> Self {
        self.reference_name = Some(value);
        self
    }

    pub fn reference_template_id(mut self, value: ReferenceTemplateId) -> Self {
        self.reference_template_id = Some(value);
        self
    }

    pub fn resumable(mut self, value: Resumable) -> Self {
        self.resumable = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BoardingLinkQueryRecord`].
    pub fn build(self) -> Result<BoardingLinkQueryRecord, BuildError> {
        Ok(BoardingLinkQueryRecord {
            accept_oauth: self.accept_oauth,
            accept_register: self.accept_register,
            builder_data: self.builder_data,
            entry_attributes: self.entry_attributes,
            id: self.id,
            logo: self.logo,
            org_id: self.org_id,
            page_identifier: self.page_identifier,
            recipient_email_notification: self.recipient_email_notification,
            reference_name: self.reference_name,
            reference_template_id: self.reference_template_id,
            resumable: self.resumable,
        })
    }
}
