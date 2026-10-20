pub use crate::prelude::*;

/// The enablement progress of one service the application requested.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct EnablementService {
    /// The service, such as `Card`.
    #[serde(default)]
    pub service: String,
    /// Whether the service moves money in (`MoneyIn`) or out (`MoneyOut`).
    #[serde(default)]
    pub rail: String,
    /// The service's enablement state. One of:
    ///
    /// - `Pending`: Not sent to the processor yet.
    /// - `Submitted`: Sent to the processor, waiting for a response.
    /// - `Approved`: The processor approved the service.
    /// - `Provisioned`: The service is set up and ready to use.
    /// - `HeldForInput`: Needs a correction or more information. Enablement resumes after someone acts.
    /// - `Failed`: Enablement failed for this service.
    #[serde(default)]
    pub state: String,
    /// A sentence about the service when its last attempt didn't go through, or when it failed. When the processor refused fields or reported them as missing, the sentence names them. `null` otherwise.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// The boarding API fields the processor refused or reported as missing, such as `addresses.cityLocality`. `null` when the processor named none.
    #[serde(rename = "missingFields")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_fields: Option<Vec<String>>,
}

impl EnablementService {
    pub fn builder() -> EnablementServiceBuilder {
        <EnablementServiceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct EnablementServiceBuilder {
    service: Option<String>,
    rail: Option<String>,
    state: Option<String>,
    message: Option<String>,
    missing_fields: Option<Vec<String>>,
}

impl EnablementServiceBuilder {
    pub fn service(mut self, value: impl Into<String>) -> Self {
        self.service = Some(value.into());
        self
    }

    pub fn rail(mut self, value: impl Into<String>) -> Self {
        self.rail = Some(value.into());
        self
    }

    pub fn state(mut self, value: impl Into<String>) -> Self {
        self.state = Some(value.into());
        self
    }

    pub fn message(mut self, value: impl Into<String>) -> Self {
        self.message = Some(value.into());
        self
    }

    pub fn missing_fields(mut self, value: Vec<String>) -> Self {
        self.missing_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`EnablementService`].
    /// This method will fail if any of the following fields are not set:
    /// - [`service`](EnablementServiceBuilder::service)
    /// - [`rail`](EnablementServiceBuilder::rail)
    /// - [`state`](EnablementServiceBuilder::state)
    pub fn build(self) -> Result<EnablementService, BuildError> {
        Ok(EnablementService {
            service: self
                .service
                .ok_or_else(|| BuildError::missing_field("service"))?,
            rail: self.rail.ok_or_else(|| BuildError::missing_field("rail"))?,
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
            message: self.message,
            missing_fields: self.missing_fields,
        })
    }
}
