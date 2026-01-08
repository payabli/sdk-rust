pub use crate::prelude::*;

/// Information about the point of interaction device (also known as a terminal or cloud device) used to process the transaction.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PoiDevice {
    /// The device connection status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,
    /// The date the device was unregistered.
    #[serde(rename = "dateDeRegistered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub date_de_registered: Option<DateTime<FixedOffset>>,
    /// The date the device was registered.
    #[serde(rename = "dateRegistered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub date_registered: Option<DateTime<FixedOffset>>,
    /// The device identifier.
    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// Device license. This is typically the same as `deviceId`.
    #[serde(rename = "deviceLicense")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_license: Option<String>,
    /// Device description provided during registration.
    #[serde(rename = "deviceNickName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_nick_name: Option<String>,
    /// Last connected date.
    #[serde(rename = "lastConnectedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_connected_date: Option<DateTime<FixedOffset>>,
    /// Last disconnected date.
    #[serde(rename = "lastDisconnectedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_disconnected_date: Option<DateTime<FixedOffset>>,
    /// Last transaction date.
    #[serde(rename = "lastTransactionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::offset::option")]
    pub last_transaction_date: Option<DateTime<FixedOffset>>,
    /// The device manufacturer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub make: Option<String>,
    /// The device model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// The device registration status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registered: Option<bool>,
    /// The device serial number.
    #[serde(rename = "serialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
}
