pub use crate::prelude::*;

/// Query parameters for getExternalApplication
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct GetExternalApplicationQueryRequest {
    /// If `true`, sends an email that includes the link to the application to the `mail2` address. Defaults to `false`.
    #[serde(rename = "sendEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub send_email: Option<bool>,
}

impl GetExternalApplicationQueryRequest {
    pub fn builder() -> GetExternalApplicationQueryRequestBuilder {
        <GetExternalApplicationQueryRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct GetExternalApplicationQueryRequestBuilder {
    send_email: Option<bool>,
}

impl GetExternalApplicationQueryRequestBuilder {
    pub fn send_email(mut self, value: bool) -> Self {
        self.send_email = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`GetExternalApplicationQueryRequest`].
    pub fn build(self) -> Result<GetExternalApplicationQueryRequest, BuildError> {
        Ok(GetExternalApplicationQueryRequest {
            send_email: self.send_email,
        })
    }
}
