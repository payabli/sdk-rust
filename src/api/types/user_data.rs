pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
