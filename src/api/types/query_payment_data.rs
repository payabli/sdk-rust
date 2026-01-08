pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryPaymentData {
    #[serde(rename = "AccountExp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_exp: Option<Accountexp>,
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<Accountid>,
    #[serde(rename = "AccountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<Accounttype>,
    #[serde(rename = "AccountZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_zip: Option<Accountzip>,
    #[serde(rename = "binData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_data: Option<BinData>,
    #[serde(rename = "HolderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder_name: Option<Holdername>,
    #[serde(rename = "Initiator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<Initiator>,
    #[serde(rename = "MaskedAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masked_account: Option<Maskedaccount>,
    #[serde(rename = "orderDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_description: Option<Orderdescription>,
    #[serde(rename = "paymentDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_details: Option<PaymentDetail>,
    #[serde(rename = "Sequence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<Sequence>,
    #[serde(rename = "SignatureData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_data: Option<Signaturedata>,
    /// Identifier of stored payment method used in transaction.
    #[serde(rename = "StoredId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_id: Option<Storedmethodid>,
    #[serde(rename = "StoredMethodUsageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_method_usage_type: Option<StoredMethodUsageType>,
}
