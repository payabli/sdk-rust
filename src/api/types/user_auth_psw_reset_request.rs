pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserAuthPswResetRequest {
    /// New User password
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psw: Option<String>,
}

impl UserAuthPswResetRequest {
    pub fn builder() -> UserAuthPswResetRequestBuilder {
        <UserAuthPswResetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserAuthPswResetRequestBuilder {
    psw: Option<String>,
}

impl UserAuthPswResetRequestBuilder {
    pub fn psw(mut self, value: impl Into<String>) -> Self {
        self.psw = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UserAuthPswResetRequest`].
    pub fn build(self) -> Result<UserAuthPswResetRequest, BuildError> {
        Ok(UserAuthPswResetRequest { psw: self.psw })
    }
}
