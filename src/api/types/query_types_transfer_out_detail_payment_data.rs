pub use crate::prelude::*;

/// Payment data for an outbound transfer detail.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TransferOutDetailPaymentData {
    /// Masked account number.
    #[serde(rename = "MaskedAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masked_account: Option<String>,
    /// Type of account.
    #[serde(rename = "AccountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// Account expiration date.
    #[serde(rename = "AccountExp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_exp: Option<String>,
    /// ZIP code associated with the account.
    #[serde(rename = "AccountZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_zip: Option<String>,
    /// Name of the account holder.
    #[serde(rename = "HolderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder_name: Option<String>,
    /// ID of the stored payment method.
    #[serde(rename = "StoredId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_id: Option<String>,
    /// Initiator of the payment.
    #[serde(rename = "Initiator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<String>,
    /// Usage type for stored method.
    #[serde(rename = "StoredMethodUsageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_method_usage_type: Option<String>,
    /// Sequence number.
    #[serde(rename = "Sequence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<String>,
    /// Description of the order.
    #[serde(rename = "orderDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_description: Option<String>,
    /// Cloud signature data.
    #[serde(rename = "cloudSignatureData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_signature_data: Option<String>,
    /// Format of cloud signature.
    #[serde(rename = "cloudSignatureFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_signature_format: Option<String>,
    /// Additional payment details.
    #[serde(rename = "paymentDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_details: Option<serde_json::Value>,
    /// Data about the payor.
    #[serde(rename = "payorData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payor_data: Option<String>,
    /// Account ID.
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    /// Bank account information.
    #[serde(rename = "bankAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<String>,
    /// Gateway connector used.
    #[serde(rename = "gatewayConnector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_connector: Option<String>,
    /// BIN data for the card.
    #[serde(rename = "binData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_data: Option<serde_json::Value>,
}