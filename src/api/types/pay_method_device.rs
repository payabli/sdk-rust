pub use crate::prelude::*;

/// The required fields for a payment made with a semi-integrated device.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PayMethodDevice {
    /// Identifier of the registered semi-integrated device that takes the payment.
    /// Omitting this field returns response code 7017, and an identifier that
    /// isn't registered to the paypoint returns 7018.
    #[serde(default)]
    pub device: Device,
    /// Method to use for the transaction. For semi-integrated device transactions, the method is `device`.
    pub method: PayMethodDeviceMethod,
    #[serde(rename = "saveIfSuccess")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub save_if_success: Option<SaveIfSuccess>,
}

impl PayMethodDevice {
    pub fn builder() -> PayMethodDeviceBuilder {
        <PayMethodDeviceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayMethodDeviceBuilder {
    device: Option<Device>,
    method: Option<PayMethodDeviceMethod>,
    save_if_success: Option<SaveIfSuccess>,
}

impl PayMethodDeviceBuilder {
    pub fn device(mut self, value: Device) -> Self {
        self.device = Some(value);
        self
    }

    pub fn method(mut self, value: PayMethodDeviceMethod) -> Self {
        self.method = Some(value);
        self
    }

    pub fn save_if_success(mut self, value: SaveIfSuccess) -> Self {
        self.save_if_success = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayMethodDevice`].
    /// This method will fail if any of the following fields are not set:
    /// - [`device`](PayMethodDeviceBuilder::device)
    /// - [`method`](PayMethodDeviceBuilder::method)
    pub fn build(self) -> Result<PayMethodDevice, BuildError> {
        Ok(PayMethodDevice {
            device: self
                .device
                .ok_or_else(|| BuildError::missing_field("device"))?,
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
            save_if_success: self.save_if_success,
        })
    }
}
