pub use crate::prelude::*;

/// Create a paypoint and its backing business together. Provide `business` plus `doingBusinessAs`; all other fields are optional.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct CreatePaypointWithInlineBusiness {
    #[serde(flatten)]
    pub paypoint_writable_fields_fields: PaypointWritableFields,
    /// The business to create inline and back the paypoint with.
    #[serde(default)]
    pub business: CreateBusinessRequest,
}

impl CreatePaypointWithInlineBusiness {
    pub fn builder() -> CreatePaypointWithInlineBusinessBuilder {
        <CreatePaypointWithInlineBusinessBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreatePaypointWithInlineBusinessBuilder {
    paypoint_writable_fields_fields: Option<PaypointWritableFields>,
    business: Option<CreateBusinessRequest>,
}

impl CreatePaypointWithInlineBusinessBuilder {
    pub fn paypoint_writable_fields_fields(mut self, value: PaypointWritableFields) -> Self {
        self.paypoint_writable_fields_fields = Some(value);
        self
    }

    pub fn business(mut self, value: CreateBusinessRequest) -> Self {
        self.business = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`CreatePaypointWithInlineBusiness`].
    /// This method will fail if any of the following fields are not set:
    /// - [`paypoint_writable_fields_fields`](CreatePaypointWithInlineBusinessBuilder::paypoint_writable_fields_fields)
    /// - [`business`](CreatePaypointWithInlineBusinessBuilder::business)
    pub fn build(self) -> Result<CreatePaypointWithInlineBusiness, BuildError> {
        Ok(CreatePaypointWithInlineBusiness {
            paypoint_writable_fields_fields: self
                .paypoint_writable_fields_fields
                .ok_or_else(|| BuildError::missing_field("paypoint_writable_fields_fields"))?,
            business: self
                .business
                .ok_or_else(|| BuildError::missing_field("business"))?,
        })
    }
}
