pub use crate::prelude::*;

/// Request type for API operation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
    pub org_type: Orgtype,
    #[serde(rename = "orgWebsite")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_website: Option<Orgwebsite>,
    #[serde(rename = "orgZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_zip: Option<Orgzip>,
    #[serde(rename = "replyToEmail")]
    pub reply_to_email: ReplyToEmail,
}
