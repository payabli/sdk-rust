pub use crate::prelude::*;

/// Details of the registered device used for the transaction. `null` when the
/// transaction has no `DeviceId`, or when no matching device is registered to
/// the paypoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct QueryDeviceData {
    /// Device type, for example `SemiIntegrated`, `Standalone`, or `Softpos`.
    #[serde(rename = "deviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<String>,
    /// Device operating system, for example `Ios` or `Android`.
    #[serde(rename = "deviceOs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_os: Option<String>,
    /// Name assigned to the device at registration.
    #[serde(rename = "friendlyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// Device serial number.
    #[serde(rename = "serialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// Device manufacturer. Can be an empty string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub make: Option<String>,
    /// Device model.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Device MAC address.
    #[serde(rename = "macAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// Device registration status: `Inactive`, `Active`, `Pending`, `Superseded`, `Unregistering`, or `Unregistered`.
    #[serde(rename = "deviceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_status: Option<String>,
}

impl QueryDeviceData {
    pub fn builder() -> QueryDeviceDataBuilder {
        <QueryDeviceDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct QueryDeviceDataBuilder {
    device_type: Option<String>,
    device_os: Option<String>,
    friendly_name: Option<String>,
    serial_number: Option<String>,
    make: Option<String>,
    model: Option<String>,
    mac_address: Option<String>,
    device_status: Option<String>,
}

impl QueryDeviceDataBuilder {
    pub fn device_type(mut self, value: impl Into<String>) -> Self {
        self.device_type = Some(value.into());
        self
    }

    pub fn device_os(mut self, value: impl Into<String>) -> Self {
        self.device_os = Some(value.into());
        self
    }

    pub fn friendly_name(mut self, value: impl Into<String>) -> Self {
        self.friendly_name = Some(value.into());
        self
    }

    pub fn serial_number(mut self, value: impl Into<String>) -> Self {
        self.serial_number = Some(value.into());
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

    pub fn mac_address(mut self, value: impl Into<String>) -> Self {
        self.mac_address = Some(value.into());
        self
    }

    pub fn device_status(mut self, value: impl Into<String>) -> Self {
        self.device_status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`QueryDeviceData`].
    pub fn build(self) -> Result<QueryDeviceData, BuildError> {
        Ok(QueryDeviceData {
            device_type: self.device_type,
            device_os: self.device_os,
            friendly_name: self.friendly_name,
            serial_number: self.serial_number,
            make: self.make,
            model: self.model,
            mac_address: self.mac_address,
            device_status: self.device_status,
        })
    }
}
