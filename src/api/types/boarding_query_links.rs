pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
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

impl BoardingQueryLinks {
    pub fn builder() -> BoardingQueryLinksBuilder {
        <BoardingQueryLinksBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BoardingQueryLinksBuilder {
    accept_oauth: Option<AcceptOauth>,
    accept_register: Option<AcceptRegister>,
    entry_attributes: Option<EntryAttributes>,
    id: Option<BoardingLinkId>,
    last_updated: Option<LastModified>,
    org_parent_name: Option<OrgParentName>,
    reference_name: Option<ReferenceName>,
    reference_template_id: Option<ReferenceTemplateId>,
    template_code: Option<TemplateCode>,
    template_name: Option<TemplateName>,
}

impl BoardingQueryLinksBuilder {
    pub fn accept_oauth(mut self, value: AcceptOauth) -> Self {
        self.accept_oauth = Some(value);
        self
    }

    pub fn accept_register(mut self, value: AcceptRegister) -> Self {
        self.accept_register = Some(value);
        self
    }

    pub fn entry_attributes(mut self, value: EntryAttributes) -> Self {
        self.entry_attributes = Some(value);
        self
    }

    pub fn id(mut self, value: BoardingLinkId) -> Self {
        self.id = Some(value);
        self
    }

    pub fn last_updated(mut self, value: LastModified) -> Self {
        self.last_updated = Some(value);
        self
    }

    pub fn org_parent_name(mut self, value: OrgParentName) -> Self {
        self.org_parent_name = Some(value);
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

    pub fn template_code(mut self, value: TemplateCode) -> Self {
        self.template_code = Some(value);
        self
    }

    pub fn template_name(mut self, value: TemplateName) -> Self {
        self.template_name = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BoardingQueryLinks`].
    pub fn build(self) -> Result<BoardingQueryLinks, BuildError> {
        Ok(BoardingQueryLinks {
            accept_oauth: self.accept_oauth,
            accept_register: self.accept_register,
            entry_attributes: self.entry_attributes,
            id: self.id,
            last_updated: self.last_updated,
            org_parent_name: self.org_parent_name,
            reference_name: self.reference_name,
            reference_template_id: self.reference_template_id,
            template_code: self.template_code,
            template_name: self.template_name,
        })
    }
}
