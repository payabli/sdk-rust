pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SettingsQueryRecord {
    /// Any custom fields defined for the org.
    #[serde(rename = "customFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<KeyValue>>,
    #[serde(rename = "forInvoices")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_invoices: Option<Vec<KeyValue>>,
    #[serde(rename = "forPayOuts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_pay_outs: Option<Vec<KeyValue>>,
    /// Information about digital wallet settings for the entity. Available values are `isApplePayEnabled` and `isGooglePayEnabled`.
    #[serde(rename = "forWallets")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub for_wallets: Option<Vec<KeyValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub general: Option<Vec<KeyValue>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identifiers: Option<Vec<KeyValue>>,
}

impl SettingsQueryRecord {
    pub fn builder() -> SettingsQueryRecordBuilder {
        <SettingsQueryRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SettingsQueryRecordBuilder {
    custom_fields: Option<Vec<KeyValue>>,
    for_invoices: Option<Vec<KeyValue>>,
    for_pay_outs: Option<Vec<KeyValue>>,
    for_wallets: Option<Vec<KeyValue>>,
    general: Option<Vec<KeyValue>>,
    identifiers: Option<Vec<KeyValue>>,
}

impl SettingsQueryRecordBuilder {
    pub fn custom_fields(mut self, value: Vec<KeyValue>) -> Self {
        self.custom_fields = Some(value);
        self
    }

    pub fn for_invoices(mut self, value: Vec<KeyValue>) -> Self {
        self.for_invoices = Some(value);
        self
    }

    pub fn for_pay_outs(mut self, value: Vec<KeyValue>) -> Self {
        self.for_pay_outs = Some(value);
        self
    }

    pub fn for_wallets(mut self, value: Vec<KeyValue>) -> Self {
        self.for_wallets = Some(value);
        self
    }

    pub fn general(mut self, value: Vec<KeyValue>) -> Self {
        self.general = Some(value);
        self
    }

    pub fn identifiers(mut self, value: Vec<KeyValue>) -> Self {
        self.identifiers = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SettingsQueryRecord`].
    pub fn build(self) -> Result<SettingsQueryRecord, BuildError> {
        Ok(SettingsQueryRecord {
            custom_fields: self.custom_fields,
            for_invoices: self.for_invoices,
            for_pay_outs: self.for_pay_outs,
            for_wallets: self.for_wallets,
            general: self.general,
            identifiers: self.identifiers,
        })
    }
}
