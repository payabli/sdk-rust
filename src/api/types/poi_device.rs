pub use crate::prelude::*;

/// Information about the point of interaction device (also known as a terminal or cloud device) used to process the transaction.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PoiDevice {
    /// The device connection status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected: Option<bool>,
    /// The date the device was unregistered.
    #[serde(rename = "dateDeRegistered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub date_de_registered: Option<DateTime<Utc>>,
    /// The date the device was registered.
    #[serde(rename = "dateRegistered")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub date_registered: Option<DateTime<Utc>>,
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
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub last_connected_date: Option<DateTime<Utc>>,
    /// Last disconnected date.
    #[serde(rename = "lastDisconnectedDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub last_disconnected_date: Option<DateTime<Utc>>,
    /// Last transaction date.
    #[serde(rename = "lastTransactionDate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub last_transaction_date: Option<DateTime<Utc>>,
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

impl PoiDevice {
    pub fn builder() -> PoiDeviceBuilder {
        <PoiDeviceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PoiDeviceBuilder {
    connected: Option<bool>,
    date_de_registered: Option<DateTime<Utc>>,
    date_registered: Option<DateTime<Utc>>,
    device_id: Option<String>,
    device_license: Option<String>,
    device_nick_name: Option<String>,
    last_connected_date: Option<DateTime<Utc>>,
    last_disconnected_date: Option<DateTime<Utc>>,
    last_transaction_date: Option<DateTime<Utc>>,
    make: Option<String>,
    model: Option<String>,
    registered: Option<bool>,
    serial_number: Option<String>,
}

impl PoiDeviceBuilder {
    pub fn connected(mut self, value: bool) -> Self {
        self.connected = Some(value);
        self
    }

    pub fn date_de_registered(mut self, value: DateTime<Utc>) -> Self {
        self.date_de_registered = Some(value);
        self
    }

    pub fn date_registered(mut self, value: DateTime<Utc>) -> Self {
        self.date_registered = Some(value);
        self
    }

    pub fn device_id(mut self, value: impl Into<String>) -> Self {
        self.device_id = Some(value.into());
        self
    }

    pub fn device_license(mut self, value: impl Into<String>) -> Self {
        self.device_license = Some(value.into());
        self
    }

    pub fn device_nick_name(mut self, value: impl Into<String>) -> Self {
        self.device_nick_name = Some(value.into());
        self
    }

    pub fn last_connected_date(mut self, value: DateTime<Utc>) -> Self {
        self.last_connected_date = Some(value);
        self
    }

    pub fn last_disconnected_date(mut self, value: DateTime<Utc>) -> Self {
        self.last_disconnected_date = Some(value);
        self
    }

    pub fn last_transaction_date(mut self, value: DateTime<Utc>) -> Self {
        self.last_transaction_date = Some(value);
        self
    }

    pub fn make(mut self, value: impl Into<String>) -> Self {
        self.make = Some(value.into());
        self
    }

    pub fn model(mut self, value: impl Into<String>) -> Self {
        self.model = Some(value.into());
        self
    }

    pub fn registered(mut self, value: bool) -> Self {
        self.registered = Some(value);
        self
    }

    pub fn serial_number(mut self, value: impl Into<String>) -> Self {
        self.serial_number = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PoiDevice`].
    pub fn build(self) -> Result<PoiDevice, BuildError> {
        Ok(PoiDevice {
            connected: self.connected,
            date_de_registered: self.date_de_registered,
            date_registered: self.date_registered,
            device_id: self.device_id,
            device_license: self.device_license,
            device_nick_name: self.device_nick_name,
            last_connected_date: self.last_connected_date,
            last_disconnected_date: self.last_disconnected_date,
            last_transaction_date: self.last_transaction_date,
            make: self.make,
            model: self.model,
            registered: self.registered,
            serial_number: self.serial_number,
        })
    }
}
