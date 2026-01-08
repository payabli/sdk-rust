pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
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
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_access: Option<DateTime<FixedOffset>>,
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
