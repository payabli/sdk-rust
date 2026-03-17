pub use crate::prelude::*;

/// Response for the add payment method domain operation.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AddPaymentMethodDomainApiResponse {
    #[serde(rename = "isSuccess")]
    pub is_success: IsSuccess,
    pub pageidentifier: PageIdentifier,
    #[serde(rename = "responseData")]
    pub response_data: PaymentMethodDomainApiResponse,
    #[serde(rename = "responseText")]
    pub response_text: String,
}
