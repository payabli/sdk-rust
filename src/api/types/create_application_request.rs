pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreateApplicationRequest {
    /// The reference of the business to board. The business must have a paypoint.
    #[serde(rename = "businessReference")]
    #[serde(default)]
    pub business_reference: String,
    /// The reference of the paypoint to board. Defaults to the business's paypoint when omitted.
    #[serde(rename = "paypointReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_reference: Option<String>,
    /// An optional identifier from your own system.
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// The type of entity being boarded.
    #[serde(rename = "entityType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_type: Option<String>,
    /// The reference of the boarding template to apply. Send one on every application. Payabli assigns the underwriting policy from the template, so a submitted application without one fails in review.
    #[serde(rename = "requestTemplate")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_template: Option<String>,
    /// Optional labels for the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    /// The sales or service representatives assigned to the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub representatives: Option<Vec<RepresentativeInput>>,
    /// Opaque application-level configuration. Null when not supplied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configurations: Option<HashMap<String, serde_json::Value>>,
    /// The services to request on the application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub services: Option<RequestedServices>,
}

impl CreateApplicationRequest {
    pub fn builder() -> CreateApplicationRequestBuilder {
        <CreateApplicationRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateApplicationRequestBuilder {
    business_reference: Option<String>,
    paypoint_reference: Option<String>,
    external_id: Option<String>,
    entity_type: Option<String>,
    request_template: Option<String>,
    tags: Option<Vec<String>>,
    representatives: Option<Vec<RepresentativeInput>>,
    configurations: Option<HashMap<String, serde_json::Value>>,
    services: Option<RequestedServices>,
}

impl CreateApplicationRequestBuilder {
    pub fn business_reference(mut self, value: impl Into<String>) -> Self {
        self.business_reference = Some(value.into());
        self
    }

    pub fn paypoint_reference(mut self, value: impl Into<String>) -> Self {
        self.paypoint_reference = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn entity_type(mut self, value: impl Into<String>) -> Self {
        self.entity_type = Some(value.into());
        self
    }

    pub fn request_template(mut self, value: impl Into<String>) -> Self {
        self.request_template = Some(value.into());
        self
    }

    pub fn tags(mut self, value: Vec<String>) -> Self {
        self.tags = Some(value);
        self
    }

    pub fn representatives(mut self, value: Vec<RepresentativeInput>) -> Self {
        self.representatives = Some(value);
        self
    }

    pub fn configurations(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.configurations = Some(value);
        self
    }

    pub fn services(mut self, value: RequestedServices) -> Self {
        self.services = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreateApplicationRequest`].
    /// This method will fail if any of the following fields are not set:
    /// - [`business_reference`](CreateApplicationRequestBuilder::business_reference)
    pub fn build(self) -> Result<CreateApplicationRequest, BuildError> {
        Ok(CreateApplicationRequest {
            business_reference: self
                .business_reference
                .ok_or_else(|| BuildError::missing_field("business_reference"))?,
            paypoint_reference: self.paypoint_reference,
            external_id: self.external_id,
            entity_type: self.entity_type,
            request_template: self.request_template,
            tags: self.tags,
            representatives: self.representatives,
            configurations: self.configurations,
            services: self.services,
        })
    }
}
