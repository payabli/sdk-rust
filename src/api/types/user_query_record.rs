pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct UserQueryRecord {
    #[serde(rename = "Access")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access: Option<Vec<UsrAccess>>,
    #[serde(rename = "AdditionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalDataString>,
    /// The timestamp for the user's creation, in UTC.
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    /// The user's email address.
    #[serde(rename = "Email")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<Email>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<Language>,
    /// The timestamp for the user's last activity, in UTC.
    #[serde(rename = "lastAccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub last_access: Option<DateTime<Utc>>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<NameUser>,
    /// The user's phone number.
    #[serde(rename = "Phone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<PhoneNumber>,
    #[serde(rename = "Scope")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<Vec<OrgXScope>>,
    /// Additional data provided by the social network related to the customer.
    #[serde(rename = "snData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sn_data: Option<String>,
    /// Identifier or token for customer in linked social network.
    #[serde(rename = "snIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sn_identifier: Option<String>,
    /// Social network linked to customer. Possible values: facebook, google, twitter, microsoft.
    #[serde(rename = "snProvider")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sn_provider: Option<String>,
    #[serde(rename = "timeZone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_zone: Option<Timezone>,
    /// The user's ID in Payabli.
    #[serde(rename = "userId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<i64>,
    #[serde(rename = "UsrMFA")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usr_mfa: Option<Mfa>,
    #[serde(rename = "UsrMFAMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usr_mfa_mode: Option<MfaMode>,
    #[serde(rename = "UsrStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usr_status: Option<UsrStatus>,
}

impl UserQueryRecord {
    pub fn builder() -> UserQueryRecordBuilder {
        <UserQueryRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct UserQueryRecordBuilder {
    access: Option<Vec<UsrAccess>>,
    additional_data: Option<AdditionalDataString>,
    created_at: Option<CreatedAt>,
    email: Option<Email>,
    language: Option<Language>,
    last_access: Option<DateTime<Utc>>,
    name: Option<NameUser>,
    phone: Option<PhoneNumber>,
    scope: Option<Vec<OrgXScope>>,
    sn_data: Option<String>,
    sn_identifier: Option<String>,
    sn_provider: Option<String>,
    time_zone: Option<Timezone>,
    user_id: Option<i64>,
    usr_mfa: Option<Mfa>,
    usr_mfa_mode: Option<MfaMode>,
    usr_status: Option<UsrStatus>,
}

impl UserQueryRecordBuilder {
    pub fn access(mut self, value: Vec<UsrAccess>) -> Self {
        self.access = Some(value);
        self
    }

    pub fn additional_data(mut self, value: AdditionalDataString) -> Self {
        self.additional_data = Some(value);
        self
    }

    pub fn created_at(mut self, value: CreatedAt) -> Self {
        self.created_at = Some(value);
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

    pub fn last_access(mut self, value: DateTime<Utc>) -> Self {
        self.last_access = Some(value);
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

    pub fn scope(mut self, value: Vec<OrgXScope>) -> Self {
        self.scope = Some(value);
        self
    }

    pub fn sn_data(mut self, value: impl Into<String>) -> Self {
        self.sn_data = Some(value.into());
        self
    }

    pub fn sn_identifier(mut self, value: impl Into<String>) -> Self {
        self.sn_identifier = Some(value.into());
        self
    }

    pub fn sn_provider(mut self, value: impl Into<String>) -> Self {
        self.sn_provider = Some(value.into());
        self
    }

    pub fn time_zone(mut self, value: Timezone) -> Self {
        self.time_zone = Some(value);
        self
    }

    pub fn user_id(mut self, value: i64) -> Self {
        self.user_id = Some(value);
        self
    }

    pub fn usr_mfa(mut self, value: Mfa) -> Self {
        self.usr_mfa = Some(value);
        self
    }

    pub fn usr_mfa_mode(mut self, value: MfaMode) -> Self {
        self.usr_mfa_mode = Some(value);
        self
    }

    pub fn usr_status(mut self, value: UsrStatus) -> Self {
        self.usr_status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`UserQueryRecord`].
    pub fn build(self) -> Result<UserQueryRecord, BuildError> {
        Ok(UserQueryRecord {
            access: self.access,
            additional_data: self.additional_data,
            created_at: self.created_at,
            email: self.email,
            language: self.language,
            last_access: self.last_access,
            name: self.name,
            phone: self.phone,
            scope: self.scope,
            sn_data: self.sn_data,
            sn_identifier: self.sn_identifier,
            sn_provider: self.sn_provider,
            time_zone: self.time_zone,
            user_id: self.user_id,
            usr_mfa: self.usr_mfa,
            usr_mfa_mode: self.usr_mfa_mode,
            usr_status: self.usr_status,
        })
    }
}
