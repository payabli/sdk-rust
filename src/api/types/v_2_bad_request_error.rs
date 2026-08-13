pub use crate::prelude::*;

/// Bad request error response (HTTP 400) returned when request validation fails. Follows RFC 7807 Problem Details format with additional Payabli-specific fields.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct V2BadRequestError {
    /// A URI reference that identifies the problem type. Points to human-readable documentation for this error type.
    #[serde(default)]
    pub r#type: String,
    /// Always "Bad Request" for 400 errors.
    #[serde(default)]
    pub title: String,
    /// HTTP status code, always 400 for bad requests.
    #[serde(default)]
    pub status: i64,
    /// Short description of the error.
    #[serde(default)]
    pub detail: String,
    /// Request URL that caused the error.
    #[serde(default)]
    pub instance: String,
    /// Payabli's unified response code for validation errors. Starts with 'E'. See [Pay In unified response codes reference](/guides/pay-in-unified-response-codes-reference) for more information.
    #[serde(default)]
    pub code: String,
    /// Dictionary of field-specific validation errors. Keys are field paths (for example, "paymentMethod.cardnumber") and values are arrays of error details.
    #[serde(default)]
    pub errors: HashMap<String, Vec<V2BadRequestErrorDetail>>,
    /// Pagination token (equivalent to pageIdentifier in v1 APIs). Usually null for errors.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,
}

impl V2BadRequestError {
    pub fn builder() -> V2BadRequestErrorBuilder {
        <V2BadRequestErrorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct V2BadRequestErrorBuilder {
    r#type: Option<String>,
    title: Option<String>,
    status: Option<i64>,
    detail: Option<String>,
    instance: Option<String>,
    code: Option<String>,
    errors: Option<HashMap<String, Vec<V2BadRequestErrorDetail>>>,
    token: Option<String>,
}

impl V2BadRequestErrorBuilder {
    pub fn r#type(mut self, value: impl Into<String>) -> Self {
        self.r#type = Some(value.into());
        self
    }

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

    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn errors(mut self, value: HashMap<String, Vec<V2BadRequestErrorDetail>>) -> Self {
        self.errors = Some(value);
        self
    }

    pub fn token(mut self, value: impl Into<String>) -> Self {
        self.token = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`V2BadRequestError`].
    /// This method will fail if any of the following fields are not set:
    /// - [`r#type`](V2BadRequestErrorBuilder::r#type)
    /// - [`title`](V2BadRequestErrorBuilder::title)
    /// - [`status`](V2BadRequestErrorBuilder::status)
    /// - [`detail`](V2BadRequestErrorBuilder::detail)
    /// - [`instance`](V2BadRequestErrorBuilder::instance)
    /// - [`code`](V2BadRequestErrorBuilder::code)
    /// - [`errors`](V2BadRequestErrorBuilder::errors)
    pub fn build(self) -> Result<V2BadRequestError, BuildError> {
        Ok(V2BadRequestError {
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
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
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            errors: self
                .errors
                .ok_or_else(|| BuildError::missing_field("errors"))?,
            token: self.token,
        })
    }
}
