pub use crate::prelude::*;

/// Shows whether the device can accept a transaction now. The value
/// reflects the device's state when you send the query.
///
/// - `online`: The device is active and isn't running a command. It can
/// accept a transaction.
/// - `busy`: The device is running a command that's pending, dispatched,
/// or processing, and the command hasn't expired. A new transaction sent
/// to the device fails with error `E7069` (Device busy). Wait for the
/// command to finish or expire, then query the device again.
/// - `offline`: The device isn't active, so it can't accept transactions.
/// Its lifecycle status (`deviceStatus`) isn't active.
///
/// Check this field before you send a transaction to a device.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeviceQueryRecordOperationalStatus {
    Online,
    Busy,
    Offline,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for DeviceQueryRecordOperationalStatus {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::Online => serializer.serialize_str("online"),
            Self::Busy => serializer.serialize_str("busy"),
            Self::Offline => serializer.serialize_str("offline"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for DeviceQueryRecordOperationalStatus {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "online" => Ok(Self::Online),
            "busy" => Ok(Self::Busy),
            "offline" => Ok(Self::Offline),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for DeviceQueryRecordOperationalStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Online => write!(f, "online"),
            Self::Busy => write!(f, "busy"),
            Self::Offline => write!(f, "offline"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
