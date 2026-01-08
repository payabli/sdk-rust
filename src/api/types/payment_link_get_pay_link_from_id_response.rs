pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetPayLinkFromIdResponse {
    #[serde(flatten)]
    pub payabli_api_response_generic_2_part_fields: PayabliApiResponseGeneric2Part,
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<GetPayLinkFromIdResponseResponseData>,
}
