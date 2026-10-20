pub use crate::prelude::*;

/// A paypoint as it appears in a list response.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PaypointSummaryResponse {
    /// The paypoint reference.
    #[serde(rename = "paypointReference")]
    #[serde(default)]
    pub paypoint_reference: String,
    /// The reference of the business the paypoint belongs to.
    #[serde(rename = "businessReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_reference: Option<String>,
    /// The reference of the parent paypoint this paypoint nests under. For a merchant boarded directly under an organization, this is the organization, which is itself a paypoint.
    #[serde(rename = "parentPaypointReference")]
    #[serde(default)]
    pub parent_paypoint_reference: String,
    /// The paypoint type, for example `merchant`.
    #[serde(rename = "paypointType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_type: Option<String>,
    /// The paypoint's current status.
    #[serde(rename = "paypointStatus")]
    #[serde(default)]
    pub paypoint_status: String,
    /// The name the paypoint does business as (DBA).
    #[serde(rename = "doingBusinessAs")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doing_business_as: Option<String>,
    /// A free-text description of the paypoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The paypoint's website URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    /// The paypoint's time zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// The customer service phone number.
    #[serde(rename = "customerServicePhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_service_phone: Option<String>,
    /// The customer service email address.
    #[serde(rename = "customerServiceEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_service_email: Option<String>,
    /// The paypoint's entrypoint name, used as the `entry` value on endpoints that target the paypoint.
    #[serde(rename = "entryName")]
    #[serde(default)]
    pub entry_name: String,
    /// When the paypoint was created.
    #[serde(rename = "createdAt")]
    #[serde(default)]
    pub created_at: String,
    /// When the paypoint was last updated.
    #[serde(rename = "updatedAt")]
    #[serde(default)]
    pub updated_at: String,
    /// Hypermedia links for the paypoint resource.
    #[serde(rename = "_links")]
    #[serde(default)]
    pub links: PaypointLinks,
}

impl PaypointSummaryResponse {
    pub fn builder() -> PaypointSummaryResponseBuilder {
        <PaypointSummaryResponseBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaypointSummaryResponseBuilder {
    paypoint_reference: Option<String>,
    business_reference: Option<String>,
    parent_paypoint_reference: Option<String>,
    paypoint_type: Option<String>,
    paypoint_status: Option<String>,
    doing_business_as: Option<String>,
    description: Option<String>,
    website: Option<String>,
    timezone: Option<String>,
    customer_service_phone: Option<String>,
    customer_service_email: Option<String>,
    entry_name: Option<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
    links: Option<PaypointLinks>,
}

impl PaypointSummaryResponseBuilder {
    pub fn paypoint_reference(mut self, value: impl Into<String>) -> Self {
        self.paypoint_reference = Some(value.into());
        self
    }

    pub fn business_reference(mut self, value: impl Into<String>) -> Self {
        self.business_reference = Some(value.into());
        self
    }

    pub fn parent_paypoint_reference(mut self, value: impl Into<String>) -> Self {
        self.parent_paypoint_reference = Some(value.into());
        self
    }

    pub fn paypoint_type(mut self, value: impl Into<String>) -> Self {
        self.paypoint_type = Some(value.into());
        self
    }

    pub fn paypoint_status(mut self, value: impl Into<String>) -> Self {
        self.paypoint_status = Some(value.into());
        self
    }

    pub fn doing_business_as(mut self, value: impl Into<String>) -> Self {
        self.doing_business_as = Some(value.into());
        self
    }

    pub fn description(mut self, value: impl Into<String>) -> Self {
        self.description = Some(value.into());
        self
    }

    pub fn website(mut self, value: impl Into<String>) -> Self {
        self.website = Some(value.into());
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    pub fn customer_service_phone(mut self, value: impl Into<String>) -> Self {
        self.customer_service_phone = Some(value.into());
        self
    }

    pub fn customer_service_email(mut self, value: impl Into<String>) -> Self {
        self.customer_service_email = Some(value.into());
        self
    }

    pub fn entry_name(mut self, value: impl Into<String>) -> Self {
        self.entry_name = Some(value.into());
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

    pub fn links(mut self, value: PaypointLinks) -> Self {
        self.links = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PaypointSummaryResponse`].
    /// This method will fail if any of the following fields are not set:
    /// - [`paypoint_reference`](PaypointSummaryResponseBuilder::paypoint_reference)
    /// - [`parent_paypoint_reference`](PaypointSummaryResponseBuilder::parent_paypoint_reference)
    /// - [`paypoint_status`](PaypointSummaryResponseBuilder::paypoint_status)
    /// - [`entry_name`](PaypointSummaryResponseBuilder::entry_name)
    /// - [`created_at`](PaypointSummaryResponseBuilder::created_at)
    /// - [`updated_at`](PaypointSummaryResponseBuilder::updated_at)
    /// - [`links`](PaypointSummaryResponseBuilder::links)
    pub fn build(self) -> Result<PaypointSummaryResponse, BuildError> {
        Ok(PaypointSummaryResponse {
            paypoint_reference: self
                .paypoint_reference
                .ok_or_else(|| BuildError::missing_field("paypoint_reference"))?,
            business_reference: self.business_reference,
            parent_paypoint_reference: self
                .parent_paypoint_reference
                .ok_or_else(|| BuildError::missing_field("parent_paypoint_reference"))?,
            paypoint_type: self.paypoint_type,
            paypoint_status: self
                .paypoint_status
                .ok_or_else(|| BuildError::missing_field("paypoint_status"))?,
            doing_business_as: self.doing_business_as,
            description: self.description,
            website: self.website,
            timezone: self.timezone,
            customer_service_phone: self.customer_service_phone,
            customer_service_email: self.customer_service_email,
            entry_name: self
                .entry_name
                .ok_or_else(|| BuildError::missing_field("entry_name"))?,
            created_at: self
                .created_at
                .ok_or_else(|| BuildError::missing_field("created_at"))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| BuildError::missing_field("updated_at"))?,
            links: self
                .links
                .ok_or_else(|| BuildError::missing_field("links"))?,
        })
    }
}
