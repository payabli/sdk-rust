pub use crate::prelude::*;

/// Payment method and transaction details
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct TransactionDetailPaymentData {
    #[serde(rename = "maskedAccount")]
    #[serde(default)]
    pub masked_account: Maskedaccount,
    #[serde(rename = "accountType")]
    #[serde(default)]
    pub account_type: Accounttype,
    #[serde(rename = "accountExp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_exp: Option<Accountexp>,
    #[serde(rename = "holderName")]
    #[serde(default)]
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
    #[serde(default)]
    pub order_description: Orderdescription,
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountId>,
    #[serde(rename = "signatureData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature_data: Option<Signaturedata>,
    #[serde(rename = "binData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_data: Option<BinData>,
    #[serde(rename = "paymentDetails")]
    #[serde(default)]
    pub payment_details: TransactionDetailPaymentDetails,
}

impl TransactionDetailPaymentData {
    pub fn builder() -> TransactionDetailPaymentDataBuilder {
        <TransactionDetailPaymentDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransactionDetailPaymentDataBuilder {
    masked_account: Option<Maskedaccount>,
    account_type: Option<Accounttype>,
    account_exp: Option<Accountexp>,
    holder_name: Option<Holdername>,
    stored_id: Option<Storedmethodid>,
    initiator: Option<Initiator>,
    stored_method_usage_type: Option<StoredMethodUsageType>,
    sequence: Option<Sequence>,
    order_description: Option<Orderdescription>,
    account_id: Option<AccountId>,
    signature_data: Option<Signaturedata>,
    bin_data: Option<BinData>,
    payment_details: Option<TransactionDetailPaymentDetails>,
}

impl TransactionDetailPaymentDataBuilder {
    pub fn masked_account(mut self, value: Maskedaccount) -> Self {
        self.masked_account = Some(value);
        self
    }

    pub fn account_type(mut self, value: Accounttype) -> Self {
        self.account_type = Some(value);
        self
    }

    pub fn account_exp(mut self, value: Accountexp) -> Self {
        self.account_exp = Some(value);
        self
    }

    pub fn holder_name(mut self, value: Holdername) -> Self {
        self.holder_name = Some(value);
        self
    }

    pub fn stored_id(mut self, value: Storedmethodid) -> Self {
        self.stored_id = Some(value);
        self
    }

    pub fn initiator(mut self, value: Initiator) -> Self {
        self.initiator = Some(value);
        self
    }

    pub fn stored_method_usage_type(mut self, value: StoredMethodUsageType) -> Self {
        self.stored_method_usage_type = Some(value);
        self
    }

    pub fn sequence(mut self, value: Sequence) -> Self {
        self.sequence = Some(value);
        self
    }

    pub fn order_description(mut self, value: Orderdescription) -> Self {
        self.order_description = Some(value);
        self
    }

    pub fn account_id(mut self, value: AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn signature_data(mut self, value: Signaturedata) -> Self {
        self.signature_data = Some(value);
        self
    }

    pub fn bin_data(mut self, value: BinData) -> Self {
        self.bin_data = Some(value);
        self
    }

    pub fn payment_details(mut self, value: TransactionDetailPaymentDetails) -> Self {
        self.payment_details = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransactionDetailPaymentData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`masked_account`](TransactionDetailPaymentDataBuilder::masked_account)
    /// - [`account_type`](TransactionDetailPaymentDataBuilder::account_type)
    /// - [`holder_name`](TransactionDetailPaymentDataBuilder::holder_name)
    /// - [`order_description`](TransactionDetailPaymentDataBuilder::order_description)
    /// - [`payment_details`](TransactionDetailPaymentDataBuilder::payment_details)
    pub fn build(self) -> Result<TransactionDetailPaymentData, BuildError> {
        Ok(TransactionDetailPaymentData {
            masked_account: self
                .masked_account
                .ok_or_else(|| BuildError::missing_field("masked_account"))?,
            account_type: self
                .account_type
                .ok_or_else(|| BuildError::missing_field("account_type"))?,
            account_exp: self.account_exp,
            holder_name: self
                .holder_name
                .ok_or_else(|| BuildError::missing_field("holder_name"))?,
            stored_id: self.stored_id,
            initiator: self.initiator,
            stored_method_usage_type: self.stored_method_usage_type,
            sequence: self.sequence,
            order_description: self
                .order_description
                .ok_or_else(|| BuildError::missing_field("order_description"))?,
            account_id: self.account_id,
            signature_data: self.signature_data,
            bin_data: self.bin_data,
            payment_details: self
                .payment_details
                .ok_or_else(|| BuildError::missing_field("payment_details"))?,
        })
    }
}
