pub use crate::prelude::*;

/// A single entry in a case's state history.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct StateTransitionResponse {
    /// The transition's unique identifier.
    #[serde(default)]
    pub uuid: String,
    /// The case this transition belongs to.
    #[serde(rename = "caseUuid")]
    #[serde(default)]
    pub case_uuid: String,
    #[serde(rename = "fromState")]
    pub from_state: CaseState,
    #[serde(rename = "toState")]
    pub to_state: CaseState,
    /// The IP address of the actor. Null for system transitions.
    #[serde(rename = "ipAddress")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    /// The numeric id of the user who triggered the transition. Null for system transitions.
    #[serde(rename = "triggeredBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered_by: Option<i64>,
    /// The reason recorded for the transition.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    /// When the transition occurred.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub created_at: DateTime<Utc>,
    /// The resolved user who triggered the transition. Null for system transitions.
    #[serde(rename = "triggeredByUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub triggered_by_user: Option<UserRef>,
}

impl StateTransitionResponse {
    pub fn builder() -> StateTransitionResponseBuilder {
        <StateTransitionResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct StateTransitionResponseBuilder {
    uuid: Option<String>,
    case_uuid: Option<String>,
    from_state: Option<CaseState>,
    to_state: Option<CaseState>,
    ip_address: Option<String>,
    triggered_by: Option<i64>,
    reason: Option<String>,
    created_at: Option<DateTime<Utc>>,
    triggered_by_user: Option<UserRef>,
}

impl StateTransitionResponseBuilder {
    pub fn uuid(mut self, value: impl Into<String>) -> Self {
        self.uuid = Some(value.into());
        self
    }

    pub fn case_uuid(mut self, value: impl Into<String>) -> Self {
        self.case_uuid = Some(value.into());
        self
    }

    pub fn from_state(mut self, value: CaseState) -> Self {
        self.from_state = Some(value);
        self
    }

    pub fn to_state(mut self, value: CaseState) -> Self {
        self.to_state = Some(value);
        self
    }

    pub fn ip_address(mut self, value: impl Into<String>) -> Self {
        self.ip_address = Some(value.into());
        self
    }

    pub fn triggered_by(mut self, value: i64) -> Self {
        self.triggered_by = Some(value);
        self
    }

    pub fn reason(mut self, value: impl Into<String>) -> Self {
        self.reason = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn triggered_by_user(mut self, value: UserRef) -> Self {
        self.triggered_by_user = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`StateTransitionResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`uuid`](StateTransitionResponseBuilder::uuid)
    /// - [`case_uuid`](StateTransitionResponseBuilder::case_uuid)
    /// - [`from_state`](StateTransitionResponseBuilder::from_state)
    /// - [`to_state`](StateTransitionResponseBuilder::to_state)
    /// - [`created_at`](StateTransitionResponseBuilder::created_at)
    pub fn build(self) -> Result<StateTransitionResponse, BuildError> {
        Ok(StateTransitionResponse {
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
            case_uuid: self
                .case_uuid
                .ok_or_else(|| BuildError::missing_field("case_uuid"))?,
            from_state: self
                .from_state
                .ok_or_else(|| BuildError::missing_field("from_state"))?,
            to_state: self
                .to_state
                .ok_or_else(|| BuildError::missing_field("to_state"))?,
            ip_address: self.ip_address,
            triggered_by: self.triggered_by,
            reason: self.reason,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            triggered_by_user: self.triggered_by_user,
        })
    }
}
