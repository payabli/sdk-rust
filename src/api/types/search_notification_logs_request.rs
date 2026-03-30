pub use crate::prelude::*;

/// Request for searchNotificationLogs (body + query parameters)
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchNotificationLogsRequest {
    #[serde(rename = "PageSize")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<Pagesize>,
    /// The page number to retrieve. Defaults to 1 if not provided.
    #[serde(rename = "Page")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(default)]
    pub body: NotificationLogSearchRequest,
}

impl SearchNotificationLogsRequest {
    pub fn builder() -> SearchNotificationLogsRequestBuilder {
        <SearchNotificationLogsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchNotificationLogsRequestBuilder {
    page_size: Option<Pagesize>,
    page: Option<i64>,
    body: Option<NotificationLogSearchRequest>,
}

impl SearchNotificationLogsRequestBuilder {
    pub fn page_size(mut self, value: Pagesize) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    pub fn body(mut self, value: NotificationLogSearchRequest) -> Self {
        self.body = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchNotificationLogsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`body`](SearchNotificationLogsRequestBuilder::body)
    pub fn build(self) -> Result<SearchNotificationLogsRequest, BuildError> {
        Ok(SearchNotificationLogsRequest {
            page_size: self.page_size,
            page: self.page,
            body: self.body.ok_or_else(|| BuildError::missing_field("body"))?,
        })
    }
}
