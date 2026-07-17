pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct QueryPayoutTransactionRecordsItemPaymentData {
    #[serde(rename = "MaskedAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masked_account: Option<Maskedaccount>,
    #[serde(rename = "AccountType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<Accounttype>,
    #[serde(rename = "AccountExp")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_exp: Option<Accountexp>,
    #[serde(rename = "AccountZip")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_zip: Option<Accountzip>,
    /// Card or bank account holder name.
    #[serde(rename = "HolderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder_name: Option<Holdername>,
    /// Identifier of stored payment method used in transaction.
    #[serde(rename = "StoredId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_id: Option<Storedmethodid>,
    #[serde(rename = "Initiator")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiator: Option<Initiator>,
    #[serde(rename = "StoredMethodUsageType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_method_usage_type: Option<StoredMethodUsageType>,
    #[serde(rename = "Sequence")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sequence: Option<Sequence>,
    #[serde(rename = "orderDescription")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_description: Option<Orderdescription>,
    #[serde(rename = "cloudSignatureData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_signature_data: Option<String>,
    #[serde(rename = "cloudSignatureFormat")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloud_signature_format: Option<String>,
    #[serde(rename = "paymentDetails")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_details: Option<PaymentDetail>,
    #[serde(rename = "payorData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payor_data: Option<String>,
    #[serde(rename = "accountId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountId>,
    #[serde(rename = "bankAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<String>,
    #[serde(rename = "gatewayConnector")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_connector: Option<PayoutGatewayConnector>,
    #[serde(rename = "binData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_data: Option<BinData>,
}

impl QueryPayoutTransactionRecordsItemPaymentData {
    pub fn builder() -> QueryPayoutTransactionRecordsItemPaymentDataBuilder {
        <QueryPayoutTransactionRecordsItemPaymentDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryPayoutTransactionRecordsItemPaymentDataBuilder {
    masked_account: Option<Maskedaccount>,
    account_type: Option<Accounttype>,
    account_exp: Option<Accountexp>,
    account_zip: Option<Accountzip>,
    holder_name: Option<Holdername>,
    stored_id: Option<Storedmethodid>,
    initiator: Option<Initiator>,
    stored_method_usage_type: Option<StoredMethodUsageType>,
    sequence: Option<Sequence>,
    order_description: Option<Orderdescription>,
    cloud_signature_data: Option<String>,
    cloud_signature_format: Option<String>,
    payment_details: Option<PaymentDetail>,
    payor_data: Option<String>,
    account_id: Option<AccountId>,
    bank_account: Option<String>,
    gateway_connector: Option<PayoutGatewayConnector>,
    bin_data: Option<BinData>,
}

impl QueryPayoutTransactionRecordsItemPaymentDataBuilder {
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

    pub fn account_zip(mut self, value: Accountzip) -> Self {
        self.account_zip = Some(value);
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

    pub fn cloud_signature_data(mut self, value: impl Into<String>) -> Self {
        self.cloud_signature_data = Some(value.into());
        self
    }

    pub fn cloud_signature_format(mut self, value: impl Into<String>) -> Self {
        self.cloud_signature_format = Some(value.into());
        self
    }

    pub fn payment_details(mut self, value: PaymentDetail) -> Self {
        self.payment_details = Some(value);
        self
    }

    pub fn payor_data(mut self, value: impl Into<String>) -> Self {
        self.payor_data = Some(value.into());
        self
    }

    pub fn account_id(mut self, value: AccountId) -> Self {
        self.account_id = Some(value);
        self
    }

    pub fn bank_account(mut self, value: impl Into<String>) -> Self {
        self.bank_account = Some(value.into());
        self
    }

    pub fn gateway_connector(mut self, value: PayoutGatewayConnector) -> Self {
        self.gateway_connector = Some(value);
        self
    }

    pub fn bin_data(mut self, value: BinData) -> Self {
        self.bin_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`QueryPayoutTransactionRecordsItemPaymentData`].
    pub fn build(self) -> Result<QueryPayoutTransactionRecordsItemPaymentData, BuildError> {
        Ok(QueryPayoutTransactionRecordsItemPaymentData {
            masked_account: self.masked_account,
            account_type: self.account_type,
            account_exp: self.account_exp,
            account_zip: self.account_zip,
            holder_name: self.holder_name,
            stored_id: self.stored_id,
            initiator: self.initiator,
            stored_method_usage_type: self.stored_method_usage_type,
            sequence: self.sequence,
            order_description: self.order_description,
            cloud_signature_data: self.cloud_signature_data,
            cloud_signature_format: self.cloud_signature_format,
            payment_details: self.payment_details,
            payor_data: self.payor_data,
            account_id: self.account_id,
            bank_account: self.bank_account,
            gateway_connector: self.gateway_connector,
            bin_data: self.bin_data,
        })
    }
}
