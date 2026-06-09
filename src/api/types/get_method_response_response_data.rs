pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetMethodResponseResponseData {
    /// Bank routing number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<String>,
    #[serde(rename = "achHolderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_holder_type: Option<AchHolderType>,
    #[serde(rename = "achSecCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_sec_code: Option<AchSecCode>,
    /// The bank identification number (BIN)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<String>,
    #[serde(rename = "binData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_data: Option<BinData>,
    /// Timestamp for when card was last updated
    #[serde(rename = "cardUpdatedOn")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub card_updated_on: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customers: Option<Vec<GetMethodResponseResponseDataCustomersItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<Descriptor>,
    /// Expiration date for card in stored method in format MM/YY
    #[serde(rename = "expDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_date: Option<String>,
    /// Account holder name in stored method
    #[serde(rename = "holderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder_name: Option<Holdername>,
    /// The stored payment method's identifier in Payabli
    #[serde(rename = "idPmethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_pmethod: Option<String>,
    /// Whether the ACH account has been validated
    #[serde(rename = "isValidatedACH")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_validated_ach: Option<bool>,
    /// Timestamp for last update of stored method, in UTC
    #[serde(rename = "lastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub last_updated: Option<DateTime<Utc>>,
    #[serde(rename = "maskedAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masked_account: Option<Maskedaccount>,
    /// The saved method's type: `card` or `ach`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// The payment method's token type
    #[serde(rename = "methodType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_type: Option<String>,
    /// The payment method postal code
    #[serde(rename = "postalCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendors: Option<Vec<GetMethodResponseResponseDataVendorsItem>>,
}

impl GetMethodResponseResponseData {
    pub fn builder() -> GetMethodResponseResponseDataBuilder {
        <GetMethodResponseResponseDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetMethodResponseResponseDataBuilder {
    aba: Option<String>,
    ach_holder_type: Option<AchHolderType>,
    ach_sec_code: Option<AchSecCode>,
    bin: Option<String>,
    bin_data: Option<BinData>,
    card_updated_on: Option<DateTime<Utc>>,
    customers: Option<Vec<GetMethodResponseResponseDataCustomersItem>>,
    descriptor: Option<Descriptor>,
    exp_date: Option<String>,
    holder_name: Option<Holdername>,
    id_pmethod: Option<String>,
    is_validated_ach: Option<bool>,
    last_updated: Option<DateTime<Utc>>,
    masked_account: Option<Maskedaccount>,
    method: Option<String>,
    method_type: Option<String>,
    postal_code: Option<String>,
    vendors: Option<Vec<GetMethodResponseResponseDataVendorsItem>>,
}

impl GetMethodResponseResponseDataBuilder {
    pub fn aba(mut self, value: impl Into<String>) -> Self {
        self.aba = Some(value.into());
        self
    }

    pub fn ach_holder_type(mut self, value: AchHolderType) -> Self {
        self.ach_holder_type = Some(value);
        self
    }

    pub fn ach_sec_code(mut self, value: AchSecCode) -> Self {
        self.ach_sec_code = Some(value);
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

    pub fn card_updated_on(mut self, value: DateTime<Utc>) -> Self {
        self.card_updated_on = Some(value);
        self
    }

    pub fn customers(mut self, value: Vec<GetMethodResponseResponseDataCustomersItem>) -> Self {
        self.customers = Some(value);
        self
    }

    pub fn descriptor(mut self, value: Descriptor) -> Self {
        self.descriptor = Some(value);
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

    pub fn id_pmethod(mut self, value: impl Into<String>) -> Self {
        self.id_pmethod = Some(value.into());
        self
    }

    pub fn is_validated_ach(mut self, value: bool) -> Self {
        self.is_validated_ach = Some(value);
        self
    }

    pub fn last_updated(mut self, value: DateTime<Utc>) -> Self {
        self.last_updated = Some(value);
        self
    }

    pub fn masked_account(mut self, value: Maskedaccount) -> Self {
        self.masked_account = Some(value);
        self
    }

    pub fn method(mut self, value: impl Into<String>) -> Self {
        self.method = Some(value.into());
        self
    }

    pub fn method_type(mut self, value: impl Into<String>) -> Self {
        self.method_type = Some(value.into());
        self
    }

    pub fn postal_code(mut self, value: impl Into<String>) -> Self {
        self.postal_code = Some(value.into());
        self
    }

    pub fn vendors(mut self, value: Vec<GetMethodResponseResponseDataVendorsItem>) -> Self {
        self.vendors = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetMethodResponseResponseData`].
    pub fn build(self) -> Result<GetMethodResponseResponseData, BuildError> {
        Ok(GetMethodResponseResponseData {
            aba: self.aba,
            ach_holder_type: self.ach_holder_type,
            ach_sec_code: self.ach_sec_code,
            bin: self.bin,
            bin_data: self.bin_data,
            card_updated_on: self.card_updated_on,
            customers: self.customers,
            descriptor: self.descriptor,
            exp_date: self.exp_date,
            holder_name: self.holder_name,
            id_pmethod: self.id_pmethod,
            is_validated_ach: self.is_validated_ach,
            last_updated: self.last_updated,
            masked_account: self.masked_account,
            method: self.method,
            method_type: self.method_type,
            postal_code: self.postal_code,
            vendors: self.vendors,
        })
    }
}
