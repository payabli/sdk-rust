pub use crate::prelude::*;

/// A sales or service representative assigned to the application.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RepresentativeResponse {
    /// The representative's user identifier.
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// The representative's code.
    #[serde(rename = "repCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rep_code: Option<i64>,
    /// The representative's first name.
    #[serde(rename = "firstName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    /// The representative's last name.
    #[serde(rename = "lastName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    /// The representative's location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    /// The representative's team.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<String>,
}

impl RepresentativeResponse {
    pub fn builder() -> RepresentativeResponseBuilder {
        <RepresentativeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RepresentativeResponseBuilder {
    user_id: Option<String>,
    rep_code: Option<i64>,
    first_name: Option<String>,
    last_name: Option<String>,
    location: Option<String>,
    team: Option<String>,
}

impl RepresentativeResponseBuilder {
    pub fn user_id(mut self, value: impl Into<String>) -> Self {
        self.user_id = Some(value.into());
        self
    }

    pub fn rep_code(mut self, value: i64) -> Self {
        self.rep_code = Some(value);
        self
    }

    pub fn first_name(mut self, value: impl Into<String>) -> Self {
        self.first_name = Some(value.into());
        self
    }

    pub fn last_name(mut self, value: impl Into<String>) -> Self {
        self.last_name = Some(value.into());
        self
    }

    pub fn location(mut self, value: impl Into<String>) -> Self {
        self.location = Some(value.into());
        self
    }

    pub fn team(mut self, value: impl Into<String>) -> Self {
        self.team = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RepresentativeResponse`].
    pub fn build(self) -> Result<RepresentativeResponse, BuildError> {
        Ok(RepresentativeResponse {
            user_id: self.user_id,
            rep_code: self.rep_code,
            first_name: self.first_name,
            last_name: self.last_name,
            location: self.location,
            team: self.team,
        })
    }
}
