pub use crate::prelude::*;

/// The vendor associated with the bill. Although you can create a vendor
/// in a create bill request, Payabli recommends creating a vendor
/// separately and passing a valid `vendorNumber` here. At minimum, the
/// `vendorNumber` is required.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct BillOutDataVendor {
    #[serde(rename = "vendorNumber")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vendor_number: Option<VendorNumber>,
}

impl BillOutDataVendor {
    pub fn builder() -> BillOutDataVendorBuilder {
        <BillOutDataVendorBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct BillOutDataVendorBuilder {
    vendor_number: Option<VendorNumber>,
}

impl BillOutDataVendorBuilder {
    pub fn vendor_number(mut self, value: VendorNumber) -> Self {
        self.vendor_number = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`BillOutDataVendor`].
    pub fn build(self) -> Result<BillOutDataVendor, BuildError> {
        Ok(BillOutDataVendor {
            vendor_number: self.vendor_number,
        })
    }
}
