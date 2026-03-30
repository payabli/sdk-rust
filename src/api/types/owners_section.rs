pub use crate::prelude::*;

/// Information about a business owner.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OwnersSection {
    #[serde(rename = "contactEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<TemplateElement>,
    #[serde(rename = "contactName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_name: Option<TemplateElement>,
    #[serde(rename = "contactPhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_phone: Option<TemplateElement>,
    #[serde(rename = "contactTitle")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_title: Option<TemplateElement>,
    /// Offer add more contacts
    #[serde(rename = "multipleContacts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_contacts: Option<bool>,
    /// offer add more owners
    #[serde(rename = "multipleOwners")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiple_owners: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oaddress: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocity: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocountry: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub odriverstate: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ostate: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownerdob: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownerdriver: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owneremail: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownername: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownerpercent: Option<TemplateElement>,
    #[serde(rename = "ownerphone1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownerphone_1: Option<TemplateElement>,
    #[serde(rename = "ownerphone2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownerphone_2: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownerssn: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownertitle: Option<TemplateElement>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ozip: Option<TemplateElement>,
    #[serde(rename = "subFooter")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_footer: Option<SubFooter>,
    #[serde(rename = "subHeader")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_header: Option<SubHeader>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visible: Option<Visible>,
    #[serde(rename = "additionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<TemplateAdditionalDataSection>,
}

impl OwnersSection {
    pub fn builder() -> OwnersSectionBuilder {
        <OwnersSectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OwnersSectionBuilder {
    contact_email: Option<TemplateElement>,
    contact_name: Option<TemplateElement>,
    contact_phone: Option<TemplateElement>,
    contact_title: Option<TemplateElement>,
    multiple_contacts: Option<bool>,
    multiple_owners: Option<bool>,
    oaddress: Option<TemplateElement>,
    ocity: Option<TemplateElement>,
    ocountry: Option<TemplateElement>,
    odriverstate: Option<TemplateElement>,
    ostate: Option<TemplateElement>,
    ownerdob: Option<TemplateElement>,
    ownerdriver: Option<TemplateElement>,
    owneremail: Option<TemplateElement>,
    ownername: Option<TemplateElement>,
    ownerpercent: Option<TemplateElement>,
    ownerphone_1: Option<TemplateElement>,
    ownerphone_2: Option<TemplateElement>,
    ownerssn: Option<TemplateElement>,
    ownertitle: Option<TemplateElement>,
    ozip: Option<TemplateElement>,
    sub_footer: Option<SubFooter>,
    sub_header: Option<SubHeader>,
    visible: Option<Visible>,
    additional_data: Option<TemplateAdditionalDataSection>,
}

impl OwnersSectionBuilder {
    pub fn contact_email(mut self, value: TemplateElement) -> Self {
        self.contact_email = Some(value);
        self
    }

    pub fn contact_name(mut self, value: TemplateElement) -> Self {
        self.contact_name = Some(value);
        self
    }

    pub fn contact_phone(mut self, value: TemplateElement) -> Self {
        self.contact_phone = Some(value);
        self
    }

    pub fn contact_title(mut self, value: TemplateElement) -> Self {
        self.contact_title = Some(value);
        self
    }

    pub fn multiple_contacts(mut self, value: bool) -> Self {
        self.multiple_contacts = Some(value);
        self
    }

    pub fn multiple_owners(mut self, value: bool) -> Self {
        self.multiple_owners = Some(value);
        self
    }

    pub fn oaddress(mut self, value: TemplateElement) -> Self {
        self.oaddress = Some(value);
        self
    }

    pub fn ocity(mut self, value: TemplateElement) -> Self {
        self.ocity = Some(value);
        self
    }

    pub fn ocountry(mut self, value: TemplateElement) -> Self {
        self.ocountry = Some(value);
        self
    }

    pub fn odriverstate(mut self, value: TemplateElement) -> Self {
        self.odriverstate = Some(value);
        self
    }

    pub fn ostate(mut self, value: TemplateElement) -> Self {
        self.ostate = Some(value);
        self
    }

    pub fn ownerdob(mut self, value: TemplateElement) -> Self {
        self.ownerdob = Some(value);
        self
    }

    pub fn ownerdriver(mut self, value: TemplateElement) -> Self {
        self.ownerdriver = Some(value);
        self
    }

    pub fn owneremail(mut self, value: TemplateElement) -> Self {
        self.owneremail = Some(value);
        self
    }

    pub fn ownername(mut self, value: TemplateElement) -> Self {
        self.ownername = Some(value);
        self
    }

    pub fn ownerpercent(mut self, value: TemplateElement) -> Self {
        self.ownerpercent = Some(value);
        self
    }

    pub fn ownerphone_1(mut self, value: TemplateElement) -> Self {
        self.ownerphone_1 = Some(value);
        self
    }

    pub fn ownerphone_2(mut self, value: TemplateElement) -> Self {
        self.ownerphone_2 = Some(value);
        self
    }

    pub fn ownerssn(mut self, value: TemplateElement) -> Self {
        self.ownerssn = Some(value);
        self
    }

    pub fn ownertitle(mut self, value: TemplateElement) -> Self {
        self.ownertitle = Some(value);
        self
    }

    pub fn ozip(mut self, value: TemplateElement) -> Self {
        self.ozip = Some(value);
        self
    }

    pub fn sub_footer(mut self, value: SubFooter) -> Self {
        self.sub_footer = Some(value);
        self
    }

    pub fn sub_header(mut self, value: SubHeader) -> Self {
        self.sub_header = Some(value);
        self
    }

    pub fn visible(mut self, value: Visible) -> Self {
        self.visible = Some(value);
        self
    }

    pub fn additional_data(mut self, value: TemplateAdditionalDataSection) -> Self {
        self.additional_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OwnersSection`].
    pub fn build(self) -> Result<OwnersSection, BuildError> {
        Ok(OwnersSection {
            contact_email: self.contact_email,
            contact_name: self.contact_name,
            contact_phone: self.contact_phone,
            contact_title: self.contact_title,
            multiple_contacts: self.multiple_contacts,
            multiple_owners: self.multiple_owners,
            oaddress: self.oaddress,
            ocity: self.ocity,
            ocountry: self.ocountry,
            odriverstate: self.odriverstate,
            ostate: self.ostate,
            ownerdob: self.ownerdob,
            ownerdriver: self.ownerdriver,
            owneremail: self.owneremail,
            ownername: self.ownername,
            ownerpercent: self.ownerpercent,
            ownerphone_1: self.ownerphone_1,
            ownerphone_2: self.ownerphone_2,
            ownerssn: self.ownerssn,
            ownertitle: self.ownertitle,
            ozip: self.ozip,
            sub_footer: self.sub_footer,
            sub_header: self.sub_header,
            visible: self.visible,
            additional_data: self.additional_data,
        })
    }
}
