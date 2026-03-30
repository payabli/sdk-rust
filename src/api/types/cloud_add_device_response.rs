pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AddDeviceResponse {
    #[serde(flatten)]
    pub payabli_api_response_generic_2_part_fields: PayabliApiResponseGeneric2Part,
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
    /// If `isSuccess` = true, this contains the device identifier.
    /// If `isSuccess` = false, this contains the reason for the error.
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<String>,
}

impl AddDeviceResponse {
    pub fn builder() -> AddDeviceResponseBuilder {
        <AddDeviceResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AddDeviceResponseBuilder {
    payabli_api_response_generic_2_part_fields: Option<PayabliApiResponseGeneric2Part>,
    page_identifier: Option<PageIdentifier>,
    response_data: Option<String>,
}

impl AddDeviceResponseBuilder {
    pub fn payabli_api_response_generic_2_part_fields(
        mut self,
        value: PayabliApiResponseGeneric2Part,
    ) -> Self {
        self.payabli_api_response_generic_2_part_fields = Some(value);
        self
    }

    pub fn page_identifier(mut self, value: PageIdentifier) -> Self {
        self.page_identifier = Some(value);
        self
    }

    pub fn response_data(mut self, value: impl Into<String>) -> Self {
        self.response_data = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AddDeviceResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`payabli_api_response_generic_2_part_fields`](AddDeviceResponseBuilder::payabli_api_response_generic_2_part_fields)
    pub fn build(self) -> Result<AddDeviceResponse, BuildError> {
        Ok(AddDeviceResponse {
            payabli_api_response_generic_2_part_fields: self
                .payabli_api_response_generic_2_part_fields
                .ok_or_else(|| {
                    BuildError::missing_field("payabli_api_response_generic_2_part_fields")
                })?,
            page_identifier: self.page_identifier,
            response_data: self.response_data,
        })
    }
}
