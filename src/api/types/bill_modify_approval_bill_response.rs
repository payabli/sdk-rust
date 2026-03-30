pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ModifyApprovalBillResponse {
    #[serde(flatten)]
    pub payabli_api_response_generic_2_part_fields: PayabliApiResponseGeneric2Part,
    /// If `isSuccess` = true, this contains the bill identifier. If `isSuccess` = false, this contains the reason for the error.
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<i64>,
}

impl ModifyApprovalBillResponse {
    pub fn builder() -> ModifyApprovalBillResponseBuilder {
        <ModifyApprovalBillResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ModifyApprovalBillResponseBuilder {
    payabli_api_response_generic_2_part_fields: Option<PayabliApiResponseGeneric2Part>,
    response_data: Option<i64>,
}

impl ModifyApprovalBillResponseBuilder {
    pub fn payabli_api_response_generic_2_part_fields(
        mut self,
        value: PayabliApiResponseGeneric2Part,
    ) -> Self {
        self.payabli_api_response_generic_2_part_fields = Some(value);
        self
    }

    pub fn response_data(mut self, value: i64) -> Self {
        self.response_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ModifyApprovalBillResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`payabli_api_response_generic_2_part_fields`](ModifyApprovalBillResponseBuilder::payabli_api_response_generic_2_part_fields)
    pub fn build(self) -> Result<ModifyApprovalBillResponse, BuildError> {
        Ok(ModifyApprovalBillResponse {
            payabli_api_response_generic_2_part_fields: self
                .payabli_api_response_generic_2_part_fields
                .ok_or_else(|| {
                    BuildError::missing_field("payabli_api_response_generic_2_part_fields")
                })?,
            response_data: self.response_data,
        })
    }
}
