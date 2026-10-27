pub use crate::prelude::*;

/// Fields you can set when you create an organization. Payabli sets the organization type, so the request has no `paypointType`. Of these shared fields, only `doingBusinessAs` is required. Set `description`, `website`, `customerServicePhone`, `customerServiceEmail`, and `timezone` when you have them.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct OrganizationWritableFields {
    /// The reference of the parent organization this organization nests under. If you omit it, the organization nests under the organization your credential belongs to.
    #[serde(rename = "parentPaypointReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_paypoint_reference: Option<String>,
    /// The name the organization does business as (DBA).
    #[serde(rename = "doingBusinessAs")]
    #[serde(default)]
    pub doing_business_as: String,
    /// A free-text description of the organization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The organization's website URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub website: Option<String>,
    /// The customer service phone number.
    #[serde(rename = "customerServicePhone")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_service_phone: Option<String>,
    /// The customer service email address.
    #[serde(rename = "customerServiceEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_service_email: Option<String>,
    /// The organization's time zone, as an IANA identifier such as `America/Chicago`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Your own identifier for the organization.
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Arbitrary key-value data you attach to the organization.
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<HashMap<String, serde_json::Value>>,
}

impl OrganizationWritableFields {
    pub fn builder() -> OrganizationWritableFieldsBuilder {
        <OrganizationWritableFieldsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct OrganizationWritableFieldsBuilder {
    parent_paypoint_reference: Option<String>,
    doing_business_as: Option<String>,
    description: Option<String>,
    website: Option<String>,
    customer_service_phone: Option<String>,
    customer_service_email: Option<String>,
    timezone: Option<String>,
    external_id: Option<String>,
    custom_data: Option<HashMap<String, serde_json::Value>>,
}

impl OrganizationWritableFieldsBuilder {
    pub fn parent_paypoint_reference(mut self, value: impl Into<String>) -> Self {
        self.parent_paypoint_reference = Some(value.into());
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

    pub fn customer_service_phone(mut self, value: impl Into<String>) -> Self {
        self.customer_service_phone = Some(value.into());
        self
    }

    pub fn customer_service_email(mut self, value: impl Into<String>) -> Self {
        self.customer_service_email = Some(value.into());
        self
    }

    pub fn timezone(mut self, value: impl Into<String>) -> Self {
        self.timezone = Some(value.into());
        self
    }

    pub fn external_id(mut self, value: impl Into<String>) -> Self {
        self.external_id = Some(value.into());
        self
    }

    pub fn custom_data(mut self, value: HashMap<String, serde_json::Value>) -> Self {
        self.custom_data = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`OrganizationWritableFields`].
    /// This method will fail if any of the following fields are not set:
    /// - [`doing_business_as`](OrganizationWritableFieldsBuilder::doing_business_as)
    pub fn build(self) -> Result<OrganizationWritableFields, BuildError> {
        Ok(OrganizationWritableFields {
            parent_paypoint_reference: self.parent_paypoint_reference,
            doing_business_as: self
                .doing_business_as
                .ok_or_else(|| BuildError::missing_field("doing_business_as"))?,
            description: self.description,
            website: self.website,
            customer_service_phone: self.customer_service_phone,
            customer_service_email: self.customer_service_email,
            timezone: self.timezone,
            external_id: self.external_id,
            custom_data: self.custom_data,
        })
    }
}
