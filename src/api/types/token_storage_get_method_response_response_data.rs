pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetMethodResponseResponseData {
    /// Bank routing number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<String>,
    #[serde(rename = "achHolderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_holder_type: Option<AchHolderType>,
    #[serde(rename = "achSecCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_sec_code: Option<AchSecCode>,
    /// The bank identification number (BIN)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<String>,
    #[serde(rename = "binData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_data: Option<BinData>,
    /// Timestamp for when card was last updated
    #[serde(rename = "cardUpdatedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub card_updated_on: Option<DateTime<FixedOffset>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customers: Option<Vec<GetMethodResponseResponseDataCustomersItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<Descriptor>,
    /// Expiration date for card in stored method in format MM/YY
    #[serde(rename = "expDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_date: Option<String>,
    /// Account holder name in stored method
    #[serde(rename = "holderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder_name: Option<Holdername>,
    /// The stored payment method's identifier in Payabli
    #[serde(rename = "idPmethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_pmethod: Option<String>,
    /// Whether the ACH account has been validated
    #[serde(rename = "isValidatedACH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_validated_ach: Option<bool>,
    /// Timestamp for last update of stored method, in UTC
    #[serde(rename = "lastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_updated: Option<DateTime<FixedOffset>>,
    #[serde(rename = "maskedAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masked_account: Option<Maskedaccount>,
    /// The saved method's type: `card` or `ach`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// The payment method's token type
    #[serde(rename = "methodType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_type: Option<String>,
    /// The payment method postal code
    #[serde(rename = "postalCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendors: Option<Vec<GetMethodResponseResponseDataVendorsItem>>,
}
