pub use crate::prelude::*;

/// The result of a lifecycle action, with the application's new status.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct TransitionResult {
    /// The application reference.
    #[serde(rename = "requestsReference")]
    #[serde(default)]
    pub requests_reference: String,
    /// The application's lifecycle status after the action. See the application object for the list of statuses.
    #[serde(rename = "applicationStatus")]
    #[serde(default)]
    pub application_status: String,
    /// The lifecycle actions now allowed on the application. The list can include actions that Payabli or its internal services perform, which you can't call through the API.
    #[serde(rename = "allowedActions")]
    #[serde(default)]
    pub allowed_actions: Vec<String>,
}

impl TransitionResult {
    pub fn builder() -> TransitionResultBuilder {
        <TransitionResultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct TransitionResultBuilder {
    requests_reference: Option<String>,
    application_status: Option<String>,
    allowed_actions: Option<Vec<String>>,
}

impl TransitionResultBuilder {
    pub fn requests_reference(mut self, value: impl Into<String>) -> Self {
        self.requests_reference = Some(value.into());
        self
    }

    pub fn application_status(mut self, value: impl Into<String>) -> Self {
        self.application_status = Some(value.into());
        self
    }

    pub fn allowed_actions(mut self, value: Vec<String>) -> Self {
        self.allowed_actions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`TransitionResult`].
    /// This method will fail if any of the following fields are not set:
    /// - [`requests_reference`](TransitionResultBuilder::requests_reference)
    /// - [`application_status`](TransitionResultBuilder::application_status)
    /// - [`allowed_actions`](TransitionResultBuilder::allowed_actions)
    pub fn build(self) -> Result<TransitionResult, BuildError> {
        Ok(TransitionResult {
            requests_reference: self
                .requests_reference
                .ok_or_else(|| BuildError::missing_field("requests_reference"))?,
            application_status: self
                .application_status
                .ok_or_else(|| BuildError::missing_field("application_status"))?,
            allowed_actions: self
                .allowed_actions
                .ok_or_else(|| BuildError::missing_field("allowed_actions"))?,
        })
    }
}
