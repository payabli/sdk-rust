pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OrgXScope {
    #[serde(rename = "orgEntry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_entry: Option<Orgentryname>,
    #[serde(rename = "orgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<Orgid>,
    #[serde(rename = "orgType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_type: Option<Orgtype>,
}

impl OrgXScope {
    pub fn builder() -> OrgXScopeBuilder {
        <OrgXScopeBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrgXScopeBuilder {
    org_entry: Option<Orgentryname>,
    org_id: Option<Orgid>,
    org_type: Option<Orgtype>,
}

impl OrgXScopeBuilder {
    pub fn org_entry(mut self, value: Orgentryname) -> Self {
        self.org_entry = Some(value);
        self
    }

    pub fn org_id(mut self, value: Orgid) -> Self {
        self.org_id = Some(value);
        self
    }

    pub fn org_type(mut self, value: Orgtype) -> Self {
        self.org_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrgXScope`].
    pub fn build(self) -> Result<OrgXScope, BuildError> {
        Ok(OrgXScope {
            org_entry: self.org_entry,
            org_id: self.org_id,
            org_type: self.org_type,
        })
    }
}
