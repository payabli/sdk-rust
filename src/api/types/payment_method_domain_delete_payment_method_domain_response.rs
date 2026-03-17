pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeletePaymentMethodDomainResponse {
    #[serde(rename = "isSuccess")]
    pub is_success: IsSuccess,
    #[serde(rename = "pageIdentifier")]
    pub page_identifier: PageIdentifier,
    /// The deleted domain's domain ID.
    #[serde(rename = "responseData")]
    pub response_data: Responsedatanonobject,
    #[serde(rename = "responseText")]
    pub response_text: ResponseText,
}
