use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("BadRequestError: Bad request - {{message}}")]
    BadRequestError {
        message: String,
        field: Option<String>,
        details: Option<String>,
    },
    #[error("ConflictError: Conflict - {{message}}")]
    ConflictError {
        message: String,
        conflict_type: Option<String>,
    },
    #[error("ForbiddenError: Access forbidden - {{message}}")]
    ForbiddenError {
        message: String,
        resource: Option<String>,
        required_permission: Option<String>,
    },
    #[error("InternalServerError: Internal server error - {{message}}")]
    InternalServerError {
        message: String,
        error_id: Option<String>,
    },
    #[error("ServiceUnavailableError: {{message}}")]
    ServiceUnavailableError { message: String },
    #[error("UnauthorizedError: Authentication failed - {{message}}")]
    UnauthorizedError {
        message: String,
        auth_type: Option<String>,
    },
    #[error("BadRequestAuthResponseErrorV2: Bad request - {{message}}")]
    BadRequestAuthResponseErrorV2 {
        message: String,
        field: Option<String>,
        details: Option<String>,
    },
    #[error("BadRequestCaptureResponseErrorV2: Bad request - {{message}}")]
    BadRequestCaptureResponseErrorV2 {
        message: String,
        field: Option<String>,
        details: Option<String>,
    },
    #[error("BadRequestRefundResponseErrorV2: Bad request - {{message}}")]
    BadRequestRefundResponseErrorV2 {
        message: String,
        field: Option<String>,
        details: Option<String>,
    },
    #[error("BadRequestVoidResponseErrorV2: Bad request - {{message}}")]
    BadRequestVoidResponseErrorV2 {
        message: String,
        field: Option<String>,
        details: Option<String>,
    },
    #[error("DeclinedCaptureResponseErrorV2: {{message}}")]
    DeclinedCaptureResponseErrorV2 { message: String },
    #[error("DeclinedAuthResponseErrorV2: {{message}}")]
    DeclinedAuthResponseErrorV2 { message: String },
    #[error("DeclinedRefundResponseErrorV2: {{message}}")]
    DeclinedRefundResponseErrorV2 { message: String },
    #[error("DeclinedVoidResponseErrorV2: {{message}}")]
    DeclinedVoidResponseErrorV2 { message: String },
    #[error("InternalServerResponseErrorV2: Internal server error - {{message}}")]
    InternalServerResponseErrorV2 {
        message: String,
        error_id: Option<String>,
    },
    #[error("InvalidTransStatusError: Bad request - {{message}}")]
    InvalidTransStatusError {
        message: String,
        field: Option<String>,
        details: Option<String>,
    },
    #[error("CaptureError: Bad request - {{message}}")]
    CaptureError {
        message: String,
        field: Option<String>,
        details: Option<String>,
    },
    #[error("HTTP error {status}: {message}")]
    Http { status: u16, message: String },
    #[error("Network error: {0}")]
    Network(reqwest::Error),
    #[error("Serialization error: {0}")]
    Serialization(serde_json::Error),
    #[error("Configuration error: {0}")]
    Configuration(String),
    #[error("Invalid header value")]
    InvalidHeader,
    #[error("Could not clone request for retry")]
    RequestClone,
    #[error("SSE stream terminated")]
    StreamTerminated,
    #[error("SSE parse error: {0}")]
    SseParseError(String),
}

impl ApiError {
    pub fn from_response(status_code: u16, body: Option<&str>) -> Self {
        match status_code {
            400 => {
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        let message = parsed
                            .get("message")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string())
                            .unwrap_or("Unknown error".to_string());
                        let error_type = parsed.get("error_type").and_then(|v| v.as_str());
                        return match error_type {
                            Some("BadRequestError") => Self::BadRequestError {
                                message: message,
                                field: parsed
                                    .get("field")
                                    .and_then(|v| v.as_str().map(|s| s.to_string())),
                                details: parsed
                                    .get("details")
                                    .and_then(|v| v.as_str().map(|s| s.to_string())),
                            },
                            Some("BadRequestAuthResponseErrorV2") => {
                                Self::BadRequestAuthResponseErrorV2 {
                                    message: message,
                                    field: parsed
                                        .get("field")
                                        .and_then(|v| v.as_str().map(|s| s.to_string())),
                                    details: parsed
                                        .get("details")
                                        .and_then(|v| v.as_str().map(|s| s.to_string())),
                                }
                            }
                            Some("BadRequestCaptureResponseErrorV2") => {
                                Self::BadRequestCaptureResponseErrorV2 {
                                    message: message,
                                    field: parsed
                                        .get("field")
                                        .and_then(|v| v.as_str().map(|s| s.to_string())),
                                    details: parsed
                                        .get("details")
                                        .and_then(|v| v.as_str().map(|s| s.to_string())),
                                }
                            }
                            Some("BadRequestRefundResponseErrorV2") => {
                                Self::BadRequestRefundResponseErrorV2 {
                                    message: message,
                                    field: parsed
                                        .get("field")
                                        .and_then(|v| v.as_str().map(|s| s.to_string())),
                                    details: parsed
                                        .get("details")
                                        .and_then(|v| v.as_str().map(|s| s.to_string())),
                                }
                            }
                            Some("BadRequestVoidResponseErrorV2") => {
                                Self::BadRequestVoidResponseErrorV2 {
                                    message: message,
                                    field: parsed
                                        .get("field")
                                        .and_then(|v| v.as_str().map(|s| s.to_string())),
                                    details: parsed
                                        .get("details")
                                        .and_then(|v| v.as_str().map(|s| s.to_string())),
                                }
                            }
                            Some("InvalidTransStatusError") => Self::InvalidTransStatusError {
                                message: message,
                                field: parsed
                                    .get("field")
                                    .and_then(|v| v.as_str().map(|s| s.to_string())),
                                details: parsed
                                    .get("details")
                                    .and_then(|v| v.as_str().map(|s| s.to_string())),
                            },
                            Some("CaptureError") => Self::CaptureError {
                                message: message,
                                field: parsed
                                    .get("field")
                                    .and_then(|v| v.as_str().map(|s| s.to_string())),
                                details: parsed
                                    .get("details")
                                    .and_then(|v| v.as_str().map(|s| s.to_string())),
                            },
                            _ => Self::BadRequestError {
                                message: message,
                                field: parsed
                                    .get("field")
                                    .and_then(|v| v.as_str().map(|s| s.to_string())),
                                details: parsed
                                    .get("details")
                                    .and_then(|v| v.as_str().map(|s| s.to_string())),
                            },
                        };
                    }
                    return Self::BadRequestError {
                        message: body.unwrap_or("Unknown error").to_string(),
                        field: None,
                        details: None,
                    };
                }
                return Self::BadRequestError {
                    message: "Unknown error".to_string(),
                    field: None,
                    details: None,
                };
            }
            409 => {
                // Parse error body for ConflictError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::ConflictError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            conflict_type: parsed
                                .get("conflict_type")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::ConflictError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    conflict_type: None,
                };
            }
            403 => {
                // Parse error body for ForbiddenError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::ForbiddenError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            resource: parsed
                                .get("resource")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                            required_permission: parsed
                                .get("required_permission")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::ForbiddenError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    resource: None,
                    required_permission: None,
                };
            }
            500 => {
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        let message = parsed
                            .get("message")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string())
                            .unwrap_or("Unknown error".to_string());
                        let error_type = parsed.get("error_type").and_then(|v| v.as_str());
                        return match error_type {
                            Some("InternalServerError") => Self::InternalServerError {
                                message: message,
                                error_id: parsed
                                    .get("error_id")
                                    .and_then(|v| v.as_str().map(|s| s.to_string())),
                            },
                            Some("InternalServerResponseErrorV2") => {
                                Self::InternalServerResponseErrorV2 {
                                    message: message,
                                    error_id: parsed
                                        .get("error_id")
                                        .and_then(|v| v.as_str().map(|s| s.to_string())),
                                }
                            }
                            _ => Self::InternalServerError {
                                message: message,
                                error_id: parsed
                                    .get("error_id")
                                    .and_then(|v| v.as_str().map(|s| s.to_string())),
                            },
                        };
                    }
                    return Self::InternalServerError {
                        message: body.unwrap_or("Unknown error").to_string(),
                        error_id: None,
                    };
                }
                return Self::InternalServerError {
                    message: "Unknown error".to_string(),
                    error_id: None,
                };
            }
            503 => {
                // Parse error body for ServiceUnavailableError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::ServiceUnavailableError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                        };
                    }
                }
                return Self::ServiceUnavailableError {
                    message: body.unwrap_or("Unknown error").to_string(),
                };
            }
            401 => {
                // Parse error body for UnauthorizedError;
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        return Self::UnauthorizedError {
                            message: parsed
                                .get("message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Unknown error")
                                .to_string(),
                            auth_type: parsed
                                .get("auth_type")
                                .and_then(|v| v.as_str().map(|s| s.to_string())),
                        };
                    }
                }
                return Self::UnauthorizedError {
                    message: body.unwrap_or("Unknown error").to_string(),
                    auth_type: None,
                };
            }
            402 => {
                if let Some(body_str) = body {
                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(body_str) {
                        let message = parsed
                            .get("message")
                            .and_then(|v| v.as_str())
                            .map(|s| s.to_string())
                            .unwrap_or("Unknown error".to_string());
                        let error_type = parsed.get("error_type").and_then(|v| v.as_str());
                        return match error_type {
                            Some("DeclinedCaptureResponseErrorV2") => {
                                Self::DeclinedCaptureResponseErrorV2 { message: message }
                            }
                            Some("DeclinedAuthResponseErrorV2") => {
                                Self::DeclinedAuthResponseErrorV2 { message: message }
                            }
                            Some("DeclinedRefundResponseErrorV2") => {
                                Self::DeclinedRefundResponseErrorV2 { message: message }
                            }
                            Some("DeclinedVoidResponseErrorV2") => {
                                Self::DeclinedVoidResponseErrorV2 { message: message }
                            }
                            _ => Self::DeclinedCaptureResponseErrorV2 { message: message },
                        };
                    }
                    return Self::DeclinedCaptureResponseErrorV2 {
                        message: body.unwrap_or("Unknown error").to_string(),
                    };
                }
                return Self::DeclinedCaptureResponseErrorV2 {
                    message: "Unknown error".to_string(),
                };
            }
            _ => Self::Http {
                status: status_code,
                message: body.unwrap_or("Unknown error").to_string(),
            },
        }
    }
}
