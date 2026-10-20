pub use crate::prelude::*;

/// Fields shared by a boarding application in single-record and list responses.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct ApplicationCommon {
    /// The application reference, used to read and manage the application.
    #[serde(rename = "requestsReference")]
    #[serde(default)]
    pub requests_reference: String,
    /// The reference of the business being boarded.
    #[serde(rename = "businessReference")]
    #[serde(default)]
    pub business_reference: String,
    /// An identifier from your own system.
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// The reference of the organization the application belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    /// The name of the organization the application belongs to.
    #[serde(rename = "parentOrgName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_org_name: Option<String>,
    /// The type of entity being boarded.
    #[serde(rename = "entityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    /// The doing-business-as name.
    #[serde(rename = "dbaName")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dba_name: Option<String>,
    /// The application's lifecycle status, as a lowercase name. For what moves an application between statuses, see [Boarding application statuses](/guides/pay-ops-boarding-application-status-reference). One of:
    ///
    /// - `draft` (1): Created and editable. The only status that accepts updates.
    /// - `submitted` (2): Submitted with the signature of record.
    /// - `verifying` (3): Automated verification is in progress.
    /// - `pendingreview` (4): Awaiting underwriting review.
    /// - `escalatedreview` (5): Escalated to a higher review tier.
    /// - `pendingdocuments` (6): Underwriting requested documents from the applicant.
    /// - `pendingresponse` (7): Underwriting asked the applicant a question.
    /// - `approved` (8): Approved and waiting for services to be enabled.
    /// - `autoapproved` (9): Approved without manual review.
    /// - `denied` (10): Denied. Terminal.
    /// - `withdrawn` (11): Withdrawn. Terminal.
    /// - `abandoned` (12): Reserved. No action moves an application to this status.
    /// - `error` (13): Processing hit an error. Payabli can recover the application from this status.
    /// - `activated` (14): The requested services are provisioned and live. Terminal. An `approved` or `autoapproved` application moves to `activated` automatically after all requested services are boarded.
    #[serde(rename = "applicationStatus")]
    #[serde(default)]
    pub application_status: String,
    /// The numeric code of the application's lifecycle status. See `applicationStatus` for the code of each status.
    #[serde(rename = "applicationStatusCode")]
    #[serde(default)]
    pub application_status_code: i64,
    /// The lifecycle actions currently allowed on the application. The list can include actions that Payabli or its internal services perform, which you can't call through the API.
    #[serde(rename = "allowedActions")]
    #[serde(default)]
    pub allowed_actions: Vec<String>,
    /// The reference of the boarding template applied to the application.
    #[serde(rename = "requestTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_template: Option<String>,
    /// The services the application requests. Left out when the application doesn't request any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<RequestedServices>,
    /// Labels for the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The representatives assigned to the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub representatives: Option<Vec<RepresentativeResponse>>,
    /// When the application was created.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    /// When the application was submitted.
    #[serde(rename = "submittedAt")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submitted_at: Option<String>,
    /// Hypermedia links for the application resource.
    #[serde(rename = "_links")]
    #[serde(default)]
    pub links: ApplicationLinks,
}

impl ApplicationCommon {
    pub fn builder() -> ApplicationCommonBuilder {
        <ApplicationCommonBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct ApplicationCommonBuilder {
    requests_reference: Option<String>,
    business_reference: Option<String>,
    external_id: Option<String>,
    parent: Option<String>,
    parent_org_name: Option<String>,
    entity_type: Option<String>,
    dba_name: Option<String>,
    application_status: Option<String>,
    application_status_code: Option<i64>,
    allowed_actions: Option<Vec<String>>,
    request_template: Option<String>,
    services: Option<RequestedServices>,
    tags: Option<Vec<String>>,
    representatives: Option<Vec<RepresentativeResponse>>,
    created_at: Option<String>,
    submitted_at: Option<String>,
    links: Option<ApplicationLinks>,
}

impl ApplicationCommonBuilder {
    pub fn requests_reference(mut self, value: impl Into<String>) -> Self {
        self.requests_reference = Some(value.into());
        self
    }

    pub fn business_reference(mut self, value: impl Into<String>) -> Self {
        self.business_reference = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn parent(mut self, value: impl Into<String>) -> Self {
        self.parent = Some(value.into());
        self
    }

    pub fn parent_org_name(mut self, value: impl Into<String>) -> Self {
        self.parent_org_name = Some(value.into());
        self
    }

    pub fn entity_type(mut self, value: impl Into<String>) -> Self {
        self.entity_type = Some(value.into());
        self
    }

    pub fn dba_name(mut self, value: impl Into<String>) -> Self {
        self.dba_name = Some(value.into());
        self
    }

    pub fn application_status(mut self, value: impl Into<String>) -> Self {
        self.application_status = Some(value.into());
        self
    }

    pub fn application_status_code(mut self, value: i64) -> Self {
        self.application_status_code = Some(value);
        self
    }

    pub fn allowed_actions(mut self, value: Vec<String>) -> Self {
        self.allowed_actions = Some(value);
        self
    }

    pub fn request_template(mut self, value: impl Into<String>) -> Self {
        self.request_template = Some(value.into());
        self
    }

    pub fn services(mut self, value: RequestedServices) -> Self {
        self.services = Some(value);
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn representatives(mut self, value: Vec<RepresentativeResponse>) -> Self {
        self.representatives = Some(value);
        self
    }

    pub fn created_at(mut self, value: impl Into<String>) -> Self {
        self.created_at = Some(value.into());
        self
    }

    pub fn submitted_at(mut self, value: impl Into<String>) -> Self {
        self.submitted_at = Some(value.into());
        self
    }

    pub fn links(mut self, value: ApplicationLinks) -> Self {
        self.links = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`ApplicationCommon`].
    /// This method will fail if any of the following fields are not set:
    /// - [`requests_reference`](ApplicationCommonBuilder::requests_reference)
    /// - [`business_reference`](ApplicationCommonBuilder::business_reference)
    /// - [`application_status`](ApplicationCommonBuilder::application_status)
    /// - [`application_status_code`](ApplicationCommonBuilder::application_status_code)
    /// - [`allowed_actions`](ApplicationCommonBuilder::allowed_actions)
    /// - [`created_at`](ApplicationCommonBuilder::created_at)
    /// - [`links`](ApplicationCommonBuilder::links)
    pub fn build(self) -> Result<ApplicationCommon, BuildError> {
        Ok(ApplicationCommon {
            requests_reference: self
                .requests_reference
                .ok_or_else(|| BuildError::missing_field("requests_reference"))?,
            business_reference: self
                .business_reference
                .ok_or_else(|| BuildError::missing_field("business_reference"))?,
            external_id: self.external_id,
            parent: self.parent,
            parent_org_name: self.parent_org_name,
            entity_type: self.entity_type,
            dba_name: self.dba_name,
            application_status: self
                .application_status
                .ok_or_else(|| BuildError::missing_field("application_status"))?,
            application_status_code: self
                .application_status_code
                .ok_or_else(|| BuildError::missing_field("application_status_code"))?,
            allowed_actions: self
                .allowed_actions
                .ok_or_else(|| BuildError::missing_field("allowed_actions"))?,
            request_template: self.request_template,
            services: self.services,
            tags: self.tags,
            representatives: self.representatives,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            submitted_at: self.submitted_at,
            links: self
                .links
                .ok_or_else(|| BuildError::missing_field("links"))?,
        })
    }
}
