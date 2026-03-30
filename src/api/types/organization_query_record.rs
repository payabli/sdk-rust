pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OrganizationQueryRecord {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<OrganizationQueryRecordServicesItem>>,
    #[serde(rename = "billingInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_info: Option<Instrument>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<ContactsField>,
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    #[serde(rename = "hasBilling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_billing: Option<bool>,
    #[serde(rename = "hasResidual")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_residual: Option<bool>,
    #[serde(rename = "idOrg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_org: Option<Orgid>,
    #[serde(rename = "isRoot")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_root: Option<IsRoot>,
    #[serde(rename = "orgAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_address: Option<Orgaddress>,
    #[serde(rename = "orgCity")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_city: Option<Orgcity>,
    #[serde(rename = "orgCountry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_country: Option<Orgcountry>,
    #[serde(rename = "orgEntryName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_entry_name: Option<Orgentryname>,
    #[serde(rename = "orgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<Orgidstring>,
    #[serde(rename = "orgLogo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_logo: Option<FileContent>,
    #[serde(rename = "orgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_name: Option<Orgname>,
    #[serde(rename = "orgParentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_parent_id: Option<OrgParentId>,
    #[serde(rename = "orgParentName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_parent_name: Option<OrgParentName>,
    #[serde(rename = "orgState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_state: Option<Orgstate>,
    #[serde(rename = "orgTimezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_timezone: Option<Orgtimezone>,
    #[serde(rename = "orgType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_type: Option<Orgtype>,
    #[serde(rename = "orgWebsite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_website: Option<Orgwebsite>,
    #[serde(rename = "orgZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_zip: Option<Orgzip>,
    #[serde(rename = "recipientEmailNotification")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_email_notification: Option<RecipientEmailNotification>,
    #[serde(rename = "replyToEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_email: Option<ReplyToEmail>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resumable: Option<Resumable>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<SummaryOrg>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<UserQueryRecord>>,
}

impl OrganizationQueryRecord {
    pub fn builder() -> OrganizationQueryRecordBuilder {
        <OrganizationQueryRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrganizationQueryRecordBuilder {
    services: Option<Vec<OrganizationQueryRecordServicesItem>>,
    billing_info: Option<Instrument>,
    contacts: Option<ContactsField>,
    created_at: Option<CreatedAt>,
    has_billing: Option<bool>,
    has_residual: Option<bool>,
    id_org: Option<Orgid>,
    is_root: Option<IsRoot>,
    org_address: Option<Orgaddress>,
    org_city: Option<Orgcity>,
    org_country: Option<Orgcountry>,
    org_entry_name: Option<Orgentryname>,
    org_id: Option<Orgidstring>,
    org_logo: Option<FileContent>,
    org_name: Option<Orgname>,
    org_parent_id: Option<OrgParentId>,
    org_parent_name: Option<OrgParentName>,
    org_state: Option<Orgstate>,
    org_timezone: Option<Orgtimezone>,
    org_type: Option<Orgtype>,
    org_website: Option<Orgwebsite>,
    org_zip: Option<Orgzip>,
    recipient_email_notification: Option<RecipientEmailNotification>,
    reply_to_email: Option<ReplyToEmail>,
    resumable: Option<Resumable>,
    summary: Option<SummaryOrg>,
    users: Option<Vec<UserQueryRecord>>,
}

impl OrganizationQueryRecordBuilder {
    pub fn services(mut self, value: Vec<OrganizationQueryRecordServicesItem>) -> Self {
        self.services = Some(value);
        self
    }

    pub fn billing_info(mut self, value: Instrument) -> Self {
        self.billing_info = Some(value);
        self
    }

    pub fn contacts(mut self, value: ContactsField) -> Self {
        self.contacts = Some(value);
        self
    }

    pub fn created_at(mut self, value: CreatedAt) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn has_billing(mut self, value: bool) -> Self {
        self.has_billing = Some(value);
        self
    }

    pub fn has_residual(mut self, value: bool) -> Self {
        self.has_residual = Some(value);
        self
    }

    pub fn id_org(mut self, value: Orgid) -> Self {
        self.id_org = Some(value);
        self
    }

    pub fn is_root(mut self, value: IsRoot) -> Self {
        self.is_root = Some(value);
        self
    }

    pub fn org_address(mut self, value: Orgaddress) -> Self {
        self.org_address = Some(value);
        self
    }

    pub fn org_city(mut self, value: Orgcity) -> Self {
        self.org_city = Some(value);
        self
    }

    pub fn org_country(mut self, value: Orgcountry) -> Self {
        self.org_country = Some(value);
        self
    }

    pub fn org_entry_name(mut self, value: Orgentryname) -> Self {
        self.org_entry_name = Some(value);
        self
    }

    pub fn org_id(mut self, value: Orgidstring) -> Self {
        self.org_id = Some(value);
        self
    }

    pub fn org_logo(mut self, value: FileContent) -> Self {
        self.org_logo = Some(value);
        self
    }

    pub fn org_name(mut self, value: Orgname) -> Self {
        self.org_name = Some(value);
        self
    }

    pub fn org_parent_id(mut self, value: OrgParentId) -> Self {
        self.org_parent_id = Some(value);
        self
    }

    pub fn org_parent_name(mut self, value: OrgParentName) -> Self {
        self.org_parent_name = Some(value);
        self
    }

    pub fn org_state(mut self, value: Orgstate) -> Self {
        self.org_state = Some(value);
        self
    }

    pub fn org_timezone(mut self, value: Orgtimezone) -> Self {
        self.org_timezone = Some(value);
        self
    }

    pub fn org_type(mut self, value: Orgtype) -> Self {
        self.org_type = Some(value);
        self
    }

    pub fn org_website(mut self, value: Orgwebsite) -> Self {
        self.org_website = Some(value);
        self
    }

    pub fn org_zip(mut self, value: Orgzip) -> Self {
        self.org_zip = Some(value);
        self
    }

    pub fn recipient_email_notification(mut self, value: RecipientEmailNotification) -> Self {
        self.recipient_email_notification = Some(value);
        self
    }

    pub fn reply_to_email(mut self, value: ReplyToEmail) -> Self {
        self.reply_to_email = Some(value);
        self
    }

    pub fn resumable(mut self, value: Resumable) -> Self {
        self.resumable = Some(value);
        self
    }

    pub fn summary(mut self, value: SummaryOrg) -> Self {
        self.summary = Some(value);
        self
    }

    pub fn users(mut self, value: Vec<UserQueryRecord>) -> Self {
        self.users = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrganizationQueryRecord`].
    pub fn build(self) -> Result<OrganizationQueryRecord, BuildError> {
        Ok(OrganizationQueryRecord {
            services: self.services,
            billing_info: self.billing_info,
            contacts: self.contacts,
            created_at: self.created_at,
            has_billing: self.has_billing,
            has_residual: self.has_residual,
            id_org: self.id_org,
            is_root: self.is_root,
            org_address: self.org_address,
            org_city: self.org_city,
            org_country: self.org_country,
            org_entry_name: self.org_entry_name,
            org_id: self.org_id,
            org_logo: self.org_logo,
            org_name: self.org_name,
            org_parent_id: self.org_parent_id,
            org_parent_name: self.org_parent_name,
            org_state: self.org_state,
            org_timezone: self.org_timezone,
            org_type: self.org_type,
            org_website: self.org_website,
            org_zip: self.org_zip,
            recipient_email_notification: self.recipient_email_notification,
            reply_to_email: self.reply_to_email,
            resumable: self.resumable,
            summary: self.summary,
            users: self.users,
        })
    }
}
