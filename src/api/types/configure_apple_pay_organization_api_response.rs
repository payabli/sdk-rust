pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ConfigureApplePayOrganizationApiResponse {
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
    #[serde(rename = "responseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<Responsecode>,
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<ApplePayOrganizationUpdateData>,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
}

impl ConfigureApplePayOrganizationApiResponse {
    pub fn builder() -> ConfigureApplePayOrganizationApiResponseBuilder {
        <ConfigureApplePayOrganizationApiResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ConfigureApplePayOrganizationApiResponseBuilder {
    is_success: Option<IsSuccess>,
    page_identifier: Option<PageIdentifier>,
    response_code: Option<Responsecode>,
    response_data: Option<ApplePayOrganizationUpdateData>,
    response_text: Option<ResponseText>,
}

impl ConfigureApplePayOrganizationApiResponseBuilder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn page_identifier(mut self, value: PageIdentifier) -> Self {
        self.page_identifier = Some(value);
        self
    }

    pub fn response_code(mut self, value: Responsecode) -> Self {
        self.response_code = Some(value);
        self
    }

    pub fn response_data(mut self, value: ApplePayOrganizationUpdateData) -> Self {
        self.response_data = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ConfigureApplePayOrganizationApiResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](ConfigureApplePayOrganizationApiResponseBuilder::response_text)
    pub fn build(self) -> Result<ConfigureApplePayOrganizationApiResponse, BuildError> {
        Ok(ConfigureApplePayOrganizationApiResponse {
            is_success: self.is_success,
            page_identifier: self.page_identifier,
            response_code: self.response_code,
            response_data: self.response_data,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
        })
    }
}
