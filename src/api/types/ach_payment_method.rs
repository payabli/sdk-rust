pub use crate::prelude::*;

/// ACH payment method.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct AchPaymentMethod {
    /// Payment method type
    pub method: String,
    /// ID of the stored ACH payment method. Required when using a previously saved ACH method when the vendor has more than one saved method. See the [Payouts with saved ACH payment methods](/developers/developer-guides/pay-out-manage-payouts) section for more details.
    #[serde(rename = "storedMethodId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stored_method_id: Option<String>,
}

impl AchPaymentMethod {
    pub fn builder() -> AchPaymentMethodBuilder {
        <AchPaymentMethodBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct AchPaymentMethodBuilder {
    method: Option<String>,
    stored_method_id: Option<String>,
}

impl AchPaymentMethodBuilder {
    pub fn method(mut self, value: impl Into<String>) -> Self {
        self.method = Some(value.into());
        self
    }

    pub fn stored_method_id(mut self, value: impl Into<String>) -> Self {
        self.stored_method_id = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`AchPaymentMethod`].
    /// This method will fail if any of the following fields are not set:
    /// - [`method`](AchPaymentMethodBuilder::method)
    pub fn build(self) -> Result<AchPaymentMethod, BuildError> {
        Ok(AchPaymentMethod {
            method: self
                .method
                .ok_or_else(|| BuildError::missing_field("method"))?,
            stored_method_id: self.stored_method_id,
        })
    }
}
