pub use crate::prelude::*;

/// The transition actions currently available on a case. Empty when no user action is available.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct AvailableTransitionsResponse {
    /// The available transition actions.
    #[serde(default)]
    pub transitions: Vec<CaseTrigger>,
}

impl AvailableTransitionsResponse {
    pub fn builder() -> AvailableTransitionsResponseBuilder {
        <AvailableTransitionsResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AvailableTransitionsResponseBuilder {
    transitions: Option<Vec<CaseTrigger>>,
}

impl AvailableTransitionsResponseBuilder {
    pub fn transitions(mut self, value: Vec<CaseTrigger>) -> Self {
        self.transitions = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`AvailableTransitionsResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`transitions`](AvailableTransitionsResponseBuilder::transitions)
    pub fn build(self) -> Result<AvailableTransitionsResponse, BuildError> {
        Ok(AvailableTransitionsResponse {
            transitions: self
                .transitions
                .ok_or_else(|| BuildError::missing_field("transitions"))?,
        })
    }
}
