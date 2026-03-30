pub use crate::prelude::*;

/// Describes the customer's SMS communications consent status.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CustomerQueryRecordsCustomerConsentSms {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<OptinStatus>,
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<LastModified>,
}

impl CustomerQueryRecordsCustomerConsentSms {
    pub fn builder() -> CustomerQueryRecordsCustomerConsentSmsBuilder {
        <CustomerQueryRecordsCustomerConsentSmsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomerQueryRecordsCustomerConsentSmsBuilder {
    status: Option<OptinStatus>,
    updated_at: Option<LastModified>,
}

impl CustomerQueryRecordsCustomerConsentSmsBuilder {
    pub fn status(mut self, value: OptinStatus) -> Self {
        self.status = Some(value);
        self
    }

    pub fn updated_at(mut self, value: LastModified) -> Self {
        self.updated_at = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CustomerQueryRecordsCustomerConsentSms`].
    pub fn build(self) -> Result<CustomerQueryRecordsCustomerConsentSms, BuildError> {
        Ok(CustomerQueryRecordsCustomerConsentSms {
            status: self.status,
            updated_at: self.updated_at,
        })
    }
}
