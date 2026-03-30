pub use crate::prelude::*;

/// The wallet data.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
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

impl AppleWalletData {
    pub fn builder() -> AppleWalletDataBuilder {
        <AppleWalletDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AppleWalletDataBuilder {
    entry: Option<Entry>,
    apple_pay_merchant_id: Option<String>,
    domain_names: Option<Vec<DomainName>>,
    paypoint_name: Option<PaypointName>,
    paypoint_url: Option<String>,
    marked_for_deletion_at: Option<DateTime<Utc>>,
    created_at: Option<CreatedAt>,
    updated_at: Option<LastModified>,
    id: Option<ApplePayId>,
    r#type: Option<ApplePayType>,
}

impl AppleWalletDataBuilder {
    pub fn entry(mut self, value: Entry) -> Self {
        self.entry = Some(value);
        self
    }

    pub fn apple_pay_merchant_id(mut self, value: impl Into<String>) -> Self {
        self.apple_pay_merchant_id = Some(value.into());
        self
    }

    pub fn domain_names(mut self, value: Vec<DomainName>) -> Self {
        self.domain_names = Some(value);
        self
    }

    pub fn paypoint_name(mut self, value: PaypointName) -> Self {
        self.paypoint_name = Some(value);
        self
    }

    pub fn paypoint_url(mut self, value: impl Into<String>) -> Self {
        self.paypoint_url = Some(value.into());
        self
    }

    pub fn marked_for_deletion_at(mut self, value: DateTime<Utc>) -> Self {
        self.marked_for_deletion_at = Some(value);
        self
    }

    pub fn created_at(mut self, value: CreatedAt) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: LastModified) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn id(mut self, value: ApplePayId) -> Self {
        self.id = Some(value);
        self
    }

    pub fn r#type(mut self, value: ApplePayType) -> Self {
        self.r#type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AppleWalletData`].
    pub fn build(self) -> Result<AppleWalletData, BuildError> {
        Ok(AppleWalletData {
            entry: self.entry,
            apple_pay_merchant_id: self.apple_pay_merchant_id,
            domain_names: self.domain_names,
            paypoint_name: self.paypoint_name,
            paypoint_url: self.paypoint_url,
            marked_for_deletion_at: self.marked_for_deletion_at,
            created_at: self.created_at,
            updated_at: self.updated_at,
            id: self.id,
            r#type: self.r#type,
        })
    }
}
