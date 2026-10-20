pub use crate::prelude::*;

/// Fields you can set when you create a paypoint.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PaypointWritableFields {
    /// The reference of the parent paypoint, for a paypoint that nests under another.
    #[serde(rename = "parentPaypointReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_paypoint_reference: Option<String>,
    /// The paypoint type, for example `merchant`.
    #[serde(rename = "paypointType")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypoint_type: Option<String>,
    /// The name the paypoint does business as (DBA).
    #[serde(rename = "doingBusinessAs")]
    #[serde(default)]
    pub doing_business_as: String,
    /// A free-text description of the paypoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The paypoint's website URL.
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
    /// The paypoint's time zone.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    /// Your own identifier for the paypoint.
    #[serde(rename = "externalId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    /// Arbitrary key-value data you attach to the paypoint.
    #[serde(rename = "customData")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_data: Option<HashMap<String, serde_json::Value>>,
}

impl PaypointWritableFields {
    pub fn builder() -> PaypointWritableFieldsBuilder {
        <PaypointWritableFieldsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PaypointWritableFieldsBuilder {
    parent_paypoint_reference: Option<String>,
    paypoint_type: Option<String>,
    doing_business_as: Option<String>,
    description: Option<String>,
    website: Option<String>,
    customer_service_phone: Option<String>,
    customer_service_email: Option<String>,
    timezone: Option<String>,
    external_id: Option<String>,
    custom_data: Option<HashMap<String, serde_json::Value>>,
}

impl PaypointWritableFieldsBuilder {
    pub fn parent_paypoint_reference(mut self, value: impl Into<String>) -> Self {
        self.parent_paypoint_reference = Some(value.into());
        self
    }

    pub fn paypoint_type(mut self, value: impl Into<String>) -> Self {
        self.paypoint_type = Some(value.into());
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

    /// Consumes the builder and constructs a [`PaypointWritableFields`].
    /// This method will fail if any of the following fields are not set:
    /// - [`doing_business_as`](PaypointWritableFieldsBuilder::doing_business_as)
    pub fn build(self) -> Result<PaypointWritableFields, BuildError> {
        Ok(PaypointWritableFields {
            parent_paypoint_reference: self.parent_paypoint_reference,
            paypoint_type: self.paypoint_type,
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
