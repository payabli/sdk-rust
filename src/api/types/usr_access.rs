pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UsrAccess {
    #[serde(rename = "roleLabel")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_label: Option<String>,
    #[serde(rename = "roleValue")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_value: Option<bool>,
}

impl UsrAccess {
    pub fn builder() -> UsrAccessBuilder {
        <UsrAccessBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UsrAccessBuilder {
    role_label: Option<String>,
    role_value: Option<bool>,
}

impl UsrAccessBuilder {
    pub fn role_label(mut self, value: impl Into<String>) -> Self {
        self.role_label = Some(value.into());
        self
    }

    pub fn role_value(mut self, value: bool) -> Self {
        self.role_value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UsrAccess`].
    pub fn build(self) -> Result<UsrAccess, BuildError> {
        Ok(UsrAccess {
            role_label: self.role_label,
            role_value: self.role_value,
        })
    }
}
