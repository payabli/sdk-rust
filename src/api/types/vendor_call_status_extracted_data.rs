pub use crate::prelude::*;

/// Data extracted from a completed outreach call.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VendorCallStatusExtractedData {
    /// Payment method the vendor said they accept. Values are `card`, `ach`, or `check`.
    #[serde(rename = "selectedPaymentMethod")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selected_payment_method: Option<String>,
    /// Contact email collected during the call.
    #[serde(rename = "contactEmail")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact_email: Option<String>,
}

impl VendorCallStatusExtractedData {
    pub fn builder() -> VendorCallStatusExtractedDataBuilder {
        <VendorCallStatusExtractedDataBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VendorCallStatusExtractedDataBuilder {
    selected_payment_method: Option<String>,
    contact_email: Option<String>,
}

impl VendorCallStatusExtractedDataBuilder {
    pub fn selected_payment_method(mut self, value: impl Into<String>) -> Self {
        self.selected_payment_method = Some(value.into());
        self
    }

    pub fn contact_email(mut self, value: impl Into<String>) -> Self {
        self.contact_email = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VendorCallStatusExtractedData`].
    pub fn build(self) -> Result<VendorCallStatusExtractedData, BuildError> {
        Ok(VendorCallStatusExtractedData {
            selected_payment_method: self.selected_payment_method,
            contact_email: self.contact_email,
        })
    }
}
