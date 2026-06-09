pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
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
    /// Active - `1`
    /// Inactive - 0
    #[serde(rename = "orgStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_status: Option<i64>,
    #[serde(rename = "orgType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_type: Option<Orgtype>,
}

impl OrgData {
    pub fn builder() -> OrgDataBuilder {
        <OrgDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrgDataBuilder {
    id_org: Option<Orgid>,
    org_address: Option<Orgaddress>,
    org_logo: Option<FileContent>,
    org_name: Option<Orgname>,
    org_status: Option<i64>,
    org_type: Option<Orgtype>,
}

impl OrgDataBuilder {
    pub fn id_org(mut self, value: Orgid) -> Self {
        self.id_org = Some(value);
        self
    }

    pub fn org_address(mut self, value: Orgaddress) -> Self {
        self.org_address = Some(value);
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

    pub fn org_status(mut self, value: i64) -> Self {
        self.org_status = Some(value);
        self
    }

    pub fn org_type(mut self, value: Orgtype) -> Self {
        self.org_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrgData`].
    pub fn build(self) -> Result<OrgData, BuildError> {
        Ok(OrgData {
            id_org: self.id_org,
            org_address: self.org_address,
            org_logo: self.org_logo,
            org_name: self.org_name,
            org_status: self.org_status,
            org_type: self.org_type,
        })
    }
}
