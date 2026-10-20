pub use crate::prelude::*;

/// The reference and initial status of a newly created paypoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaypointCreatedResponse {
    /// The paypoint reference, used to read and manage the paypoint.
    #[serde(rename = "paypointReference")]
    #[serde(default)]
    pub paypoint_reference: String,
    /// The reference of the business the paypoint belongs to.
    #[serde(rename = "businessReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_reference: Option<String>,
    /// The name the paypoint does business as (DBA).
    #[serde(rename = "doingBusinessAs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doing_business_as: Option<String>,
    /// The paypoint status. New paypoints start in `pending`.
    #[serde(rename = "paypointStatus")]
    #[serde(default)]
    pub paypoint_status: String,
    /// The paypoint's time zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// When the paypoint was created.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    #[serde(with = "crate::core::flexible_datetime::utc")]
    pub created_at: DateTime<Utc>,
    /// Hypermedia links for the paypoint resource.
    #[serde(rename = "_links")]
    #[serde(default)]
    pub links: PaypointLinks,
}

impl PaypointCreatedResponse {
    pub fn builder() -> PaypointCreatedResponseBuilder {
        <PaypointCreatedResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaypointCreatedResponseBuilder {
    paypoint_reference: Option<String>,
    business_reference: Option<String>,
    doing_business_as: Option<String>,
    paypoint_status: Option<String>,
    timezone: Option<String>,
    created_at: Option<DateTime<Utc>>,
    links: Option<PaypointLinks>,
}

impl PaypointCreatedResponseBuilder {
    pub fn paypoint_reference(mut self, value: impl Into<String>) -> Self {
        self.paypoint_reference = Some(value.into());
        self
    }

    pub fn business_reference(mut self, value: impl Into<String>) -> Self {
        self.business_reference = Some(value.into());
        self
    }

    pub fn doing_business_as(mut self, value: impl Into<String>) -> Self {
        self.doing_business_as = Some(value.into());
        self
    }

    pub fn paypoint_status(mut self, value: impl Into<String>) -> Self {
        self.paypoint_status = Some(value.into());
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    pub fn created_at(mut self, value: DateTime<Utc>) -> Self {
        self.created_at = Some(value);
        self
    }

    pub fn links(mut self, value: PaypointLinks) -> Self {
        self.links = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaypointCreatedResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`paypoint_reference`](PaypointCreatedResponseBuilder::paypoint_reference)
    /// - [`paypoint_status`](PaypointCreatedResponseBuilder::paypoint_status)
    /// - [`created_at`](PaypointCreatedResponseBuilder::created_at)
    /// - [`links`](PaypointCreatedResponseBuilder::links)
    pub fn build(self) -> Result<PaypointCreatedResponse, BuildError> {
        Ok(PaypointCreatedResponse {
            paypoint_reference: self
                .paypoint_reference
                .ok_or_else(|| BuildError::missing_field("paypoint_reference"))?,
            business_reference: self.business_reference,
            doing_business_as: self.doing_business_as,
            paypoint_status: self
                .paypoint_status
                .ok_or_else(|| BuildError::missing_field("paypoint_status"))?,
            timezone: self.timezone,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            links: self
                .links
                .ok_or_else(|| BuildError::missing_field("links"))?,
        })
    }
}
