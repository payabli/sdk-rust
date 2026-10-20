pub use crate::prelude::*;

/// All fields accepted when creating a paypoint. Provide exactly one of `businessReference` or `business`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreatePaypointAllFields {
    #[serde(flatten)]
    pub paypoint_writable_fields_fields: PaypointWritableFields,
    /// The reference of an existing business the paypoint boards under.
    #[serde(rename = "businessReference")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_reference: Option<String>,
    /// The business to create inline and back the paypoint with.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business: Option<CreateBusinessRequest>,
}

impl CreatePaypointAllFields {
    pub fn builder() -> CreatePaypointAllFieldsBuilder {
        <CreatePaypointAllFieldsBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaypointAllFieldsBuilder {
    paypoint_writable_fields_fields: Option<PaypointWritableFields>,
    business_reference: Option<String>,
    business: Option<CreateBusinessRequest>,
}

impl CreatePaypointAllFieldsBuilder {
    pub fn paypoint_writable_fields_fields(mut self, value: PaypointWritableFields) -> Self {
        self.paypoint_writable_fields_fields = Some(value);
        self
    }

    pub fn business_reference(mut self, value: impl Into<String>) -> Self {
        self.business_reference = Some(value.into());
        self
    }

    pub fn business(mut self, value: CreateBusinessRequest) -> Self {
        self.business = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePaypointAllFields`].
    /// This method will fail if any of the following fields are not set:
    /// - [`paypoint_writable_fields_fields`](CreatePaypointAllFieldsBuilder::paypoint_writable_fields_fields)
    pub fn build(self) -> Result<CreatePaypointAllFields, BuildError> {
        Ok(CreatePaypointAllFields {
            paypoint_writable_fields_fields: self
                .paypoint_writable_fields_fields
                .ok_or_else(|| BuildError::missing_field("paypoint_writable_fields_fields"))?,
            business_reference: self.business_reference,
            business: self.business,
        })
    }
}
