pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AddOrganizationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<Vec<ServiceCost>>,
    #[serde(rename = "billingInfo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_info: Option<Instrument>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contacts: Option<ContactsField>,
    #[serde(rename = "hasBilling")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_billing: Option<bool>,
    #[serde(rename = "hasResidual")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_residual: Option<bool>,
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
    #[serde(default)]
    pub org_name: Orgname,
    #[serde(rename = "orgParentId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_parent_id: Option<OrgParentId>,
    #[serde(rename = "orgState")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_state: Option<Orgstate>,
    #[serde(rename = "orgTimezone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_timezone: Option<Orgtimezone>,
    #[serde(rename = "orgType")]
    #[serde(default)]
    pub org_type: Orgtype,
    #[serde(rename = "orgWebsite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_website: Option<Orgwebsite>,
    #[serde(rename = "orgZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_zip: Option<Orgzip>,
    #[serde(rename = "replyToEmail")]
    #[serde(default)]
    pub reply_to_email: ReplyToEmail,
}

impl AddOrganizationRequest {
    pub fn builder() -> AddOrganizationRequestBuilder {
        <AddOrganizationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddOrganizationRequestBuilder {
    services: Option<Vec<ServiceCost>>,
    billing_info: Option<Instrument>,
    contacts: Option<ContactsField>,
    has_billing: Option<bool>,
    has_residual: Option<bool>,
    org_address: Option<Orgaddress>,
    org_city: Option<Orgcity>,
    org_country: Option<Orgcountry>,
    org_entry_name: Option<Orgentryname>,
    org_id: Option<Orgidstring>,
    org_logo: Option<FileContent>,
    org_name: Option<Orgname>,
    org_parent_id: Option<OrgParentId>,
    org_state: Option<Orgstate>,
    org_timezone: Option<Orgtimezone>,
    org_type: Option<Orgtype>,
    org_website: Option<Orgwebsite>,
    org_zip: Option<Orgzip>,
    reply_to_email: Option<ReplyToEmail>,
}

impl AddOrganizationRequestBuilder {
    pub fn services(mut self, value: Vec<ServiceCost>) -> Self {
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

    pub fn has_billing(mut self, value: bool) -> Self {
        self.has_billing = Some(value);
        self
    }

    pub fn has_residual(mut self, value: bool) -> Self {
        self.has_residual = Some(value);
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

    pub fn reply_to_email(mut self, value: ReplyToEmail) -> Self {
        self.reply_to_email = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddOrganizationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`org_name`](AddOrganizationRequestBuilder::org_name)
    /// - [`org_type`](AddOrganizationRequestBuilder::org_type)
    /// - [`reply_to_email`](AddOrganizationRequestBuilder::reply_to_email)
    pub fn build(self) -> Result<AddOrganizationRequest, BuildError> {
        Ok(AddOrganizationRequest {
            services: self.services,
            billing_info: self.billing_info,
            contacts: self.contacts,
            has_billing: self.has_billing,
            has_residual: self.has_residual,
            org_address: self.org_address,
            org_city: self.org_city,
            org_country: self.org_country,
            org_entry_name: self.org_entry_name,
            org_id: self.org_id,
            org_logo: self.org_logo,
            org_name: self
                .org_name
                .ok_or_else(|| BuildError::missing_field("org_name"))?,
            org_parent_id: self.org_parent_id,
            org_state: self.org_state,
            org_timezone: self.org_timezone,
            org_type: self
                .org_type
                .ok_or_else(|| BuildError::missing_field("org_type"))?,
            org_website: self.org_website,
            org_zip: self.org_zip,
            reply_to_email: self
                .reply_to_email
                .ok_or_else(|| BuildError::missing_field("reply_to_email"))?,
        })
    }
}
