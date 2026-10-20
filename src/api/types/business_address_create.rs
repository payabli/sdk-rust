pub use crate::prelude::*;

/// An address sent when creating a business. `country` is required.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BusinessAddressCreate {
    #[serde(flatten)]
    pub business_address_fields: BusinessAddress,
}

impl BusinessAddressCreate {
    pub fn builder() -> BusinessAddressCreateBuilder {
        <BusinessAddressCreateBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BusinessAddressCreateBuilder {
    business_address_fields: Option<BusinessAddress>,
}

impl BusinessAddressCreateBuilder {
    pub fn business_address_fields(mut self, value: BusinessAddress) -> Self {
        self.business_address_fields = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BusinessAddressCreate`].
    /// This method will fail if any of the following fields are not set:
    /// - [`business_address_fields`](BusinessAddressCreateBuilder::business_address_fields)
    pub fn build(self) -> Result<BusinessAddressCreate, BuildError> {
        Ok(BusinessAddressCreate {
            business_address_fields: self
                .business_address_fields
                .ok_or_else(|| BuildError::missing_field("business_address_fields"))?,
        })
    }
}
