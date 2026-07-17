pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct SearchNotificationLogsRequest {
    /// The start date for the search.
    #[serde(rename = "startDate")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub start_date: DateTime<Utc>,
    /// The end date for the search.
    #[serde(rename = "endDate")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub end_date: DateTime<Utc>,
    /// The type of notification event to filter by.
    #[serde(rename = "notificationEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_event: Option<String>,
    /// Indicates whether the notification was successful.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded: Option<bool>,
    /// The ID of the organization to filter by.
    #[serde(rename = "orgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<i64>,
    /// The ID of the paypoint to filter by.
    #[serde(rename = "paypointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_id: Option<i64>,
    /// Number of records on each response page.
    #[serde(rename = "PageSize")]
    #[serde(skip)]
    pub page_size: Option<Pagesize>,
    /// The page number to retrieve. Defaults to 1 if not provided.
    #[serde(rename = "Page")]
    #[serde(skip)]
    pub page: Option<i64>,
}

impl SearchNotificationLogsRequest {
    pub fn builder() -> SearchNotificationLogsRequestBuilder {
        <SearchNotificationLogsRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct SearchNotificationLogsRequestBuilder {
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
    notification_event: Option<String>,
    succeeded: Option<bool>,
    org_id: Option<i64>,
    paypoint_id: Option<i64>,
    page_size: Option<Pagesize>,
    page: Option<i64>,
}

impl SearchNotificationLogsRequestBuilder {
    pub fn start_date(mut self, value: DateTime<Utc>) -> Self {
        self.start_date = Some(value);
        self
    }

    pub fn end_date(mut self, value: DateTime<Utc>) -> Self {
        self.end_date = Some(value);
        self
    }

    pub fn notification_event(mut self, value: impl Into<String>) -> Self {
        self.notification_event = Some(value.into());
        self
    }

    pub fn succeeded(mut self, value: bool) -> Self {
        self.succeeded = Some(value);
        self
    }

    pub fn org_id(mut self, value: i64) -> Self {
        self.org_id = Some(value);
        self
    }

    pub fn paypoint_id(mut self, value: i64) -> Self {
        self.paypoint_id = Some(value);
        self
    }

    pub fn page_size(mut self, value: Pagesize) -> Self {
        self.page_size = Some(value);
        self
    }

    pub fn page(mut self, value: i64) -> Self {
        self.page = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`SearchNotificationLogsRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_date`](SearchNotificationLogsRequestBuilder::start_date)
    /// - [`end_date`](SearchNotificationLogsRequestBuilder::end_date)
    pub fn build(self) -> Result<SearchNotificationLogsRequest, BuildError> {
        Ok(SearchNotificationLogsRequest {
            start_date: self
                .start_date
                .ok_or_else(|| BuildError::missing_field("start_date"))?,
            end_date: self
                .end_date
                .ok_or_else(|| BuildError::missing_field("end_date"))?,
            notification_event: self.notification_event,
            succeeded: self.succeeded,
            org_id: self.org_id,
            paypoint_id: self.paypoint_id,
            page_size: self.page_size,
            page: self.page,
        })
    }
}
