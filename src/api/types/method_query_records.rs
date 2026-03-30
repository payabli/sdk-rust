pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct MethodQueryRecords {
    /// The bank identification number (BIN). Null when method is ACH.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin: Option<String>,
    #[serde(rename = "binData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bin_data: Option<BinData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<Descriptor>,
    /// Expiration date associated to the method (only for card) in format MMYY.
    #[serde(rename = "expDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_date: Option<String>,
    #[serde(rename = "holderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub holder_name: Option<Holdername>,
    /// Method internal ID
    #[serde(rename = "idPmethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_pmethod: Option<String>,
    /// Date of last update
    #[serde(rename = "lastUpdated")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<LastModified>,
    #[serde(rename = "maskedAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub masked_account: Option<Maskedaccount>,
    /// Type of payment vehicle: **ach** or **card**
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
}

impl MethodQueryRecords {
    pub fn builder() -> MethodQueryRecordsBuilder {
        <MethodQueryRecordsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct MethodQueryRecordsBuilder {
    bin: Option<String>,
    bin_data: Option<BinData>,
    descriptor: Option<Descriptor>,
    exp_date: Option<String>,
    holder_name: Option<Holdername>,
    id_pmethod: Option<String>,
    last_updated: Option<LastModified>,
    masked_account: Option<Maskedaccount>,
    method: Option<String>,
}

impl MethodQueryRecordsBuilder {
    pub fn bin(mut self, value: impl Into<String>) -> Self {
        self.bin = Some(value.into());
        self
    }

    pub fn bin_data(mut self, value: BinData) -> Self {
        self.bin_data = Some(value);
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

    pub fn last_updated(mut self, value: LastModified) -> Self {
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

    /// Consumes the builder and constructs a [`MethodQueryRecords`].
    pub fn build(self) -> Result<MethodQueryRecords, BuildError> {
        Ok(MethodQueryRecords {
            bin: self.bin,
            bin_data: self.bin_data,
            descriptor: self.descriptor,
            exp_date: self.exp_date,
            holder_name: self.holder_name,
            id_pmethod: self.id_pmethod,
            last_updated: self.last_updated,
            masked_account: self.masked_account,
            method: self.method,
        })
    }
}
