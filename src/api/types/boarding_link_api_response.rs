pub use crate::prelude::*;

///
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BoardingLinkApiResponse {
    /// Reference name for boarding link (if responseText = Success) or
    /// List of empty fields separated by comma (if responseText = Fail)
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<String>,
    #[serde(rename = "responseText")]
    #[serde(default)]
    pub response_text: ResponseText,
}

impl BoardingLinkApiResponse {
    pub fn builder() -> BoardingLinkApiResponseBuilder {
        <BoardingLinkApiResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BoardingLinkApiResponseBuilder {
    response_data: Option<String>,
    response_text: Option<ResponseText>,
}

impl BoardingLinkApiResponseBuilder {
    pub fn response_data(mut self, value: impl Into<String>) -> Self {
        self.response_data = Some(value.into());
        self
    }

    pub fn response_text(mut self, value: ResponseText) -> Self {
        self.response_text = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BoardingLinkApiResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`response_text`](BoardingLinkApiResponseBuilder::response_text)
    pub fn build(self) -> Result<BoardingLinkApiResponse, BuildError> {
        Ok(BoardingLinkApiResponse {
            response_data: self.response_data,
            response_text: self
                .response_text
                .ok_or_else(|| BuildError::missing_field("response_text"))?,
        })
    }
}
