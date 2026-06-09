pub use crate::prelude::*;

/// Information about the report notification configuration (report-email, report-web).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct NotificationReportRequest {
    #[serde(default)]
    pub content: NotificationReportRequestContent,
    pub frequency: NotificationReportRequestFrequency,
    /// Automated reporting lets you gather critical reports without manually filtering and exporting the data. Get automated daily, weekly, and monthly reports for daily sales, ACH returns, settlements, and more. You can send these reports via email or via webhook. See [Automated Reports](/developers/developer-guides/notifications-and-webhooks-overview#automated-reports) for more.
    pub method: NotificationReportRequestMethod,
    #[serde(rename = "ownerId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner_id: Option<Ownerid>,
    #[serde(rename = "ownerType")]
    #[serde(default)]
    pub owner_type: Ownertype,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<Statusnotification>,
    /// Specify the notification target.
    /// For method=report-email the expected value is a list of email addresses separated by semicolon.
    /// For method=report-web the expected value is a valid and complete URL. Webhooks support only standard HTTP ports: 80, 443, 8080, or 4443.
    #[serde(default)]
    pub target: String,
}

impl NotificationReportRequest {
    pub fn builder() -> NotificationReportRequestBuilder {
        <NotificationReportRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotificationReportRequestBuilder {
    content: Option<NotificationReportRequestContent>,
    frequency: Option<NotificationReportRequestFrequency>,
    method: Option<NotificationReportRequestMethod>,
    owner_id: Option<Ownerid>,
    owner_type: Option<Ownertype>,
    status: Option<Statusnotification>,
    target: Option<String>,
}

impl NotificationReportRequestBuilder {
    pub fn content(mut self, value: NotificationReportRequestContent) -> Self {
        self.content = Some(value);
        self
    }

    pub fn frequency(mut self, value: NotificationReportRequestFrequency) -> Self {
        self.frequency = Some(value);
        self
    }

    pub fn method(mut self, value: NotificationReportRequestMethod) -> Self {
        self.method = Some(value);
        self
    }

    pub fn owner_id(mut self, value: Ownerid) -> Self {
        self.owner_id = Some(value);
        self
    }

    pub fn owner_type(mut self, value: Ownertype) -> Self {
        self.owner_type = Some(value);
        self
    }

    pub fn status(mut self, value: Statusnotification) -> Self {
        self.status = Some(value);
        self
    }

    pub fn target(mut self, value: impl Into<String>) -> Self {
        self.target = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`NotificationReportRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`content`](NotificationReportRequestBuilder::content)
    /// - [`frequency`](NotificationReportRequestBuilder::frequency)
    /// - [`method`](NotificationReportRequestBuilder::method)
    /// - [`owner_type`](NotificationReportRequestBuilder::owner_type)
    /// - [`target`](NotificationReportRequestBuilder::target)
    pub fn build(self) -> Result<NotificationReportRequest, BuildError> {
        Ok(NotificationReportRequest {
            content: self
                .content
                .ok_or_else(|| BuildError::missing_field("content"))?,
            frequency: self
                .frequency
                .ok_or_else(|| BuildError::missing_field("frequency"))?,
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
            owner_id: self.owner_id,
            owner_type: self
                .owner_type
                .ok_or_else(|| BuildError::missing_field("owner_type"))?,
            status: self.status,
            target: self
                .target
                .ok_or_else(|| BuildError::missing_field("target"))?,
        })
    }
}
