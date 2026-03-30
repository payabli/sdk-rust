pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PayoutGatewayConnector {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration: Option<String>,
    #[serde(rename = "Name")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Mode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<i64>,
    #[serde(rename = "Bank")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<String>,
    #[serde(rename = "Descriptor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub descriptor: Option<String>,
    #[serde(rename = "gatewayID")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway_id: Option<i64>,
    #[serde(rename = "Enabled")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[serde(rename = "EnableACHValidation")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_ach_validation: Option<bool>,
    #[serde(rename = "TestMode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_mode: Option<bool>,
}

impl PayoutGatewayConnector {
    pub fn builder() -> PayoutGatewayConnectorBuilder {
        <PayoutGatewayConnectorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PayoutGatewayConnectorBuilder {
    configuration: Option<String>,
    name: Option<String>,
    mode: Option<i64>,
    bank: Option<String>,
    descriptor: Option<String>,
    gateway_id: Option<i64>,
    enabled: Option<bool>,
    enable_ach_validation: Option<bool>,
    test_mode: Option<bool>,
}

impl PayoutGatewayConnectorBuilder {
    pub fn configuration(mut self, value: impl Into<String>) -> Self {
        self.configuration = Some(value.into());
        self
    }

    pub fn name(mut self, value: impl Into<String>) -> Self {
        self.name = Some(value.into());
        self
    }

    pub fn mode(mut self, value: i64) -> Self {
        self.mode = Some(value);
        self
    }

    pub fn bank(mut self, value: impl Into<String>) -> Self {
        self.bank = Some(value.into());
        self
    }

    pub fn descriptor(mut self, value: impl Into<String>) -> Self {
        self.descriptor = Some(value.into());
        self
    }

    pub fn gateway_id(mut self, value: i64) -> Self {
        self.gateway_id = Some(value);
        self
    }

    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = Some(value);
        self
    }

    pub fn enable_ach_validation(mut self, value: bool) -> Self {
        self.enable_ach_validation = Some(value);
        self
    }

    pub fn test_mode(mut self, value: bool) -> Self {
        self.test_mode = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PayoutGatewayConnector`].
    pub fn build(self) -> Result<PayoutGatewayConnector, BuildError> {
        Ok(PayoutGatewayConnector {
            configuration: self.configuration,
            name: self.name,
            mode: self.mode,
            bank: self.bank,
            descriptor: self.descriptor,
            gateway_id: self.gateway_id,
            enabled: self.enabled,
            enable_ach_validation: self.enable_ach_validation,
            test_mode: self.test_mode,
        })
    }
}
