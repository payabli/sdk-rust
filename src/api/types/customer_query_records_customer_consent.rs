pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CustomerQueryRecordsCustomerConsent {
    /// Describes the customer's email communications consent status.
    #[serde(rename = "eCommunication")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_communication: Option<CustomerQueryRecordsCustomerConsentECommunication>,
    /// Describes the customer's SMS communications consent status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms: Option<CustomerQueryRecordsCustomerConsentSms>,
}

impl CustomerQueryRecordsCustomerConsent {
    pub fn builder() -> CustomerQueryRecordsCustomerConsentBuilder {
        <CustomerQueryRecordsCustomerConsentBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CustomerQueryRecordsCustomerConsentBuilder {
    e_communication: Option<CustomerQueryRecordsCustomerConsentECommunication>,
    sms: Option<CustomerQueryRecordsCustomerConsentSms>,
}

impl CustomerQueryRecordsCustomerConsentBuilder {
    pub fn e_communication(
        mut self,
        value: CustomerQueryRecordsCustomerConsentECommunication,
    ) -> Self {
        self.e_communication = Some(value);
        self
    }

    pub fn sms(mut self, value: CustomerQueryRecordsCustomerConsentSms) -> Self {
        self.sms = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CustomerQueryRecordsCustomerConsent`].
    pub fn build(self) -> Result<CustomerQueryRecordsCustomerConsent, BuildError> {
        Ok(CustomerQueryRecordsCustomerConsent {
            e_communication: self.e_communication,
            sms: self.sms,
        })
    }
}
