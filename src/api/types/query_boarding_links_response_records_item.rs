pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryBoardingLinksResponseRecordsItem {
    #[serde(rename = "AcceptOauth")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_oauth: Option<AcceptOauth>,
    #[serde(rename = "AcceptRegister")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accept_register: Option<AcceptRegister>,
    #[serde(rename = "EntryAttributes")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_attributes: Option<EntryAttributes>,
    /// The record ID.
    #[serde(rename = "Id")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<LastModified>,
    #[serde(rename = "OrgParentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_parent_name: Option<OrgParentName>,
    #[serde(rename = "ReferenceName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_name: Option<ReferenceName>,
    #[serde(rename = "ReferenceTemplateId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_template_id: Option<ReferenceTemplateId>,
    #[serde(rename = "TemplateCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_code: Option<TemplateCode>,
    #[serde(rename = "TemplateName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub template_name: Option<TemplateName>,
}

impl QueryBoardingLinksResponseRecordsItem {
    pub fn builder() -> QueryBoardingLinksResponseRecordsItemBuilder {
        <QueryBoardingLinksResponseRecordsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryBoardingLinksResponseRecordsItemBuilder {
    accept_oauth: Option<AcceptOauth>,
    accept_register: Option<AcceptRegister>,
    entry_attributes: Option<EntryAttributes>,
    id: Option<i64>,
    last_updated: Option<LastModified>,
    org_parent_name: Option<OrgParentName>,
    reference_name: Option<ReferenceName>,
    reference_template_id: Option<ReferenceTemplateId>,
    template_code: Option<TemplateCode>,
    template_name: Option<TemplateName>,
}

impl QueryBoardingLinksResponseRecordsItemBuilder {
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

    pub fn id(mut self, value: i64) -> Self {
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

    /// Consumes the builder and constructs a [`QueryBoardingLinksResponseRecordsItem`].
    pub fn build(self) -> Result<QueryBoardingLinksResponseRecordsItem, BuildError> {
        Ok(QueryBoardingLinksResponseRecordsItem {
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
