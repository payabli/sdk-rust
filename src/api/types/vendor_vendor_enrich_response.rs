pub use crate::prelude::*;

/// Response from the vendor enrichment endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct VendorEnrichResponse {
    #[serde(rename = "responseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<Responsecode>,
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
    #[serde(rename = "roomId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_id: Option<RoomIdNotInUse>,
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<VendorEnrichResponseData>,
}

impl VendorEnrichResponse {
    pub fn builder() -> VendorEnrichResponseBuilder {
        <VendorEnrichResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VendorEnrichResponseBuilder {
    response_code: Option<Responsecode>,
    page_identifier: Option<PageIdentifier>,
    room_id: Option<RoomIdNotInUse>,
    is_success: Option<IsSuccess>,
    response_text: Option<ResponseText>,
    response_data: Option<VendorEnrichResponseData>,
}

impl VendorEnrichResponseBuilder {
    pub fn response_code(mut self, value: Responsecode) -> Self {
        self.response_code = Some(value);
        self
    }

    pub fn page_identifier(mut self, value: PageIdentifier) -> Self {
        self.page_identifier = Some(value);
        self
    }

    pub fn room_id(mut self, value: RoomIdNotInUse) -> Self {
        self.room_id = Some(value);
        self
    }

    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    pub fn response_data(mut self, value: VendorEnrichResponseData) -> Self {
        self.response_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VendorEnrichResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](VendorEnrichResponseBuilder::response_text)
    pub fn build(self) -> Result<VendorEnrichResponse, BuildError> {
        Ok(VendorEnrichResponse {
            response_code: self.response_code,
            page_identifier: self.page_identifier,
            room_id: self.room_id,
            is_success: self.is_success,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
            response_data: self.response_data,
        })
    }
}
