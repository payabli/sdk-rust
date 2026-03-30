pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct DeviceEntry {
    /// Description or name for the device. This can be anything, but Payabli recommends entering the name of the paypoint, or some other easy to identify descriptor. If you have several devices for one paypoint, you can give them descriptions like "Cashier 1" and "Cashier 2", or "Front Desk" and "Back Office"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The device registration code or serial number, depending on the model.
    ///
    /// - Ingenico devices: This is the activation code that's displayed on the device screen during setup.
    ///
    /// - PAX A920 device: This code is the serial number on the back of the device.
    #[serde(rename = "registrationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_code: Option<String>,
}

impl DeviceEntry {
    pub fn builder() -> DeviceEntryBuilder {
        <DeviceEntryBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeviceEntryBuilder {
    description: Option<String>,
    registration_code: Option<String>,
}

impl DeviceEntryBuilder {
    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn registration_code(mut self, value: impl Into<String>) -> Self {
        self.registration_code = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`DeviceEntry`].
    pub fn build(self) -> Result<DeviceEntry, BuildError> {
        Ok(DeviceEntry {
            description: self.description,
            registration_code: self.registration_code,
        })
    }
}
