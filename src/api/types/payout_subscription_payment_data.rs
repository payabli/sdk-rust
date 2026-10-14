pub use crate::prelude::*;

/// Payment method details for the payout subscription.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PayoutSubscriptionPaymentData {
    #[serde(rename = "accountExp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_exp: Option<Accountexp>,
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountId>,
    #[serde(rename = "accountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<Accounttype>,
    #[serde(rename = "accountZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_zip: Option<Accountzip>,
    /// Card information. Always `null` on this endpoint.
    #[serde(rename = "binData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_data: Option<BinData>,
    #[serde(rename = "holderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder_name: Option<Holdername>,
    /// The transaction's initiator. Always `null` on this endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<Initiator>,
    #[serde(rename = "maskedAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masked_account: Option<Maskedaccount>,
    #[serde(rename = "orderDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_description: Option<Orderdescription>,
    #[serde(rename = "paymentDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_details: Option<PayoutSubscriptionPaymentDetails>,
    /// Signature captured on a card-present device. Always `null` for payout subscriptions.
    #[serde(rename = "payorSignature")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payor_signature: Option<PayoutSubscriptionPaymentDataPayorSignature>,
    /// The order of the transaction for cardholder-initiated and merchant-initiated transaction purposes. Always `null` on this endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<Sequence>,
    #[serde(rename = "signatureData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_data: Option<Signaturedata>,
    /// Digital wallet type if applicable. Always `null` on this endpoint.
    #[serde(rename = "walletType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_type: Option<String>,
    /// Identifier of stored payment method used in transaction.
    #[serde(rename = "storedId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_id: Option<Storedmethodid>,
    /// The usage type for the stored method. Always `null` on this endpoint.
    #[serde(rename = "storedMethodUsageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_method_usage_type: Option<StoredMethodUsageType>,
}

impl PayoutSubscriptionPaymentData {
    pub fn builder() -> PayoutSubscriptionPaymentDataBuilder {
        <PayoutSubscriptionPaymentDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayoutSubscriptionPaymentDataBuilder {
    account_exp: Option<Accountexp>,
    account_id: Option<AccountId>,
    account_type: Option<Accounttype>,
    account_zip: Option<Accountzip>,
    bin_data: Option<BinData>,
    holder_name: Option<Holdername>,
    initiator: Option<Initiator>,
    masked_account: Option<Maskedaccount>,
    order_description: Option<Orderdescription>,
    payment_details: Option<PayoutSubscriptionPaymentDetails>,
    payor_signature: Option<PayoutSubscriptionPaymentDataPayorSignature>,
    sequence: Option<Sequence>,
    signature_data: Option<Signaturedata>,
    wallet_type: Option<String>,
    stored_id: Option<Storedmethodid>,
    stored_method_usage_type: Option<StoredMethodUsageType>,
}

impl PayoutSubscriptionPaymentDataBuilder {
    pub fn account_exp(mut self, value: Accountexp) -> Self {
        self.account_exp = Some(value);
        self
    }

    pub fn account_id(mut self, value: AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn account_type(mut self, value: Accounttype) -> Self {
        self.account_type = Some(value);
        self
    }

    pub fn account_zip(mut self, value: Accountzip) -> Self {
        self.account_zip = Some(value);
        self
    }

    pub fn bin_data(mut self, value: BinData) -> Self {
        self.bin_data = Some(value);
        self
    }

    pub fn holder_name(mut self, value: Holdername) -> Self {
        self.holder_name = Some(value);
        self
    }

    pub fn initiator(mut self, value: Initiator) -> Self {
        self.initiator = Some(value);
        self
    }

    pub fn masked_account(mut self, value: Maskedaccount) -> Self {
        self.masked_account = Some(value);
        self
    }

    pub fn order_description(mut self, value: Orderdescription) -> Self {
        self.order_description = Some(value);
        self
    }

    pub fn payment_details(mut self, value: PayoutSubscriptionPaymentDetails) -> Self {
        self.payment_details = Some(value);
        self
    }

    pub fn payor_signature(mut self, value: PayoutSubscriptionPaymentDataPayorSignature) -> Self {
        self.payor_signature = Some(value);
        self
    }

    pub fn sequence(mut self, value: Sequence) -> Self {
        self.sequence = Some(value);
        self
    }

    pub fn signature_data(mut self, value: Signaturedata) -> Self {
        self.signature_data = Some(value);
        self
    }

    pub fn wallet_type(mut self, value: impl Into<String>) -> Self {
        self.wallet_type = Some(value.into());
        self
    }

    pub fn stored_id(mut self, value: Storedmethodid) -> Self {
        self.stored_id = Some(value);
        self
    }

    pub fn stored_method_usage_type(mut self, value: StoredMethodUsageType) -> Self {
        self.stored_method_usage_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayoutSubscriptionPaymentData`].
    pub fn build(self) -> Result<PayoutSubscriptionPaymentData, BuildError> {
        Ok(PayoutSubscriptionPaymentData {
            account_exp: self.account_exp,
            account_id: self.account_id,
            account_type: self.account_type,
            account_zip: self.account_zip,
            bin_data: self.bin_data,
            holder_name: self.holder_name,
            initiator: self.initiator,
            masked_account: self.masked_account,
            order_description: self.order_description,
            payment_details: self.payment_details,
            payor_signature: self.payor_signature,
            sequence: self.sequence,
            signature_data: self.signature_data,
            wallet_type: self.wallet_type,
            stored_id: self.stored_id,
            stored_method_usage_type: self.stored_method_usage_type,
        })
    }
}
