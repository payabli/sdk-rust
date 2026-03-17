pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PayabliApiResponseError400 {
    /// Boolean indicating whether the operation was successful. A `true` value indicates success. A `false` value indicates failure.
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageidentifier: Option<PageIdentifier>,
    /// A code that indicates the operation's failure reason. See [API Response Codes](https://docs.payabli.com/api-reference/api-responses) for a full reference.
    #[serde(rename = "responseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<i64>,
    /// Describes the reason for a failed operation and how to resolve it.
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<PayabliApiResponseError400ResponseData>,
    /// Response text for operation: 'Success' or 'Declined'.
    #[serde(rename = "responseText")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_text: Option<String>,
}
