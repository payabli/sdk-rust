pub use crate::prelude::*;

/// A boarding application (request), as returned when you create, get, or update one.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct Application {
    #[serde(flatten)]
    pub application_common_fields: ApplicationCommon,
    /// The reference of the paypoint the application boards.
    #[serde(rename = "paypointReference")]
    #[serde(default)]
    pub paypoint_reference: String,
    /// The status of the paypoint the application boards. New paypoints start in `pending`.
    #[serde(rename = "paypointStatus")]
    #[serde(default)]
    pub paypoint_status: String,
}

impl Application {
    pub fn builder() -> ApplicationBuilder {
        <ApplicationBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplicationBuilder {
    application_common_fields: Option<ApplicationCommon>,
    paypoint_reference: Option<String>,
    paypoint_status: Option<String>,
}

impl ApplicationBuilder {
    pub fn application_common_fields(mut self, value: ApplicationCommon) -> Self {
        self.application_common_fields = Some(value);
        self
    }

    pub fn paypoint_reference(mut self, value: impl Into<String>) -> Self {
        self.paypoint_reference = Some(value.into());
        self
    }

    pub fn paypoint_status(mut self, value: impl Into<String>) -> Self {
        self.paypoint_status = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`Application`].
    /// This method will fail if any of the following fields are not set:
    /// - [`application_common_fields`](ApplicationBuilder::application_common_fields)
    /// - [`paypoint_reference`](ApplicationBuilder::paypoint_reference)
    /// - [`paypoint_status`](ApplicationBuilder::paypoint_status)
    pub fn build(self) -> Result<Application, BuildError> {
        Ok(Application {
            application_common_fields: self
                .application_common_fields
                .ok_or_else(|| BuildError::missing_field("application_common_fields"))?,
            paypoint_reference: self
                .paypoint_reference
                .ok_or_else(|| BuildError::missing_field("paypoint_reference"))?,
            paypoint_status: self
                .paypoint_status
                .ok_or_else(|| BuildError::missing_field("paypoint_status"))?,
        })
    }
}
