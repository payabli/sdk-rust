pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetBasicOrganizationResponse {
    #[serde(rename = "isSuccess")]
    #[serde(default)]
    pub is_success: IsSuccess,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageidentifier: Option<PageIdentifier>,
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<OrganizationQueryRecord>,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
}

impl GetBasicOrganizationResponse {
    pub fn builder() -> GetBasicOrganizationResponseBuilder {
        <GetBasicOrganizationResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetBasicOrganizationResponseBuilder {
    is_success: Option<IsSuccess>,
    pageidentifier: Option<PageIdentifier>,
    response_data: Option<OrganizationQueryRecord>,
    response_text: Option<ResponseText>,
}

impl GetBasicOrganizationResponseBuilder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn pageidentifier(mut self, value: PageIdentifier) -> Self {
        self.pageidentifier = Some(value);
        self
    }

    pub fn response_data(mut self, value: OrganizationQueryRecord) -> Self {
        self.response_data = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetBasicOrganizationResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_success`](GetBasicOrganizationResponseBuilder::is_success)
    /// - [`response_text`](GetBasicOrganizationResponseBuilder::response_text)
    pub fn build(self) -> Result<GetBasicOrganizationResponse, BuildError> {
        Ok(GetBasicOrganizationResponse {
            is_success: self
                .is_success
                .ok_or_else(|| BuildError::missing_field("is_success"))?,
            pageidentifier: self.pageidentifier,
            response_data: self.response_data,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
        })
    }
}
