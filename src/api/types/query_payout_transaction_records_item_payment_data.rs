pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct QueryPayoutTransactionRecordsItemPaymentData {
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
    #[serde(rename = "bankAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<String>,
    #[serde(rename = "binData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_data: Option<BinData>,
    #[serde(rename = "cloudSignatureData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_signature_data: Option<String>,
    #[serde(rename = "cloudSignatureFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_signature_format: Option<String>,
    #[serde(rename = "gatewayConnector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_connector: Option<PayoutGatewayConnector>,
    /// Card or bank account holder name.
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
    #[serde(rename = "payorData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payor_data: Option<String>,
    #[serde(rename = "Sequence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<Sequence>,
    /// Identifier of stored payment method used in transaction.
    #[serde(rename = "StoredId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_id: Option<Storedmethodid>,
    #[serde(rename = "StoredMethodUsageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_method_usage_type: Option<StoredMethodUsageType>,
}
