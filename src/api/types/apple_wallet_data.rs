pub use crate::prelude::*;

/// The wallet data.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AppleWalletData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entry: Option<Entry>,
    /// The Apple Pay merchant identifier.
    #[serde(rename = "applePayMerchantId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay_merchant_id: Option<String>,
    /// A list of domain names that are enabled for this paypoint.
    #[serde(rename = "domainNames")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_names: Option<Vec<DomainName>>,
    #[serde(rename = "paypointName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_name: Option<PaypointName>,
    /// The paypoint URL.
    #[serde(rename = "paypointUrl")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_url: Option<String>,
    /// The date and time a paypoint's Apple Pay registration was scheduled for deletion. The paypoint will be unregistered from Apple Pay permanently 30 days from this value.
    #[serde(rename = "markedForDeletionAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub marked_for_deletion_at: Option<DateTime<Utc>>,
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<CreatedAt>,
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<LastModified>,
    /// Internal ID for the Apple Pay paypoint registration update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<ApplePayId>,
    /// The record type, in this context it will always be `ApplePayRegistration`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<ApplePayType>,
}
