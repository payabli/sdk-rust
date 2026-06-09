pub use crate::prelude::*;

/// Payment data for an outbound transfer detail.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
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

impl TransferOutDetailPaymentData {
    pub fn builder() -> TransferOutDetailPaymentDataBuilder {
        <TransferOutDetailPaymentDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransferOutDetailPaymentDataBuilder {
    masked_account: Option<String>,
    account_type: Option<String>,
    account_exp: Option<String>,
    account_zip: Option<String>,
    holder_name: Option<String>,
    stored_id: Option<String>,
    initiator: Option<String>,
    stored_method_usage_type: Option<String>,
    sequence: Option<String>,
    order_description: Option<String>,
    cloud_signature_data: Option<String>,
    cloud_signature_format: Option<String>,
    payment_details: Option<serde_json::Value>,
    payor_data: Option<String>,
    account_id: Option<String>,
    bank_account: Option<String>,
    gateway_connector: Option<String>,
    bin_data: Option<serde_json::Value>,
}

impl TransferOutDetailPaymentDataBuilder {
    pub fn masked_account(mut self, value: impl Into<String>) -> Self {
        self.masked_account = Some(value.into());
        self
    }

    pub fn account_type(mut self, value: impl Into<String>) -> Self {
        self.account_type = Some(value.into());
        self
    }

    pub fn account_exp(mut self, value: impl Into<String>) -> Self {
        self.account_exp = Some(value.into());
        self
    }

    pub fn account_zip(mut self, value: impl Into<String>) -> Self {
        self.account_zip = Some(value.into());
        self
    }

    pub fn holder_name(mut self, value: impl Into<String>) -> Self {
        self.holder_name = Some(value.into());
        self
    }

    pub fn stored_id(mut self, value: impl Into<String>) -> Self {
        self.stored_id = Some(value.into());
        self
    }

    pub fn initiator(mut self, value: impl Into<String>) -> Self {
        self.initiator = Some(value.into());
        self
    }

    pub fn stored_method_usage_type(mut self, value: impl Into<String>) -> Self {
        self.stored_method_usage_type = Some(value.into());
        self
    }

    pub fn sequence(mut self, value: impl Into<String>) -> Self {
        self.sequence = Some(value.into());
        self
    }

    pub fn order_description(mut self, value: impl Into<String>) -> Self {
        self.order_description = Some(value.into());
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

    pub fn payment_details(mut self, value: serde_json::Value) -> Self {
        self.payment_details = Some(value);
        self
    }

    pub fn payor_data(mut self, value: impl Into<String>) -> Self {
        self.payor_data = Some(value.into());
        self
    }

    pub fn account_id(mut self, value: impl Into<String>) -> Self {
        self.account_id = Some(value.into());
        self
    }

    pub fn bank_account(mut self, value: impl Into<String>) -> Self {
        self.bank_account = Some(value.into());
        self
    }

    pub fn gateway_connector(mut self, value: impl Into<String>) -> Self {
        self.gateway_connector = Some(value.into());
        self
    }

    pub fn bin_data(mut self, value: serde_json::Value) -> Self {
        self.bin_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransferOutDetailPaymentData`].
    pub fn build(self) -> Result<TransferOutDetailPaymentData, BuildError> {
        Ok(TransferOutDetailPaymentData {
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
