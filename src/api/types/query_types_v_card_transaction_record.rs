pub use crate::prelude::*;

/// A virtual card transaction record returned by the query.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VCardTransactionRecord {
    /// Unique identifier for the transaction.
    #[serde(rename = "Identifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifier: Option<String>,
    /// Token of the virtual card associated with the transaction.
    #[serde(rename = "CardToken")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_token: Option<String>,
    /// Last four digits of the masked virtual card number.
    #[serde(rename = "LastFour")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_four: Option<String>,
    /// Expiration date of the virtual card used for the transaction.
    #[serde(rename = "ExpirationDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_date: Option<String>,
    #[serde(rename = "Mcc")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mcc: Option<Mcc>,
    /// Identifier of the payout linked to this transaction.
    #[serde(rename = "PayoutId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout_id: Option<i64>,
    /// Identifier of the customer linked to this transaction.
    #[serde(rename = "CustomerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<i64>,
    /// Identifier of the vendor linked to this transaction.
    #[serde(rename = "VendorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<i64>,
    /// Custom field 1 from the virtual card.
    #[serde(rename = "MiscData1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub misc_data_1: Option<String>,
    /// Custom field 2 from the virtual card.
    #[serde(rename = "MiscData2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub misc_data_2: Option<String>,
    /// Number of times the virtual card has been used.
    #[serde(rename = "CurrentUses")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_uses: Option<i64>,
    /// Authorized amount on the virtual card.
    #[serde(rename = "Amount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<f64>,
    /// Current balance remaining on the virtual card.
    #[serde(rename = "Balance")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<f64>,
    /// Numeric identifier of the paypoint that issued the virtual card.
    #[serde(rename = "PaypointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_id: Option<i64>,
    #[serde(rename = "PaypointLegal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legal: Option<Legalname>,
    #[serde(rename = "PaypointDba")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dba: Option<Dbaname>,
    #[serde(rename = "ExternalPaypointID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<ExternalPaypointId>,
    #[serde(rename = "OrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_name: Option<OrgParentName>,
    /// Transaction type, such as `AUTHORIZATION`.
    #[serde(rename = "Type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<String>,
    /// Transaction status, such as `AUTHORIZATION`.
    #[serde(rename = "Status")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Date and time the transaction was created. Format: `YYYY-MM-DD HH:MM:SS.ffffff`.
    #[serde(rename = "CreatedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_on: Option<String>,
    /// Amount of the transaction, as a string value.
    #[serde(rename = "TransactionAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_amount: Option<String>,
    /// Posted amount of the transaction, as a string value.
    #[serde(rename = "PostedAmount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posted_amount: Option<String>,
    /// Date and time the transaction was posted, in format `YYYY-MM-DD HH:MM:SS.ffffff`. Null when the transaction hasn't posted yet.
    #[serde(rename = "PostedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posted_on: Option<String>,
    /// Name of the merchant where the virtual card was used.
    #[serde(rename = "MerchantName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_name: Option<String>,
    /// Authorization status of the transaction.
    #[serde(rename = "AuthorizationStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_status: Option<String>,
    /// Reason the transaction was declined, when applicable.
    #[serde(rename = "ReasonToDecline")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_to_decline: Option<String>,
}

impl VCardTransactionRecord {
    pub fn builder() -> VCardTransactionRecordBuilder {
        <VCardTransactionRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VCardTransactionRecordBuilder {
    identifier: Option<String>,
    card_token: Option<String>,
    last_four: Option<String>,
    expiration_date: Option<String>,
    mcc: Option<Mcc>,
    payout_id: Option<i64>,
    customer_id: Option<i64>,
    vendor_id: Option<i64>,
    misc_data_1: Option<String>,
    misc_data_2: Option<String>,
    current_uses: Option<i64>,
    amount: Option<f64>,
    balance: Option<f64>,
    paypoint_id: Option<i64>,
    paypoint_legal: Option<Legalname>,
    paypoint_dba: Option<Dbaname>,
    external_paypoint_id: Option<ExternalPaypointId>,
    org_name: Option<OrgParentName>,
    r#type: Option<String>,
    status: Option<String>,
    created_on: Option<String>,
    transaction_amount: Option<String>,
    posted_amount: Option<String>,
    posted_on: Option<String>,
    merchant_name: Option<String>,
    authorization_status: Option<String>,
    reason_to_decline: Option<String>,
}

impl VCardTransactionRecordBuilder {
    pub fn identifier(mut self, value: impl Into<String>) -> Self {
        self.identifier = Some(value.into());
        self
    }

    pub fn card_token(mut self, value: impl Into<String>) -> Self {
        self.card_token = Some(value.into());
        self
    }

    pub fn last_four(mut self, value: impl Into<String>) -> Self {
        self.last_four = Some(value.into());
        self
    }

    pub fn expiration_date(mut self, value: impl Into<String>) -> Self {
        self.expiration_date = Some(value.into());
        self
    }

    pub fn mcc(mut self, value: Mcc) -> Self {
        self.mcc = Some(value);
        self
    }

    pub fn payout_id(mut self, value: i64) -> Self {
        self.payout_id = Some(value);
        self
    }

    pub fn customer_id(mut self, value: i64) -> Self {
        self.customer_id = Some(value);
        self
    }

    pub fn vendor_id(mut self, value: i64) -> Self {
        self.vendor_id = Some(value);
        self
    }

    pub fn misc_data_1(mut self, value: impl Into<String>) -> Self {
        self.misc_data_1 = Some(value.into());
        self
    }

    pub fn misc_data_2(mut self, value: impl Into<String>) -> Self {
        self.misc_data_2 = Some(value.into());
        self
    }

    pub fn current_uses(mut self, value: i64) -> Self {
        self.current_uses = Some(value);
        self
    }

    pub fn amount(mut self, value: f64) -> Self {
        self.amount = Some(value);
        self
    }

    pub fn balance(mut self, value: f64) -> Self {
        self.balance = Some(value);
        self
    }

    pub fn paypoint_id(mut self, value: i64) -> Self {
        self.paypoint_id = Some(value);
        self
    }

    pub fn paypoint_legal(mut self, value: Legalname) -> Self {
        self.paypoint_legal = Some(value);
        self
    }

    pub fn paypoint_dba(mut self, value: Dbaname) -> Self {
        self.paypoint_dba = Some(value);
        self
    }

    pub fn external_paypoint_id(mut self, value: ExternalPaypointId) -> Self {
        self.external_paypoint_id = Some(value);
        self
    }

    pub fn org_name(mut self, value: OrgParentName) -> Self {
        self.org_name = Some(value);
        self
    }

    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

    pub fn status(mut self, value: impl Into<String>) -> Self {
        self.status = Some(value.into());
        self
    }

    pub fn created_on(mut self, value: impl Into<String>) -> Self {
        self.created_on = Some(value.into());
        self
    }

    pub fn transaction_amount(mut self, value: impl Into<String>) -> Self {
        self.transaction_amount = Some(value.into());
        self
    }

    pub fn posted_amount(mut self, value: impl Into<String>) -> Self {
        self.posted_amount = Some(value.into());
        self
    }

    pub fn posted_on(mut self, value: impl Into<String>) -> Self {
        self.posted_on = Some(value.into());
        self
    }

    pub fn merchant_name(mut self, value: impl Into<String>) -> Self {
        self.merchant_name = Some(value.into());
        self
    }

    pub fn authorization_status(mut self, value: impl Into<String>) -> Self {
        self.authorization_status = Some(value.into());
        self
    }

    pub fn reason_to_decline(mut self, value: impl Into<String>) -> Self {
        self.reason_to_decline = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VCardTransactionRecord`].
    pub fn build(self) -> Result<VCardTransactionRecord, BuildError> {
        Ok(VCardTransactionRecord {
            identifier: self.identifier,
            card_token: self.card_token,
            last_four: self.last_four,
            expiration_date: self.expiration_date,
            mcc: self.mcc,
            payout_id: self.payout_id,
            customer_id: self.customer_id,
            vendor_id: self.vendor_id,
            misc_data_1: self.misc_data_1,
            misc_data_2: self.misc_data_2,
            current_uses: self.current_uses,
            amount: self.amount,
            balance: self.balance,
            paypoint_id: self.paypoint_id,
            paypoint_legal: self.paypoint_legal,
            paypoint_dba: self.paypoint_dba,
            external_paypoint_id: self.external_paypoint_id,
            org_name: self.org_name,
            r#type: self.r#type,
            status: self.status,
            created_on: self.created_on,
            transaction_amount: self.transaction_amount,
            posted_amount: self.posted_amount,
            posted_on: self.posted_on,
            merchant_name: self.merchant_name,
            authorization_status: self.authorization_status,
            reason_to_decline: self.reason_to_decline,
        })
    }
}
