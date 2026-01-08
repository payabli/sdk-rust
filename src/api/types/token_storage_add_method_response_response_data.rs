pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AddMethodResponseResponseData {
    /// Stored method identifier in Payabli platform. This ID is used to manage the stored method.
    #[serde(rename = "referenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference_id: Option<MethodReferenceId>,
    #[serde(rename = "resultCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_code: Option<ResultCode>,
    #[serde(rename = "resultText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_text: Option<Resulttext>,
    /// Internal unique ID of customer owner of the stored method.
    ///
    /// Returns `0` if the method wasn't assigned to an existing customer or no customer was created."
    #[serde(rename = "customerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_id: Option<CustomerId>,
    #[serde(rename = "methodReferenceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method_reference_id: Option<MethodReferenceId>,
}
