pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OSection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_list: Option<CList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub own_list: Option<OList>,
}

impl OSection {
    pub fn builder() -> OSectionBuilder {
        <OSectionBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OSectionBuilder {
    contact_list: Option<CList>,
    own_list: Option<OList>,
}

impl OSectionBuilder {
    pub fn contact_list(mut self, value: CList) -> Self {
        self.contact_list = Some(value);
        self
    }

    pub fn own_list(mut self, value: OList) -> Self {
        self.own_list = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OSection`].
    pub fn build(self) -> Result<OSection, BuildError> {
        Ok(OSection {
            contact_list: self.contact_list,
            own_list: self.own_list,
        })
    }
}
