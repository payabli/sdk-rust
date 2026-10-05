pub use crate::prelude::*;

/// Fields shared by the search and detail responses for a notification log entry.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NotificationLogBase {
    /// The name of the organization the notification belongs to.
    #[serde(rename = "organizationName")]
    #[serde(default)]
    pub organization_name: String,
    /// The name of the paypoint the notification is related to. Empty for organization-level notifications.
    #[serde(rename = "paypointName")]
    #[serde(default)]
    pub paypoint_name: String,
    /// The identifier for the delivery request.
    #[serde(rename = "requestId")]
    #[serde(default)]
    pub request_id: String,
    /// The unique identifier for the notification.
    #[serde(default)]
    pub id: String,
    /// The ID of the organization the notification belongs to.
    #[serde(rename = "orgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org_id: Option<i64>,
    /// The ID of the paypoint the notification is related to. Null for organization-level notifications.
    #[serde(rename = "paypointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_id: Option<i64>,
    /// The event that triggered the notification, such as `approvedpayment`.
    #[serde(rename = "notificationEvent")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_event: Option<String>,
    /// The target the notification was delivered to, such as a webhook URL, email address, or phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// The HTTP status code the target returned, such as `200`. Returns `0` when the target sent no response.
    #[serde(rename = "responseStatusCode")]
    #[serde(default)]
    pub response_status_code: i64,
    /// The delivery status message, such as `OK` when the notification succeeded, `Dropped` when it failed, or `No response received from server.` when the target sent no response.
    #[serde(rename = "responseStatus")]
    #[serde(default)]
    pub response_status: String,
    /// Indicates whether the notification was delivered successfully.
    #[serde(default)]
    pub success: bool,
    /// The body of the notification.
    #[serde(rename = "jobData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job_data: Option<String>,
    /// The date and time when the notification was created.
    #[serde(rename = "createdDate")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub created_date: DateTime<Utc>,
    /// The date and time when the notification was delivered successfully. Null if it hasn't succeeded.
    #[serde(rename = "successDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub success_date: Option<DateTime<Utc>>,
    /// The date and time when the notification last failed. Null if it hasn't failed.
    #[serde(rename = "lastFailedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub last_failed_date: Option<DateTime<Utc>>,
    /// Indicates whether the notification is currently being sent.
    #[serde(rename = "isInProgress")]
    #[serde(default)]
    pub is_in_progress: bool,
}

impl NotificationLogBase {
    pub fn builder() -> NotificationLogBaseBuilder {
        <NotificationLogBaseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotificationLogBaseBuilder {
    organization_name: Option<String>,
    paypoint_name: Option<String>,
    request_id: Option<String>,
    id: Option<String>,
    org_id: Option<i64>,
    paypoint_id: Option<i64>,
    notification_event: Option<String>,
    target: Option<String>,
    response_status_code: Option<i64>,
    response_status: Option<String>,
    success: Option<bool>,
    job_data: Option<String>,
    created_date: Option<DateTime<Utc>>,
    success_date: Option<DateTime<Utc>>,
    last_failed_date: Option<DateTime<Utc>>,
    is_in_progress: Option<bool>,
}

impl NotificationLogBaseBuilder {
    pub fn organization_name(mut self, value: impl Into<String>) -> Self {
        self.organization_name = Some(value.into());
        self
    }

    pub fn paypoint_name(mut self, value: impl Into<String>) -> Self {
        self.paypoint_name = Some(value.into());
        self
    }

    pub fn request_id(mut self, value: impl Into<String>) -> Self {
        self.request_id = Some(value.into());
        self
    }

    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
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

    pub fn response_status_code(mut self, value: i64) -> Self {
        self.response_status_code = Some(value);
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

    /// Consumes the builder and constructs a [`NotificationLogBase`].
    /// This method will fail if any of the following fields are not set:
    /// - [`organization_name`](NotificationLogBaseBuilder::organization_name)
    /// - [`paypoint_name`](NotificationLogBaseBuilder::paypoint_name)
    /// - [`request_id`](NotificationLogBaseBuilder::request_id)
    /// - [`id`](NotificationLogBaseBuilder::id)
    /// - [`response_status_code`](NotificationLogBaseBuilder::response_status_code)
    /// - [`response_status`](NotificationLogBaseBuilder::response_status)
    /// - [`success`](NotificationLogBaseBuilder::success)
    /// - [`created_date`](NotificationLogBaseBuilder::created_date)
    /// - [`is_in_progress`](NotificationLogBaseBuilder::is_in_progress)
    pub fn build(self) -> Result<NotificationLogBase, BuildError> {
        Ok(NotificationLogBase {
            organization_name: self
                .organization_name
                .ok_or_else(|| BuildError::missing_field("organization_name"))?,
            paypoint_name: self
                .paypoint_name
                .ok_or_else(|| BuildError::missing_field("paypoint_name"))?,
            request_id: self
                .request_id
                .ok_or_else(|| BuildError::missing_field("request_id"))?,
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            org_id: self.org_id,
            paypoint_id: self.paypoint_id,
            notification_event: self.notification_event,
            target: self.target,
            response_status_code: self
                .response_status_code
                .ok_or_else(|| BuildError::missing_field("response_status_code"))?,
            response_status: self
                .response_status
                .ok_or_else(|| BuildError::missing_field("response_status"))?,
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
