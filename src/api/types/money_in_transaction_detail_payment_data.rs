pub use crate::prelude::*;

/// Payment method and transaction details
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransactionDetailPaymentData {
    #[serde(rename = "maskedAccount")]
    pub masked_account: Maskedaccount,
    #[serde(rename = "accountType")]
    pub account_type: Accounttype,
    #[serde(rename = "accountExp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_exp: Option<Accountexp>,
    #[serde(rename = "holderName")]
    pub holder_name: Holdername,
    #[serde(rename = "storedId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_id: Option<Storedmethodid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<Initiator>,
    #[serde(rename = "storedMethodUsageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_method_usage_type: Option<StoredMethodUsageType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<Sequence>,
    #[serde(rename = "orderDescription")]
    pub order_description: Orderdescription,
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<Accountid>,
    #[serde(rename = "signatureData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_data: Option<Signaturedata>,
    #[serde(rename = "binData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_data: Option<BinData>,
    #[serde(rename = "paymentDetails")]
    pub payment_details: TransactionDetailPaymentDetails,
}
