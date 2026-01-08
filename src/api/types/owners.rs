pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct Owners {
    /// Person who is registered as the beneficial owner of the business. This is a combination of first and last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownername: Option<String>,
    /// The job title of the person such as CEO or director.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownertitle: Option<String>,
    /// Percentage of ownership the person holds, in integer format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownerpercent: Option<i64>,
    /// The relevant identifier for the person such as a Social Security Number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownerssn: Option<String>,
    /// Owner's date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownerdob: Option<String>,
    /// Owner phone 1.
    #[serde(rename = "ownerphone1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownerphone_1: Option<String>,
    /// Owner phone 2.
    #[serde(rename = "ownerphone2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownerphone_2: Option<String>,
    /// Owner email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owneremail: Option<Email>,
    /// Owner driver's license ID number. Payabli strongly recommends including this.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownerdriver: Option<String>,
    /// Owner street address. This must be the physical address of the owner, not a P.O. box.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oaddress: Option<String>,
    /// Owner address city.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocity: Option<String>,
    /// Owner address country in ISO-3166-1 alpha 2 format. Check out https://en.wikipedia.org/wiki/ISO_3166-1 for reference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ocountry: Option<String>,
    /// Owner driver's license State. Payabli strongly recommends including this.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub odriverstate: Option<String>,
    /// Owner address state.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ostate: Option<String>,
    /// Owner address ZIP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ozip: Option<String>,
    #[serde(rename = "additionalData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_data: Option<AdditionalDataString>,
}
