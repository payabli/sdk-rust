pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MethodQueryRecords {
    /// Method internal ID
    #[serde(rename = "IdPmethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_pmethod: Option<String>,
    /// Type of payment vehicle: **ach** or **card**
    #[serde(rename = "Method")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    #[serde(rename = "Descriptor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<Descriptor>,
    #[serde(rename = "MaskedAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masked_account: Option<Maskedaccount>,
    /// Expiration date associated to the method (only for card) in format MMYY.
    #[serde(rename = "ExpDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_date: Option<String>,
    #[serde(rename = "HolderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder_name: Option<Holdername>,
    /// Standard Entry Class (SEC) code for the ACH transaction.
    #[serde(rename = "AchSecCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_sec_code: Option<String>,
    /// Bank accountholder type: `personal` or `business`.
    #[serde(rename = "AchHolderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_holder_type: Option<String>,
    /// Whether the ACH account has been validated.
    #[serde(rename = "IsValidatedACH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_validated_ach: Option<bool>,
    /// The bank identification number (BIN). Null when method is ACH.
    #[serde(rename = "BIN")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<String>,
    #[serde(rename = "binData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_data: Option<BinData>,
    /// Bank routing number.
    #[serde(rename = "ABA")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<String>,
    /// The payment method postal code.
    #[serde(rename = "PostalCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    /// The payment method's token type.
    #[serde(rename = "MethodType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_type: Option<String>,
    /// Digital wallet type if applicable.
    #[serde(rename = "WalletType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_type: Option<String>,
    /// Date of last update
    #[serde(rename = "LastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<LastModified>,
    /// Date and time the card was last updated.
    #[serde(rename = "CardUpdatedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_updated_on: Option<DateTime<Utc>>,
}

impl MethodQueryRecords {
    pub fn builder() -> MethodQueryRecordsBuilder {
        <MethodQueryRecordsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MethodQueryRecordsBuilder {
    id_pmethod: Option<String>,
    method: Option<String>,
    descriptor: Option<Descriptor>,
    masked_account: Option<Maskedaccount>,
    exp_date: Option<String>,
    holder_name: Option<Holdername>,
    ach_sec_code: Option<String>,
    ach_holder_type: Option<String>,
    is_validated_ach: Option<bool>,
    bin: Option<String>,
    bin_data: Option<BinData>,
    aba: Option<String>,
    postal_code: Option<String>,
    method_type: Option<String>,
    wallet_type: Option<String>,
    last_updated: Option<LastModified>,
    card_updated_on: Option<DateTime<Utc>>,
}

impl MethodQueryRecordsBuilder {
    pub fn id_pmethod(mut self, value: impl Into<String>) -> Self {
        self.id_pmethod = Some(value.into());
        self
    }

    pub fn method(mut self, value: impl Into<String>) -> Self {
        self.method = Some(value.into());
        self
    }

    pub fn descriptor(mut self, value: Descriptor) -> Self {
        self.descriptor = Some(value);
        self
    }

    pub fn masked_account(mut self, value: Maskedaccount) -> Self {
        self.masked_account = Some(value);
        self
    }

    pub fn exp_date(mut self, value: impl Into<String>) -> Self {
        self.exp_date = Some(value.into());
        self
    }

    pub fn holder_name(mut self, value: Holdername) -> Self {
        self.holder_name = Some(value);
        self
    }

    pub fn ach_sec_code(mut self, value: impl Into<String>) -> Self {
        self.ach_sec_code = Some(value.into());
        self
    }

    pub fn ach_holder_type(mut self, value: impl Into<String>) -> Self {
        self.ach_holder_type = Some(value.into());
        self
    }

    pub fn is_validated_ach(mut self, value: bool) -> Self {
        self.is_validated_ach = Some(value);
        self
    }

    pub fn bin(mut self, value: impl Into<String>) -> Self {
        self.bin = Some(value.into());
        self
    }

    pub fn bin_data(mut self, value: BinData) -> Self {
        self.bin_data = Some(value);
        self
    }

    pub fn aba(mut self, value: impl Into<String>) -> Self {
        self.aba = Some(value.into());
        self
    }

    pub fn postal_code(mut self, value: impl Into<String>) -> Self {
        self.postal_code = Some(value.into());
        self
    }

    pub fn method_type(mut self, value: impl Into<String>) -> Self {
        self.method_type = Some(value.into());
        self
    }

    pub fn wallet_type(mut self, value: impl Into<String>) -> Self {
        self.wallet_type = Some(value.into());
        self
    }

    pub fn last_updated(mut self, value: LastModified) -> Self {
        self.last_updated = Some(value);
        self
    }

    pub fn card_updated_on(mut self, value: DateTime<Utc>) -> Self {
        self.card_updated_on = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`MethodQueryRecords`].
    pub fn build(self) -> Result<MethodQueryRecords, BuildError> {
        Ok(MethodQueryRecords {
            id_pmethod: self.id_pmethod,
            method: self.method,
            descriptor: self.descriptor,
            masked_account: self.masked_account,
            exp_date: self.exp_date,
            holder_name: self.holder_name,
            ach_sec_code: self.ach_sec_code,
            ach_holder_type: self.ach_holder_type,
            is_validated_ach: self.is_validated_ach,
            bin: self.bin,
            bin_data: self.bin_data,
            aba: self.aba,
            postal_code: self.postal_code,
            method_type: self.method_type,
            wallet_type: self.wallet_type,
            last_updated: self.last_updated,
            card_updated_on: self.card_updated_on,
        })
    }
}
