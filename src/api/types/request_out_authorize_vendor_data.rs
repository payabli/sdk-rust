pub use crate::prelude::*;

/// Vendor to pay with this payout. Create the vendor first with
/// [Create vendor](/developers/api-reference/vendor/create-vendor), then
/// reference it here by `vendorNumber` or `vendorId`.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RequestOutAuthorizeVendorData {
    #[serde(rename = "vendorNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_number: Option<VendorNumber>,
    /// Payabli identifier for the vendor record. Required when `vendorNumber` isn't included.
    #[serde(rename = "vendorId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_id: Option<Vendorid>,
}

impl RequestOutAuthorizeVendorData {
    pub fn builder() -> RequestOutAuthorizeVendorDataBuilder {
        <RequestOutAuthorizeVendorDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RequestOutAuthorizeVendorDataBuilder {
    vendor_number: Option<VendorNumber>,
    vendor_id: Option<Vendorid>,
}

impl RequestOutAuthorizeVendorDataBuilder {
    pub fn vendor_number(mut self, value: VendorNumber) -> Self {
        self.vendor_number = Some(value);
        self
    }

    pub fn vendor_id(mut self, value: Vendorid) -> Self {
        self.vendor_id = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RequestOutAuthorizeVendorData`].
    pub fn build(self) -> Result<RequestOutAuthorizeVendorData, BuildError> {
        Ok(RequestOutAuthorizeVendorData {
            vendor_number: self.vendor_number,
            vendor_id: self.vendor_id,
        })
    }
}
