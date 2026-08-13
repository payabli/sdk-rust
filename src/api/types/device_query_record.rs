pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct DeviceQueryRecord {
    /// Unique identifier for the cloud device.
    #[serde(rename = "deviceId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// Internal cloud device record ID.
    #[serde(rename = "idCloud")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_cloud: Option<i64>,
    /// Description of the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Serial number of the device.
    #[serde(rename = "serialNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_number: Option<String>,
    /// Human-readable name for the device.
    #[serde(rename = "friendlyName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub friendly_name: Option<String>,
    /// Manufacturer of the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub make: Option<String>,
    /// Model name of the device.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    /// Type of device.
    #[serde(rename = "deviceType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<i64>,
    /// Current status of the device.
    #[serde(rename = "deviceStatus")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_status: Option<i64>,
    /// Operating system of the device.
    #[serde(rename = "deviceOs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_os: Option<i64>,
    /// MAC address of the device.
    #[serde(rename = "macAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mac_address: Option<String>,
    /// Timestamp of the last health check from the device.
    #[serde(rename = "lastHealthCheck")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_health_check: Option<String>,
    /// Registration code used to activate the device.
    #[serde(rename = "registrationCode")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub registration_code: Option<String>,
    /// Number of activation attempts for the device.
    #[serde(rename = "activationAttempts")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_attempts: Option<i64>,
    /// Expiration timestamp for the device activation code.
    #[serde(rename = "activationCodeExpiry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub activation_code_expiry: Option<String>,
    /// Timestamp when the device record was created.
    #[serde(rename = "createdAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    /// Timestamp when the device record was last updated.
    #[serde(rename = "updatedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    /// Numeric identifier for the paypoint.
    #[serde(rename = "paypointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_id: Option<i64>,
    /// DBA name for the paypoint.
    #[serde(rename = "paypointDba")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_dba: Option<String>,
    /// Legal name for the paypoint.
    #[serde(rename = "paypointLegal")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_legal: Option<String>,
    /// Entry identifier for the paypoint.
    #[serde(rename = "paypointEntry")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_entry: Option<String>,
    /// URL of the paypoint's logo, when available.
    #[serde(rename = "paypointLogo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_logo: Option<String>,
    /// External identifier for the paypoint.
    #[serde(rename = "externalPaypointId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_paypoint_id: Option<String>,
    /// Numeric identifier for the parent organization.
    #[serde(rename = "parentOrgId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_id: Option<i64>,
    /// Name of the parent organization.
    #[serde(rename = "parentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<String>,
    /// URL of the parent organization's logo, when available.
    #[serde(rename = "parentOrgLogo")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_logo: Option<String>,
    /// Total number of transactions processed by this device.
    #[serde(rename = "transactionCount")]
    #[serde(default)]
    pub transaction_count: i64,
    /// Total volume processed by this device, as the sum of net transaction amounts.
    #[serde(rename = "volumeProcessed")]
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub volume_processed: f64,
}

impl DeviceQueryRecord {
    pub fn builder() -> DeviceQueryRecordBuilder {
        <DeviceQueryRecordBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct DeviceQueryRecordBuilder {
    device_id: Option<String>,
    id_cloud: Option<i64>,
    description: Option<String>,
    serial_number: Option<String>,
    friendly_name: Option<String>,
    make: Option<String>,
    model: Option<String>,
    device_type: Option<i64>,
    device_status: Option<i64>,
    device_os: Option<i64>,
    mac_address: Option<String>,
    last_health_check: Option<String>,
    registration_code: Option<String>,
    activation_attempts: Option<i64>,
    activation_code_expiry: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
    paypoint_id: Option<i64>,
    paypoint_dba: Option<String>,
    paypoint_legal: Option<String>,
    paypoint_entry: Option<String>,
    paypoint_logo: Option<String>,
    external_paypoint_id: Option<String>,
    parent_org_id: Option<i64>,
    parent_org_name: Option<String>,
    parent_org_logo: Option<String>,
    transaction_count: Option<i64>,
    volume_processed: Option<f64>,
}

impl DeviceQueryRecordBuilder {
    pub fn device_id(mut self, value: impl Into<String>) -> Self {
        self.device_id = Some(value.into());
        self
    }

    pub fn id_cloud(mut self, value: i64) -> Self {
        self.id_cloud = Some(value);
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn serial_number(mut self, value: impl Into<String>) -> Self {
        self.serial_number = Some(value.into());
        self
    }

    pub fn friendly_name(mut self, value: impl Into<String>) -> Self {
        self.friendly_name = Some(value.into());
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

    pub fn device_type(mut self, value: i64) -> Self {
        self.device_type = Some(value);
        self
    }

    pub fn device_status(mut self, value: i64) -> Self {
        self.device_status = Some(value);
        self
    }

    pub fn device_os(mut self, value: i64) -> Self {
        self.device_os = Some(value);
        self
    }

    pub fn mac_address(mut self, value: impl Into<String>) -> Self {
        self.mac_address = Some(value.into());
        self
    }

    pub fn last_health_check(mut self, value: impl Into<String>) -> Self {
        self.last_health_check = Some(value.into());
        self
    }

    pub fn registration_code(mut self, value: impl Into<String>) -> Self {
        self.registration_code = Some(value.into());
        self
    }

    pub fn activation_attempts(mut self, value: i64) -> Self {
        self.activation_attempts = Some(value);
        self
    }

    pub fn activation_code_expiry(mut self, value: impl Into<String>) -> Self {
        self.activation_code_expiry = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn updated_at(mut self, value: impl Into<String>) -> Self {
        self.updated_at = Some(value.into());
        self
    }

    pub fn paypoint_id(mut self, value: i64) -> Self {
        self.paypoint_id = Some(value);
        self
    }

    pub fn paypoint_dba(mut self, value: impl Into<String>) -> Self {
        self.paypoint_dba = Some(value.into());
        self
    }

    pub fn paypoint_legal(mut self, value: impl Into<String>) -> Self {
        self.paypoint_legal = Some(value.into());
        self
    }

    pub fn paypoint_entry(mut self, value: impl Into<String>) -> Self {
        self.paypoint_entry = Some(value.into());
        self
    }

    pub fn paypoint_logo(mut self, value: impl Into<String>) -> Self {
        self.paypoint_logo = Some(value.into());
        self
    }

    pub fn external_paypoint_id(mut self, value: impl Into<String>) -> Self {
        self.external_paypoint_id = Some(value.into());
        self
    }

    pub fn parent_org_id(mut self, value: i64) -> Self {
        self.parent_org_id = Some(value);
        self
    }

    pub fn parent_org_name(mut self, value: impl Into<String>) -> Self {
        self.parent_org_name = Some(value.into());
        self
    }

    pub fn parent_org_logo(mut self, value: impl Into<String>) -> Self {
        self.parent_org_logo = Some(value.into());
        self
    }

    pub fn transaction_count(mut self, value: i64) -> Self {
        self.transaction_count = Some(value);
        self
    }

    pub fn volume_processed(mut self, value: f64) -> Self {
        self.volume_processed = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`DeviceQueryRecord`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transaction_count`](DeviceQueryRecordBuilder::transaction_count)
    /// - [`volume_processed`](DeviceQueryRecordBuilder::volume_processed)
    pub fn build(self) -> Result<DeviceQueryRecord, BuildError> {
        Ok(DeviceQueryRecord {
            device_id: self.device_id,
            id_cloud: self.id_cloud,
            description: self.description,
            serial_number: self.serial_number,
            friendly_name: self.friendly_name,
            make: self.make,
            model: self.model,
            device_type: self.device_type,
            device_status: self.device_status,
            device_os: self.device_os,
            mac_address: self.mac_address,
            last_health_check: self.last_health_check,
            registration_code: self.registration_code,
            activation_attempts: self.activation_attempts,
            activation_code_expiry: self.activation_code_expiry,
            created_at: self.created_at,
            updated_at: self.updated_at,
            paypoint_id: self.paypoint_id,
            paypoint_dba: self.paypoint_dba,
            paypoint_legal: self.paypoint_legal,
            paypoint_entry: self.paypoint_entry,
            paypoint_logo: self.paypoint_logo,
            external_paypoint_id: self.external_paypoint_id,
            parent_org_id: self.parent_org_id,
            parent_org_name: self.parent_org_name,
            parent_org_logo: self.parent_org_logo,
            transaction_count: self
                .transaction_count
                .ok_or_else(|| BuildError::missing_field("transaction_count"))?,
            volume_processed: self
                .volume_processed
                .ok_or_else(|| BuildError::missing_field("volume_processed"))?,
        })
    }
}
