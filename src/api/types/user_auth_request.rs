pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserAuthRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
    /// Identifier for entry point originating the request (used by front-end apps)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<String>,
    /// Type of entry identifier: 0 - partner, 2 - paypoint. This is used by front-end apps, required if an Entry is indicated.
    #[serde(rename = "entryType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry_type: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub psw: Option<String>,
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(rename = "userTokenId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_token_id: Option<String>,
}

impl UserAuthRequest {
    pub fn builder() -> UserAuthRequestBuilder {
        <UserAuthRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserAuthRequestBuilder {
    email: Option<Email>,
    entry: Option<String>,
    entry_type: Option<i64>,
    psw: Option<String>,
    user_id: Option<i64>,
    user_token_id: Option<String>,
}

impl UserAuthRequestBuilder {
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

    pub fn psw(mut self, value: impl Into<String>) -> Self {
        self.psw = Some(value.into());
        self
    }

    pub fn user_id(mut self, value: i64) -> Self {
        self.user_id = Some(value);
        self
    }

    pub fn user_token_id(mut self, value: impl Into<String>) -> Self {
        self.user_token_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`UserAuthRequest`].
    pub fn build(self) -> Result<UserAuthRequest, BuildError> {
        Ok(UserAuthRequest {
            email: self.email,
            entry: self.entry,
            entry_type: self.entry_type,
            psw: self.psw,
            user_id: self.user_id,
            user_token_id: self.user_token_id,
        })
    }
}
