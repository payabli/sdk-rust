pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct OcrVendorBillingData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,
    #[serde(rename = "bankName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(rename = "routingAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_account: Option<String>,
    #[serde(rename = "accountNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    #[serde(rename = "typeAccount")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_account: Option<String>,
    #[serde(rename = "bankAccountHolderName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_holder_name: Option<String>,
    #[serde(rename = "bankAccountHolderType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_holder_type: Option<String>,
    #[serde(rename = "bankAccountFunction")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account_function: Option<i64>,
}

impl OcrVendorBillingData {
    pub fn builder() -> OcrVendorBillingDataBuilder {
        <OcrVendorBillingDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OcrVendorBillingDataBuilder {
    id: Option<i64>,
    bank_name: Option<String>,
    routing_account: Option<String>,
    account_number: Option<String>,
    type_account: Option<String>,
    bank_account_holder_name: Option<String>,
    bank_account_holder_type: Option<String>,
    bank_account_function: Option<i64>,
}

impl OcrVendorBillingDataBuilder {
    pub fn id(mut self, value: i64) -> Self {
        self.id = Some(value);
        self
    }

    pub fn bank_name(mut self, value: impl Into<String>) -> Self {
        self.bank_name = Some(value.into());
        self
    }

    pub fn routing_account(mut self, value: impl Into<String>) -> Self {
        self.routing_account = Some(value.into());
        self
    }

    pub fn account_number(mut self, value: impl Into<String>) -> Self {
        self.account_number = Some(value.into());
        self
    }

    pub fn type_account(mut self, value: impl Into<String>) -> Self {
        self.type_account = Some(value.into());
        self
    }

    pub fn bank_account_holder_name(mut self, value: impl Into<String>) -> Self {
        self.bank_account_holder_name = Some(value.into());
        self
    }

    pub fn bank_account_holder_type(mut self, value: impl Into<String>) -> Self {
        self.bank_account_holder_type = Some(value.into());
        self
    }

    pub fn bank_account_function(mut self, value: i64) -> Self {
        self.bank_account_function = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OcrVendorBillingData`].
    pub fn build(self) -> Result<OcrVendorBillingData, BuildError> {
        Ok(OcrVendorBillingData {
            id: self.id,
            bank_name: self.bank_name,
            routing_account: self.routing_account,
            account_number: self.account_number,
            type_account: self.type_account,
            bank_account_holder_name: self.bank_account_holder_name,
            bank_account_holder_type: self.bank_account_holder_type,
            bank_account_function: self.bank_account_function,
        })
    }
}
