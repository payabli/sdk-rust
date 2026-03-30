pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NotificationLogSearchRequest {
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
}

impl NotificationLogSearchRequest {
    pub fn builder() -> NotificationLogSearchRequestBuilder {
        <NotificationLogSearchRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotificationLogSearchRequestBuilder {
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
    notification_event: Option<String>,
    succeeded: Option<bool>,
    org_id: Option<i64>,
    paypoint_id: Option<i64>,
}

impl NotificationLogSearchRequestBuilder {
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

    /// Consumes the builder and constructs a [`NotificationLogSearchRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`start_date`](NotificationLogSearchRequestBuilder::start_date)
    /// - [`end_date`](NotificationLogSearchRequestBuilder::end_date)
    pub fn build(self) -> Result<NotificationLogSearchRequest, BuildError> {
        Ok(NotificationLogSearchRequest {
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
        })
    }
}
