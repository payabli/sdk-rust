pub use crate::prelude::*;

/// Create a paypoint under an existing business. Provide `businessReference` plus `doingBusinessAs`; all other fields are optional.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreatePaypointWithBusinessReference {
    #[serde(flatten)]
    pub paypoint_writable_fields_fields: PaypointWritableFields,
    /// The reference of the existing business the paypoint boards under.
    #[serde(rename = "businessReference")]
    #[serde(default)]
    pub business_reference: String,
}

impl CreatePaypointWithBusinessReference {
    pub fn builder() -> CreatePaypointWithBusinessReferenceBuilder {
        <CreatePaypointWithBusinessReferenceBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaypointWithBusinessReferenceBuilder {
    paypoint_writable_fields_fields: Option<PaypointWritableFields>,
    business_reference: Option<String>,
}

impl CreatePaypointWithBusinessReferenceBuilder {
    pub fn paypoint_writable_fields_fields(mut self, value: PaypointWritableFields) -> Self {
        self.paypoint_writable_fields_fields = Some(value);
        self
    }

    pub fn business_reference(mut self, value: impl Into<String>) -> Self {
        self.business_reference = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreatePaypointWithBusinessReference`].
    /// This method will fail if any of the following fields are not set:
    /// - [`paypoint_writable_fields_fields`](CreatePaypointWithBusinessReferenceBuilder::paypoint_writable_fields_fields)
    /// - [`business_reference`](CreatePaypointWithBusinessReferenceBuilder::business_reference)
    pub fn build(self) -> Result<CreatePaypointWithBusinessReference, BuildError> {
        Ok(CreatePaypointWithBusinessReference {
            paypoint_writable_fields_fields: self
                .paypoint_writable_fields_fields
                .ok_or_else(|| BuildError::missing_field("paypoint_writable_fields_fields"))?,
            business_reference: self
                .business_reference
                .ok_or_else(|| BuildError::missing_field("business_reference"))?,
        })
    }
}
