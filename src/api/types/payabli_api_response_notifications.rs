pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PayabliApiResponseNotifications {
    /// If `isSuccess` = true, `responseData` contains the notification identifier.
    ///
    /// If `isSuccess` = false, `responseData` contains the reason for the error.
    #[serde(rename = "isSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_success: Option<IsSuccess>,
    #[serde(rename = "pageIdentifier")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_identifier: Option<PageIdentifier>,
    #[serde(rename = "responseCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_code: Option<Responsecode>,
    /// When the request was successful, this contains the notification ID, or `nID` used to manage the notification.
    #[serde(rename = "responseData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub response_data: Option<PayabliApiResponseNotificationsResponseData>,
    #[serde(rename = "responseText")]
    pub response_text: ResponseText,
}
