pub use crate::prelude::*;

/// Object describing the vendor owner of payment method. Required when saving an ACH payment method on behalf of a vendor (for Pay Out transactions).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VendorDataRequest {
    /// The unique numeric ID assigned to the vendor in Payabli. Either `vendorId` or `vendorNumber` is required.
    #[serde(rename = "vendorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<i64>,
    /// Custom vendor number assigned by the business. Either `vendorId` or `vendorNumber` is required.
    #[serde(rename = "vendorNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_number: Option<String>,
}

impl VendorDataRequest {
    pub fn builder() -> VendorDataRequestBuilder {
        <VendorDataRequestBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VendorDataRequestBuilder {
    vendor_id: Option<i64>,
    vendor_number: Option<String>,
}

impl VendorDataRequestBuilder {
    pub fn vendor_id(mut self, value: i64) -> Self {
        self.vendor_id = Some(value);
        self
    }

    pub fn vendor_number(mut self, value: impl Into<String>) -> Self {
        self.vendor_number = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VendorDataRequest`].
    pub fn build(self) -> Result<VendorDataRequest, BuildError> {
        Ok(VendorDataRequest {
            vendor_id: self.vendor_id,
            vendor_number: self.vendor_number,
        })
    }
}
