pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CustomerQueryRecordsCustomerConsent {
    /// Describes the customer's email communications consent status.
    #[serde(rename = "eCommunication")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub e_communication: Option<CustomerQueryRecordsCustomerConsentECommunication>,
    /// Describes the customer's SMS communications consent status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sms: Option<CustomerQueryRecordsCustomerConsentSms>,
}
