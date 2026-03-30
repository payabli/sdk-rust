pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NotificationLog {
    /// The unique identifier for the notification.
    #[serde(default)]
    pub id: Uuid,
    /// The ID of the organization that the notification belongs to.
    #[serde(rename = "orgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<i64>,
    /// The ID of the paypoint that the notification is related to.
    #[serde(rename = "paypointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_id: Option<i64>,
    /// The event that triggered the notification.
    #[serde(rename = "notificationEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_event: Option<String>,
    /// The target URL for the notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// The HTTP response status of the notification.
    #[serde(rename = "responseStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_status: Option<String>,
    /// Indicates whether the notification was successful.
    #[serde(default)]
    pub success: bool,
    /// Contains the body of the notification.
    #[serde(rename = "jobData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_data: Option<String>,
    /// The date and time when the notification was created.
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub created_date: DateTime<Utc>,
    /// The date and time when the notification was successfully delivered.
    #[serde(rename = "successDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub success_date: Option<DateTime<Utc>>,
    /// The date and time when the notification last failed.
    #[serde(rename = "lastFailedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub last_failed_date: Option<DateTime<Utc>>,
    /// Indicates whether the notification is currently in progress.
    #[serde(rename = "isInProgress")]
    #[serde(default)]
    pub is_in_progress: bool,
}

impl NotificationLog {
    pub fn builder() -> NotificationLogBuilder {
        <NotificationLogBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotificationLogBuilder {
    id: Option<Uuid>,
    org_id: Option<i64>,
    paypoint_id: Option<i64>,
    notification_event: Option<String>,
    target: Option<String>,
    response_status: Option<String>,
    success: Option<bool>,
    job_data: Option<String>,
    created_date: Option<DateTime<Utc>>,
    success_date: Option<DateTime<Utc>>,
    last_failed_date: Option<DateTime<Utc>>,
    is_in_progress: Option<bool>,
}

impl NotificationLogBuilder {
    pub fn id(mut self, value: Uuid) -> Self {
        self.id = Some(value);
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

    pub fn notification_event(mut self, value: impl Into<String>) -> Self {
        self.notification_event = Some(value.into());
        self
    }

    pub fn target(mut self, value: impl Into<String>) -> Self {
        self.target = Some(value.into());
        self
    }

    pub fn response_status(mut self, value: impl Into<String>) -> Self {
        self.response_status = Some(value.into());
        self
    }

    pub fn success(mut self, value: bool) -> Self {
        self.success = Some(value);
        self
    }

    pub fn job_data(mut self, value: impl Into<String>) -> Self {
        self.job_data = Some(value.into());
        self
    }

    pub fn created_date(mut self, value: DateTime<Utc>) -> Self {
        self.created_date = Some(value);
        self
    }

    pub fn success_date(mut self, value: DateTime<Utc>) -> Self {
        self.success_date = Some(value);
        self
    }

    pub fn last_failed_date(mut self, value: DateTime<Utc>) -> Self {
        self.last_failed_date = Some(value);
        self
    }

    pub fn is_in_progress(mut self, value: bool) -> Self {
        self.is_in_progress = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NotificationLog`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](NotificationLogBuilder::id)
    /// - [`success`](NotificationLogBuilder::success)
    /// - [`created_date`](NotificationLogBuilder::created_date)
    /// - [`is_in_progress`](NotificationLogBuilder::is_in_progress)
    pub fn build(self) -> Result<NotificationLog, BuildError> {
        Ok(NotificationLog {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            org_id: self.org_id,
            paypoint_id: self.paypoint_id,
            notification_event: self.notification_event,
            target: self.target,
            response_status: self.response_status,
            success: self
                .success
                .ok_or_else(|| BuildError::missing_field("success"))?,
            job_data: self.job_data,
            created_date: self
                .created_date
                .ok_or_else(|| BuildError::missing_field("created_date"))?,
            success_date: self.success_date,
            last_failed_date: self.last_failed_date,
            is_in_progress: self
                .is_in_progress
                .ok_or_else(|| BuildError::missing_field("is_in_progress"))?,
        })
    }
}
