pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrgScope {
    #[serde(rename = "orgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<Orgid>,
    #[serde(rename = "orgType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_type: Option<Orgtype>,
}

impl OrgScope {
    pub fn builder() -> OrgScopeBuilder {
        <OrgScopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrgScopeBuilder {
    org_id: Option<Orgid>,
    org_type: Option<Orgtype>,
}

impl OrgScopeBuilder {
    pub fn org_id(mut self, value: Orgid) -> Self {
        self.org_id = Some(value);
        self
    }

    pub fn org_type(mut self, value: Orgtype) -> Self {
        self.org_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrgScope`].
    pub fn build(self) -> Result<OrgScope, BuildError> {
        Ok(OrgScope {
            org_id: self.org_id,
            org_type: self.org_type,
        })
    }
}
