pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct UserData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<Vec<UsrAccess>>,
    #[serde(rename = "additionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalData>,
    /// The user's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
    #[serde(rename = "mfaData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mfa_data: Option<MfaData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<NameUser>,
    /// The user's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<PhoneNumber>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pwd: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<OrgScope>>,
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<Timezone>,
    #[serde(rename = "usrStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usr_status: Option<UsrStatus>,
}

impl UserData {
    pub fn builder() -> UserDataBuilder {
        <UserDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserDataBuilder {
    access: Option<Vec<UsrAccess>>,
    additional_data: Option<AdditionalData>,
    email: Option<Email>,
    language: Option<Language>,
    mfa_data: Option<MfaData>,
    name: Option<NameUser>,
    phone: Option<PhoneNumber>,
    pwd: Option<String>,
    scope: Option<Vec<OrgScope>>,
    time_zone: Option<Timezone>,
    usr_status: Option<UsrStatus>,
}

impl UserDataBuilder {
    pub fn access(mut self, value: Vec<UsrAccess>) -> Self {
        self.access = Some(value);
        self
    }

    pub fn additional_data(mut self, value: AdditionalData) -> Self {
        self.additional_data = Some(value);
        self
    }

    pub fn email(mut self, value: Email) -> Self {
        self.email = Some(value);
        self
    }

    pub fn language(mut self, value: Language) -> Self {
        self.language = Some(value);
        self
    }

    pub fn mfa_data(mut self, value: MfaData) -> Self {
        self.mfa_data = Some(value);
        self
    }

    pub fn name(mut self, value: NameUser) -> Self {
        self.name = Some(value);
        self
    }

    pub fn phone(mut self, value: PhoneNumber) -> Self {
        self.phone = Some(value);
        self
    }

    pub fn pwd(mut self, value: impl Into<String>) -> Self {
        self.pwd = Some(value.into());
        self
    }

    pub fn scope(mut self, value: Vec<OrgScope>) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn time_zone(mut self, value: Timezone) -> Self {
        self.time_zone = Some(value);
        self
    }

    pub fn usr_status(mut self, value: UsrStatus) -> Self {
        self.usr_status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserData`].
    pub fn build(self) -> Result<UserData, BuildError> {
        Ok(UserData {
            access: self.access,
            additional_data: self.additional_data,
            email: self.email,
            language: self.language,
            mfa_data: self.mfa_data,
            name: self.name,
            phone: self.phone,
            pwd: self.pwd,
            scope: self.scope,
            time_zone: self.time_zone,
            usr_status: self.usr_status,
        })
    }
}
