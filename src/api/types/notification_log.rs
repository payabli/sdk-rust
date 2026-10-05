pub use crate::prelude::*;

/// A notification log entry returned by the search endpoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct NotificationLog {
    #[serde(flatten)]
    pub notification_log_base_fields: NotificationLogBase,
    /// The URL of the organization's logo.
    #[serde(rename = "organizationLogo")]
    #[serde(default)]
    pub organization_logo: String,
    /// The URL of the organization's browser tab icon.
    #[serde(rename = "organizationFavIcon")]
    #[serde(default)]
    pub organization_fav_icon: String,
    /// The URL of the paypoint's logo.
    #[serde(rename = "paypointLogo")]
    #[serde(default)]
    pub paypoint_logo: String,
    /// The notification's delivery method — `1` (Email), `2` (SMS), or `3` (Webhook).
    #[serde(rename = "notificationType")]
    #[serde(default)]
    pub notification_type: i64,
}

impl NotificationLog {
    pub fn builder() -> NotificationLogBuilder {
        <NotificationLogBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct NotificationLogBuilder {
    notification_log_base_fields: Option<NotificationLogBase>,
    organization_logo: Option<String>,
    organization_fav_icon: Option<String>,
    paypoint_logo: Option<String>,
    notification_type: Option<i64>,
}

impl NotificationLogBuilder {
    pub fn notification_log_base_fields(mut self, value: NotificationLogBase) -> Self {
        self.notification_log_base_fields = Some(value);
        self
    }

    pub fn organization_logo(mut self, value: impl Into<String>) -> Self {
        self.organization_logo = Some(value.into());
        self
    }

    pub fn organization_fav_icon(mut self, value: impl Into<String>) -> Self {
        self.organization_fav_icon = Some(value.into());
        self
    }

    pub fn paypoint_logo(mut self, value: impl Into<String>) -> Self {
        self.paypoint_logo = Some(value.into());
        self
    }

    pub fn notification_type(mut self, value: i64) -> Self {
        self.notification_type = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`NotificationLog`].
    /// This method will fail if any of the following fields are not set:
    /// - [`notification_log_base_fields`](NotificationLogBuilder::notification_log_base_fields)
    /// - [`organization_logo`](NotificationLogBuilder::organization_logo)
    /// - [`organization_fav_icon`](NotificationLogBuilder::organization_fav_icon)
    /// - [`paypoint_logo`](NotificationLogBuilder::paypoint_logo)
    /// - [`notification_type`](NotificationLogBuilder::notification_type)
    pub fn build(self) -> Result<NotificationLog, BuildError> {
        Ok(NotificationLog {
            notification_log_base_fields: self
                .notification_log_base_fields
                .ok_or_else(|| BuildError::missing_field("notification_log_base_fields"))?,
            organization_logo: self
                .organization_logo
                .ok_or_else(|| BuildError::missing_field("organization_logo"))?,
            organization_fav_icon: self
                .organization_fav_icon
                .ok_or_else(|| BuildError::missing_field("organization_fav_icon"))?,
            paypoint_logo: self
                .paypoint_logo
                .ok_or_else(|| BuildError::missing_field("paypoint_logo"))?,
            notification_type: self
                .notification_type
                .ok_or_else(|| BuildError::missing_field("notification_type"))?,
        })
    }
}
