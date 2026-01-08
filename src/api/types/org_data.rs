pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct OrgData {
    #[serde(rename = "idOrg")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_org: Option<Orgid>,
    #[serde(rename = "orgAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_address: Option<Orgaddress>,
    #[serde(rename = "orgLogo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_logo: Option<FileContent>,
    #[serde(rename = "orgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_name: Option<Orgname>,
    /// The paypoint's status.
    ///
    /// Active - `1`
    ///
    /// Inactive - 0
    #[serde(rename = "orgStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_status: Option<i64>,
    #[serde(rename = "orgType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_type: Option<Orgtype>,
}
