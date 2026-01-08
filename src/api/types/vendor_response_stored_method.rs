pub use crate::prelude::*;

/// Stored payment method information
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct VendorResponseStoredMethod {
    #[serde(rename = "IdPmethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_pmethod: Option<String>,
    #[serde(rename = "Method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(rename = "Descriptor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<String>,
    #[serde(rename = "MaskedAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masked_account: Option<String>,
    #[serde(rename = "ExpDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_date: Option<String>,
    #[serde(rename = "HolderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder_name: Option<String>,
    #[serde(rename = "AchSecCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_sec_code: Option<String>,
    #[serde(rename = "AchHolderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_holder_type: Option<String>,
    #[serde(rename = "IsValidatedACH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_validated_ach: Option<bool>,
    #[serde(rename = "BIN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<String>,
    #[serde(rename = "binData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_data: Option<String>,
    #[serde(rename = "ABA")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<String>,
    #[serde(rename = "PostalCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(rename = "MethodType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_type: Option<String>,
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_updated: Option<DateTime<FixedOffset>>,
    #[serde(rename = "CardUpdatedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub card_updated_on: Option<DateTime<FixedOffset>>,
}
