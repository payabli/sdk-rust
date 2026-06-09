pub use crate::prelude::*;

/// Response schema for invoice operations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InvoiceResponseWithoutData {
    #[serde(rename = "isSuccess")]
    #[serde(default)]
    pub is_success: IsSuccess,
    #[serde(rename = "responseCode")]
    #[serde(default)]
    pub response_code: Responsecode,
    /// If `isSuccess` = true, this contains the identifier of the invoice. If `isSuccess` = false, this contains the reason for the failure.
    #[serde(rename = "responseData")]
    pub response_data: Responsedatanonobject,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pageidentifier: Option<PageIdentifier>,
    #[serde(rename = "roomId")]
    #[serde(default)]
    pub room_id: RoomIdNotInUse,
}

impl InvoiceResponseWithoutData {
    pub fn builder() -> InvoiceResponseWithoutDataBuilder {
        <InvoiceResponseWithoutDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct InvoiceResponseWithoutDataBuilder {
    is_success: Option<IsSuccess>,
    response_code: Option<Responsecode>,
    response_data: Option<Responsedatanonobject>,
    response_text: Option<ResponseText>,
    pageidentifier: Option<PageIdentifier>,
    room_id: Option<RoomIdNotInUse>,
}

impl InvoiceResponseWithoutDataBuilder {
    pub fn is_success(mut self, value: IsSuccess) -> Self {
        self.is_success = Some(value);
        self
    }

    pub fn response_code(mut self, value: Responsecode) -> Self {
        self.response_code = Some(value);
        self
    }

    pub fn response_data(mut self, value: Responsedatanonobject) -> Self {
        self.response_data = Some(value);
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    pub fn pageidentifier(mut self, value: PageIdentifier) -> Self {
        self.pageidentifier = Some(value);
        self
    }

    pub fn room_id(mut self, value: RoomIdNotInUse) -> Self {
        self.room_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`InvoiceResponseWithoutData`].
    /// This method will fail if any of the following fields are not set:
    /// - [`is_success`](InvoiceResponseWithoutDataBuilder::is_success)
    /// - [`response_code`](InvoiceResponseWithoutDataBuilder::response_code)
    /// - [`response_data`](InvoiceResponseWithoutDataBuilder::response_data)
    /// - [`response_text`](InvoiceResponseWithoutDataBuilder::response_text)
    /// - [`room_id`](InvoiceResponseWithoutDataBuilder::room_id)
    pub fn build(self) -> Result<InvoiceResponseWithoutData, BuildError> {
        Ok(InvoiceResponseWithoutData {
            is_success: self
                .is_success
                .ok_or_else(|| BuildError::missing_field("is_success"))?,
            response_code: self
                .response_code
                .ok_or_else(|| BuildError::missing_field("response_code"))?,
            response_data: self
                .response_data
                .ok_or_else(|| BuildError::missing_field("response_data"))?,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
            pageidentifier: self.pageidentifier,
            room_id: self
                .room_id
                .ok_or_else(|| BuildError::missing_field("room_id"))?,
        })
    }
}
