pub use crate::prelude::*;

/// A vendor's stored payment method.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VendorStoredMethod {
    #[serde(flatten)]
    pub vendor_response_stored_method_fields: VendorResponseStoredMethod,
    /// When `true`, this payment method is the vendor's default.
    #[serde(rename = "IsDefault")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,
}

impl VendorStoredMethod {
    pub fn builder() -> VendorStoredMethodBuilder {
        <VendorStoredMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VendorStoredMethodBuilder {
    vendor_response_stored_method_fields: Option<VendorResponseStoredMethod>,
    is_default: Option<bool>,
}

impl VendorStoredMethodBuilder {
    pub fn vendor_response_stored_method_fields(
        mut self,
        value: VendorResponseStoredMethod,
    ) -> Self {
        self.vendor_response_stored_method_fields = Some(value);
        self
    }

    pub fn is_default(mut self, value: bool) -> Self {
        self.is_default = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`VendorStoredMethod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`vendor_response_stored_method_fields`](VendorStoredMethodBuilder::vendor_response_stored_method_fields)
    pub fn build(self) -> Result<VendorStoredMethod, BuildError> {
        Ok(VendorStoredMethod {
            vendor_response_stored_method_fields: self
                .vendor_response_stored_method_fields
                .ok_or_else(|| BuildError::missing_field("vendor_response_stored_method_fields"))?,
            is_default: self.is_default,
        })
    }
}
