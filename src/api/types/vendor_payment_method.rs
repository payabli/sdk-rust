pub use crate::prelude::*;

/// Payment method object to use for the payout.
/// - `{ method: "managed" }` - Managed payment method
/// - `{ method: "vcard" }` - Virtual card payment method
/// - `{ method: "check" }` - Check payment method
/// - `{ method: "ach", storedMethodId?: "..." }` - ACH payment method with optional stored method ID
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct VendorPaymentMethod {
    /// Payment method type - "managed", "vcard", "check", or "ach"
    #[serde(default)]
    pub method: String,
    /// ID of the stored ACH payment method. Only applicable when method is "ach". Required when using a previously saved ACH method when the vendor has more than one saved method. See the [Payouts with saved ACH payment methods](/developers/developer-guides/pay-out-manage-payouts) section for more details.
    #[serde(rename = "storedMethodId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_method_id: Option<String>,
}

impl VendorPaymentMethod {
    pub fn builder() -> VendorPaymentMethodBuilder {
        <VendorPaymentMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct VendorPaymentMethodBuilder {
    method: Option<String>,
    stored_method_id: Option<String>,
}

impl VendorPaymentMethodBuilder {
    pub fn method(mut self, value: impl Into<String>) -> Self {
        self.method = Some(value.into());
        self
    }

    pub fn stored_method_id(mut self, value: impl Into<String>) -> Self {
        self.stored_method_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`VendorPaymentMethod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`method`](VendorPaymentMethodBuilder::method)
    pub fn build(self) -> Result<VendorPaymentMethod, BuildError> {
        Ok(VendorPaymentMethod {
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
            stored_method_id: self.stored_method_id,
        })
    }
}
