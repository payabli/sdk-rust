pub use crate::prelude::*;

/// Response to a Tap to Pay activation code request. On success,
/// `responseData` carries the activation code, its expiration, and
/// whether an existing code was reused.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TapToPayActivationChallengeResponse {
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
    #[serde(default)]
    pub response_data: TapToPayActivationChallengeData,
}

impl TapToPayActivationChallengeResponse {
    pub fn builder() -> TapToPayActivationChallengeResponseBuilder {
        <TapToPayActivationChallengeResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TapToPayActivationChallengeResponseBuilder {
    response_code: Option<Responsecode>,
    page_identifier: Option<PageIdentifier>,
    room_id: Option<RoomIdNotInUse>,
    is_success: Option<IsSuccess>,
    response_text: Option<ResponseText>,
    response_data: Option<TapToPayActivationChallengeData>,
}

impl TapToPayActivationChallengeResponseBuilder {
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

    pub fn response_data(mut self, value: TapToPayActivationChallengeData) -> Self {
        self.response_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TapToPayActivationChallengeResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](TapToPayActivationChallengeResponseBuilder::response_text)
    /// - [`response_data`](TapToPayActivationChallengeResponseBuilder::response_data)
    pub fn build(self) -> Result<TapToPayActivationChallengeResponse, BuildError> {
        Ok(TapToPayActivationChallengeResponse {
            response_code: self.response_code,
            page_identifier: self.page_identifier,
            room_id: self.room_id,
            is_success: self.is_success,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
            response_data: self
                .response_data
                .ok_or_else(|| BuildError::missing_field("response_data"))?,
        })
    }
}
