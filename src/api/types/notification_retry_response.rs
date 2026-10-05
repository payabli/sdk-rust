pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NotificationRetryResponse {
    /// A message describing the result of the retry.
    #[serde(default)]
    pub message: String,
}

impl NotificationRetryResponse {
    pub fn builder() -> NotificationRetryResponseBuilder {
        <NotificationRetryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotificationRetryResponseBuilder {
    message: Option<String>,
}

impl NotificationRetryResponseBuilder {
    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`NotificationRetryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`message`](NotificationRetryResponseBuilder::message)
    pub fn build(self) -> Result<NotificationRetryResponse, BuildError> {
        Ok(NotificationRetryResponse {
            message: self
                .message
                .ok_or_else(|| BuildError::missing_field("message"))?,
        })
    }
}
