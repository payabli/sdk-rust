pub use crate::prelude::*;

/// Information about a business owner.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
