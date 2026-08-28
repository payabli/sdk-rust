pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PayMethodCloud {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
    /// Method to use for the transaction. For cloud device transactions, the method is `cloud`.
    pub method: PayMethodCloudMethod,
}

impl PayMethodCloud {
    pub fn builder() -> PayMethodCloudBuilder {
        <PayMethodCloudBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayMethodCloudBuilder {
    device: Option<Device>,
    method: Option<PayMethodCloudMethod>,
}

impl PayMethodCloudBuilder {
    pub fn device(mut self, value: Device) -> Self {
        self.device = Some(value);
        self
    }

    pub fn method(mut self, value: PayMethodCloudMethod) -> Self {
        self.method = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayMethodCloud`].
    /// This method will fail if any of the following fields are not set:
    /// - [`method`](PayMethodCloudBuilder::method)
    pub fn build(self) -> Result<PayMethodCloud, BuildError> {
        Ok(PayMethodCloud {
            device: self.device,
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
        })
    }
}
