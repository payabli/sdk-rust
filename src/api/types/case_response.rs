pub use crate::prelude::*;

/// A bank-account-change case.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CaseResponse {
    /// The case's unique identifier.
    #[serde(default)]
    pub uuid: String,
    pub state: CaseState,
    #[serde(rename = "caseType")]
    pub case_type: CaseType,
    pub parameters: BankAccountChangeParameters,
    /// The organization that owns the case.
    #[serde(rename = "orgId")]
    #[serde(default)]
    pub org_id: i64,
    /// The paypoint the case applies to.
    #[serde(rename = "paypointId")]
    #[serde(default)]
    pub paypoint_id: i64,
    /// When the change is scheduled to run. Null when not scheduled.
    #[serde(rename = "scheduleFor")]
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc::option")]
    pub schedule_for: Option<DateTime<Utc>>,
    /// When the case was created.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub created_at: DateTime<Utc>,
    /// When the case was last updated.
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub updated_at: DateTime<Utc>,
    /// The numeric id of the user who created the case. `0` when created by a server-side integration.
    #[serde(rename = "createdBy")]
    #[serde(default)]
    pub created_by: i64,
    /// The numeric id of the assigned reviewer. Null when unassigned.
    #[serde(rename = "assigneeId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee_id: Option<i64>,
    /// The numeric id of the last reviewer. Null when not yet reviewed.
    #[serde(rename = "lastReviewedById")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_reviewed_by_id: Option<i64>,
    /// The ordered history of state transitions.
    #[serde(rename = "stateHistory")]
    #[serde(default)]
    pub state_history: Vec<StateTransitionResponse>,
    /// Files attached to the case.
    #[serde(default)]
    pub attachments: Vec<AttachmentResponse>,
    /// The id of the message room for the case. Null until provisioned.
    #[serde(rename = "roomId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub room_id: Option<i64>,
    /// Case metadata, including the verification outcome. Null until verification completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<CaseMetadata>,
    /// The resolved organization. Null when not enriched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub org: Option<OrgRef>,
    /// The resolved paypoint. Null when not enriched.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint: Option<PaypointRef>,
    /// The resolved creator. Null when created by a server-side integration or not enriched.
    #[serde(rename = "createdByUser")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_user: Option<UserRef>,
    /// The resolved assigned reviewer. Null when unassigned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub assignee: Option<UserRef>,
    /// The resolved last reviewer. Null when not yet reviewed.
    #[serde(rename = "lastReviewedBy")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_reviewed_by: Option<UserRef>,
}

impl CaseResponse {
    pub fn builder() -> CaseResponseBuilder {
        <CaseResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CaseResponseBuilder {
    uuid: Option<String>,
    state: Option<CaseState>,
    case_type: Option<CaseType>,
    parameters: Option<BankAccountChangeParameters>,
    org_id: Option<i64>,
    paypoint_id: Option<i64>,
    schedule_for: Option<DateTime<Utc>>,
    created_at: Option<DateTime<Utc>>,
    updated_at: Option<DateTime<Utc>>,
    created_by: Option<i64>,
    assignee_id: Option<i64>,
    last_reviewed_by_id: Option<i64>,
    state_history: Option<Vec<StateTransitionResponse>>,
    attachments: Option<Vec<AttachmentResponse>>,
    room_id: Option<i64>,
    metadata: Option<CaseMetadata>,
    org: Option<OrgRef>,
    paypoint: Option<PaypointRef>,
    created_by_user: Option<UserRef>,
    assignee: Option<UserRef>,
    last_reviewed_by: Option<UserRef>,
}

impl CaseResponseBuilder {
    pub fn uuid(mut self, value: impl Into<String>) -> Self {
        self.uuid = Some(value.into());
        self
    }

    pub fn state(mut self, value: CaseState) -> Self {
        self.state = Some(value);
        self
    }

    pub fn case_type(mut self, value: CaseType) -> Self {
        self.case_type = Some(value);
        self
    }

    pub fn parameters(mut self, value: BankAccountChangeParameters) -> Self {
        self.parameters = Some(value);
        self
    }

    pub fn org_id(mut self, value: i64) -> Self {
        self.org_id = Some(value);
        self
    }

    pub fn paypoint_id(mut self, value: i64) -> Self {
        self.paypoint_id = Some(value);
        self
    }

    pub fn schedule_for(mut self, value: DateTime<Utc>) -> Self {
        self.schedule_for = Some(value);
        self
    }

    pub fn created_at(mut self, value: DateTime<Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn updated_at(mut self, value: DateTime<Utc>) -> Self {
        self.updated_at = Some(value);
        self
    }

    pub fn created_by(mut self, value: i64) -> Self {
        self.created_by = Some(value);
        self
    }

    pub fn assignee_id(mut self, value: i64) -> Self {
        self.assignee_id = Some(value);
        self
    }

    pub fn last_reviewed_by_id(mut self, value: i64) -> Self {
        self.last_reviewed_by_id = Some(value);
        self
    }

    pub fn state_history(mut self, value: Vec<StateTransitionResponse>) -> Self {
        self.state_history = Some(value);
        self
    }

    pub fn attachments(mut self, value: Vec<AttachmentResponse>) -> Self {
        self.attachments = Some(value);
        self
    }

    pub fn room_id(mut self, value: i64) -> Self {
        self.room_id = Some(value);
        self
    }

    pub fn metadata(mut self, value: CaseMetadata) -> Self {
        self.metadata = Some(value);
        self
    }

    pub fn org(mut self, value: OrgRef) -> Self {
        self.org = Some(value);
        self
    }

    pub fn paypoint(mut self, value: PaypointRef) -> Self {
        self.paypoint = Some(value);
        self
    }

    pub fn created_by_user(mut self, value: UserRef) -> Self {
        self.created_by_user = Some(value);
        self
    }

    pub fn assignee(mut self, value: UserRef) -> Self {
        self.assignee = Some(value);
        self
    }

    pub fn last_reviewed_by(mut self, value: UserRef) -> Self {
        self.last_reviewed_by = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CaseResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`uuid`](CaseResponseBuilder::uuid)
    /// - [`state`](CaseResponseBuilder::state)
    /// - [`case_type`](CaseResponseBuilder::case_type)
    /// - [`parameters`](CaseResponseBuilder::parameters)
    /// - [`org_id`](CaseResponseBuilder::org_id)
    /// - [`paypoint_id`](CaseResponseBuilder::paypoint_id)
    /// - [`created_at`](CaseResponseBuilder::created_at)
    /// - [`updated_at`](CaseResponseBuilder::updated_at)
    /// - [`created_by`](CaseResponseBuilder::created_by)
    /// - [`state_history`](CaseResponseBuilder::state_history)
    /// - [`attachments`](CaseResponseBuilder::attachments)
    pub fn build(self) -> Result<CaseResponse, BuildError> {
        Ok(CaseResponse {
            uuid: self.uuid.ok_or_else(|| BuildError::missing_field("uuid"))?,
            state: self
                .state
                .ok_or_else(|| BuildError::missing_field("state"))?,
            case_type: self
                .case_type
                .ok_or_else(|| BuildError::missing_field("case_type"))?,
            parameters: self
                .parameters
                .ok_or_else(|| BuildError::missing_field("parameters"))?,
            org_id: self
                .org_id
                .ok_or_else(|| BuildError::missing_field("org_id"))?,
            paypoint_id: self
                .paypoint_id
                .ok_or_else(|| BuildError::missing_field("paypoint_id"))?,
            schedule_for: self.schedule_for,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            created_by: self
                .created_by
                .ok_or_else(|| BuildError::missing_field("created_by"))?,
            assignee_id: self.assignee_id,
            last_reviewed_by_id: self.last_reviewed_by_id,
            state_history: self
                .state_history
                .ok_or_else(|| BuildError::missing_field("state_history"))?,
            attachments: self
                .attachments
                .ok_or_else(|| BuildError::missing_field("attachments"))?,
            room_id: self.room_id,
            metadata: self.metadata,
            org: self.org,
            paypoint: self.paypoint,
            created_by_user: self.created_by_user,
            assignee: self.assignee,
            last_reviewed_by: self.last_reviewed_by,
        })
    }
}
