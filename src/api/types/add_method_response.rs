pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddMethodResponse {
    #[serde(flatten)]
    pub payabli_api_response_generic_2_part_fields: PayabliApiResponseGeneric2Part,
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<AddMethodResponseResponseData>,
}

impl AddMethodResponse {
    pub fn builder() -> AddMethodResponseBuilder {
        <AddMethodResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddMethodResponseBuilder {
    payabli_api_response_generic_2_part_fields: Option<PayabliApiResponseGeneric2Part>,
    response_data: Option<AddMethodResponseResponseData>,
}

impl AddMethodResponseBuilder {
    pub fn payabli_api_response_generic_2_part_fields(
        mut self,
        value: PayabliApiResponseGeneric2Part,
    ) -> Self {
        self.payabli_api_response_generic_2_part_fields = Some(value);
        self
    }

    pub fn response_data(mut self, value: AddMethodResponseResponseData) -> Self {
        self.response_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AddMethodResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`payabli_api_response_generic_2_part_fields`](AddMethodResponseBuilder::payabli_api_response_generic_2_part_fields)
    pub fn build(self) -> Result<AddMethodResponse, BuildError> {
        Ok(AddMethodResponse {
            payabli_api_response_generic_2_part_fields: self
                .payabli_api_response_generic_2_part_fields
                .ok_or_else(|| {
                    BuildError::missing_field("payabli_api_response_generic_2_part_fields")
                })?,
            response_data: self.response_data,
        })
    }
}
