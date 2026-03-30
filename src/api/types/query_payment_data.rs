pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryPaymentData {
    #[serde(rename = "AccountExp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_exp: Option<Accountexp>,
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountId>,
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

impl QueryPaymentData {
    pub fn builder() -> QueryPaymentDataBuilder {
        <QueryPaymentDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryPaymentDataBuilder {
    account_exp: Option<Accountexp>,
    account_id: Option<AccountId>,
    account_type: Option<Accounttype>,
    account_zip: Option<Accountzip>,
    bin_data: Option<BinData>,
    holder_name: Option<Holdername>,
    initiator: Option<Initiator>,
    masked_account: Option<Maskedaccount>,
    order_description: Option<Orderdescription>,
    payment_details: Option<PaymentDetail>,
    sequence: Option<Sequence>,
    signature_data: Option<Signaturedata>,
    stored_id: Option<Storedmethodid>,
    stored_method_usage_type: Option<StoredMethodUsageType>,
}

impl QueryPaymentDataBuilder {
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

    pub fn payment_details(mut self, value: PaymentDetail) -> Self {
        self.payment_details = Some(value);
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

    pub fn stored_id(mut self, value: Storedmethodid) -> Self {
        self.stored_id = Some(value);
        self
    }

    pub fn stored_method_usage_type(mut self, value: StoredMethodUsageType) -> Self {
        self.stored_method_usage_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryPaymentData`].
    pub fn build(self) -> Result<QueryPaymentData, BuildError> {
        Ok(QueryPaymentData {
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
            sequence: self.sequence,
            signature_data: self.signature_data,
            stored_id: self.stored_id,
            stored_method_usage_type: self.stored_method_usage_type,
        })
    }
}
