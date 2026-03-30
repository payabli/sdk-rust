pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserAuthResetRequest {
    /// The user's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
    /// Identifier for entrypoint originating the request (used by front-end apps)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<String>,
    /// Type of entry identifier: 0 - partner, 2 - paypoint. This is used by front-end apps, required if an Entry is indicated.
    #[serde(rename = "entryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_type: Option<i64>,
}

impl UserAuthResetRequest {
    pub fn builder() -> UserAuthResetRequestBuilder {
        <UserAuthResetRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserAuthResetRequestBuilder {
    email: Option<Email>,
    entry: Option<String>,
    entry_type: Option<i64>,
}

impl UserAuthResetRequestBuilder {
    pub fn email(mut self, value: Email) -> Self {
        self.email = Some(value);
        self
    }

    pub fn entry(mut self, value: impl Into<String>) -> Self {
        self.entry = Some(value.into());
        self
    }

    pub fn entry_type(mut self, value: i64) -> Self {
        self.entry_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserAuthResetRequest`].
    pub fn build(self) -> Result<UserAuthResetRequest, BuildError> {
        Ok(UserAuthResetRequest {
            email: self.email,
            entry: self.entry,
            entry_type: self.entry_type,
        })
    }
}
