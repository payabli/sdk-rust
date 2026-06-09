pub use crate::prelude::*;

/// Internal server error response (HTTP 500) returned when an unexpected error occurs. Follows RFC 7807 Problem Details format.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct V2InternalServerError {
    /// Always "Internal Server Error" for 500 errors.
    #[serde(default)]
    pub title: String,
    /// HTTP status code, always 500 for internal errors.
    #[serde(default)]
    pub status: i64,
    /// Additional details about the internal error.
    #[serde(default)]
    pub detail: String,
    /// Request URL that caused the error.
    #[serde(default)]
    pub instance: String,
}

impl V2InternalServerError {
    pub fn builder() -> V2InternalServerErrorBuilder {
        <V2InternalServerErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V2InternalServerErrorBuilder {
    title: Option<String>,
    status: Option<i64>,
    detail: Option<String>,
    instance: Option<String>,
}

impl V2InternalServerErrorBuilder {
    pub fn title(mut self, value: impl Into<String>) -> Self {
        self.title = Some(value.into());
        self
    }

    pub fn status(mut self, value: i64) -> Self {
        self.status = Some(value);
        self
    }

    pub fn detail(mut self, value: impl Into<String>) -> Self {
        self.detail = Some(value.into());
        self
    }

    pub fn instance(mut self, value: impl Into<String>) -> Self {
        self.instance = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`V2InternalServerError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`title`](V2InternalServerErrorBuilder::title)
    /// - [`status`](V2InternalServerErrorBuilder::status)
    /// - [`detail`](V2InternalServerErrorBuilder::detail)
    /// - [`instance`](V2InternalServerErrorBuilder::instance)
    pub fn build(self) -> Result<V2InternalServerError, BuildError> {
        Ok(V2InternalServerError {
            title: self
                .title
                .ok_or_else(|| BuildError::missing_field("title"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
            detail: self
                .detail
                .ok_or_else(|| BuildError::missing_field("detail"))?,
            instance: self
                .instance
                .ok_or_else(|| BuildError::missing_field("instance"))?,
        })
    }
}
