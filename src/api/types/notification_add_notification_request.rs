pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(untagged)]
pub enum AddNotificationRequest {
    NotificationStandardRequest(NotificationStandardRequest),

    NotificationReportRequest(NotificationReportRequest),
}

impl AddNotificationRequest {
    pub fn is_notification_standard_request(&self) -> bool {
        matches!(self, Self::NotificationStandardRequest(_))
    }

    pub fn is_notification_report_request(&self) -> bool {
        matches!(self, Self::NotificationReportRequest(_))
    }

    pub fn as_notification_standard_request(&self) -> Option<&NotificationStandardRequest> {
        match self {
            Self::NotificationStandardRequest(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_notification_standard_request(self) -> Option<NotificationStandardRequest> {
        match self {
            Self::NotificationStandardRequest(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_notification_report_request(&self) -> Option<&NotificationReportRequest> {
        match self {
            Self::NotificationReportRequest(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_notification_report_request(self) -> Option<NotificationReportRequest> {
        match self {
            Self::NotificationReportRequest(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for AddNotificationRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotificationStandardRequest(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
            Self::NotificationReportRequest(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
