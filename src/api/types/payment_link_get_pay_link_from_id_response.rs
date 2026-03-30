pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetPayLinkFromIdResponse {
    #[serde(flatten)]
    pub payabli_api_response_generic_2_part_fields: PayabliApiResponseGeneric2Part,
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<GetPayLinkFromIdResponseResponseData>,
}

impl GetPayLinkFromIdResponse {
    pub fn builder() -> GetPayLinkFromIdResponseBuilder {
        <GetPayLinkFromIdResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetPayLinkFromIdResponseBuilder {
    payabli_api_response_generic_2_part_fields: Option<PayabliApiResponseGeneric2Part>,
    response_data: Option<GetPayLinkFromIdResponseResponseData>,
}

impl GetPayLinkFromIdResponseBuilder {
    pub fn payabli_api_response_generic_2_part_fields(
        mut self,
        value: PayabliApiResponseGeneric2Part,
    ) -> Self {
        self.payabli_api_response_generic_2_part_fields = Some(value);
        self
    }

    pub fn response_data(mut self, value: GetPayLinkFromIdResponseResponseData) -> Self {
        self.response_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetPayLinkFromIdResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`payabli_api_response_generic_2_part_fields`](GetPayLinkFromIdResponseBuilder::payabli_api_response_generic_2_part_fields)
    pub fn build(self) -> Result<GetPayLinkFromIdResponse, BuildError> {
        Ok(GetPayLinkFromIdResponse {
            payabli_api_response_generic_2_part_fields: self
                .payabli_api_response_generic_2_part_fields
                .ok_or_else(|| {
                    BuildError::missing_field("payabli_api_response_generic_2_part_fields")
                })?,
            response_data: self.response_data,
        })
    }
}
